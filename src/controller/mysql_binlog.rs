
#![allow(unused)]
use mysql_async::{  Conn, Opts, OptsBuilder, Pool, Row};
use mysql_binlog_connector_rust::binlog_client::BinlogClient;
use mysql_async::prelude::{Query};
use mysql_async::Result as MySqlResult;
use mysql_binlog_connector_rust::column::column_value::ColumnValue;
use mysql_binlog_connector_rust::column::json::json_binary::JsonBinary;
use mysql_binlog_connector_rust::event::event_data::EventData;
use mysql_binlog_connector_rust::event::row_event::RowEvent;

pub async fn mysql_binlog() -> Result<(), Box<dyn std::error::Error>> {

    let pool = Pool::new(get_opt());
    // 配置MySQL连接
    let ( gt_id,gt_id_set,filename, pos) = create_binlog_stream_conn(pool).await.unwrap();
    let url = "mysql://root:meimima123@127.0.0.1:3306/test".to_string();
    let server_id: u64 = 200;
    let binlog_filename = String::from_utf8(filename).unwrap();
    let binlog_position: u32 =pos as u32;
    let mut client = BinlogClient {
        url,
        binlog_filename,
        binlog_position,
        server_id,
        gtid_enabled:gt_id,
        gtid_set:gt_id_set,
        heartbeat_interval_secs: 300,
        timeout_secs: 60,
    };
    let mut stream = client.connect().await?;

    loop {
        let (header, data) = stream.read().await.unwrap();

        println!("header: {:?}", header);
        println!("data: {:?}", data);
        parse_json_columns(data);
        println!();
    }
}

fn parse_json_columns(data: EventData) {
    let parse_row = |row: RowEvent| {
        println!("----.update rows: {:?}", row);

        for column_value in row.column_values {
            println!("----dd.update rows: {:?}", column_value);

            if let ColumnValue::Json(bytes) = column_value {
                println!(
                    "json column: {}",
                    JsonBinary::parse_as_string(&bytes).unwrap()
                )
            }
        }
    };

    match data {
        EventData::WriteRows(event) => {
            for row in event.rows {
                parse_row(row)
            }
        }
        EventData::DeleteRows(event) => {
            for row in event.rows {
                parse_row(row)
            }
        }
        EventData::UpdateRows(event) => {
            println!("update rows: {:?}", event.rows);
            for (before, after) in event.rows {
                parse_row(before);
                parse_row(after);
            }
        }
        _ => {}
    }
}

pub fn get_opt() -> OptsBuilder {
    let  builder = OptsBuilder::from_opts(Opts::from_url("mysql://root:meimima123@127.0.0.1:3306/test").unwrap());
    // if test_ssl() {
    //     let ssl_opts = SslOpts::default()
    //         .with_danger_skip_domain_validation(true)
    //         .with_danger_accept_invalid_certs(true);
    //     builder = builder.prefer_socket(false).ssl_opts(ssl_opts);
    // }
    // if test_compression() {
    //     builder = builder.compression(Compression::default());
    // }
    builder
}


async fn create_binlog_stream_conn(pool: Pool) -> MySqlResult<(  bool,String,Vec<u8>, u64)> {
    let mut conn = pool.get_conn().await?;
    if conn.server_version() >= (8, 0, 31) && conn.server_version() < (9, 0, 0) {
        let _ = "SET binlog_transaction_compression=ON"
            .ignore(&mut conn)
            .await;
    }
    let mut gt_id = false;
    let mut gt_id_set = String::new();
    if let Ok(Some(gtid_mode)) = "SELECT @@GLOBAL.GTID_MODE"
        .first::<String, _>(&mut conn)
        .await
    {
        if gtid_mode.starts_with("ON") {
            gt_id = true;
            let row: Option<Row> = "SHOW MASTER STATUS".first(&mut conn).await?;
            if let Some(row) = row {
                gt_id_set  = row.get("Executed_Gtid_Set").unwrap_or_default();

            }
        }
    }

    let row: Vec<Row> = "SHOW BINARY LOGS".fetch(&mut conn).await?;
    let mut filename = Vec::new();
    let mut position = 0;
    if let Some(row) = row.last() {
        filename = row.get("Log_name").unwrap_or_default();
        position = row.get("File_size").unwrap_or(0);
    }
    Ok(( gt_id,gt_id_set,filename, position))
}
