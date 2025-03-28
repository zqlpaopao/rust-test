#![allow(unused)]
use bb8::Pool;
use bb8_nebula::{
    graph::GraphClientConfiguration, impl_tokio::v3::graph::new_graph_connection_manager,
    GraphConnectionManager,
};
use fbthrift_transport::impl_tokio::{TokioSleep, TokioTcpStream};
use fbthrift_transport::AsyncTransportConfiguration;
use nebula_client::v3::{GraphQuery as _, GraphTransportResponseHandler};
use nebula_client::VersionV3;
use std::time::Duration;

pub async fn test_nebula() {
    //链接池创建
    let pool = make_cli().await;

    //获取边
    // get_edge(&pool).await;

    //操作
    edit(&pool).await;
}

/**************************************** insert update  ******************************************/
async fn edit(
    pool: &Pool<
        GraphConnectionManager<
            TokioTcpStream,
            TokioSleep,
            GraphTransportResponseHandler,
            VersionV3,
        >,
    >,
) {
    let mut session = pool
        .get()
        .await
        .map_err(|err| format!("Unable to connect to database: {}", err))
        .unwrap();

    //insert
    // let res = session
    //     .execute_json(b"INSERT VERTEX device(sn,unit_type,mode,status,c_time,u_time,d_time) VALUES \"10.198.12.128\":(\"2102351RFDDML7001149\",2,\"mode\",1,\"2023-10-31 16:30:08\",\"2023-10-31 16:30:08\",\"0000-00-00 00:00:00\")"
    //         .to_vec().as_ref())
    //     .await
    //     .unwrap();
    /*
       GraphQueryOutput { latency: 2.244ms, space_name: Some([116, 111, 112, 111, 108, 111, 103, 121]), data_set: [Space { name: "basketballplayer" }, Space { name: "plan_topology" }, Space { name: "real_topology" }, Space { name: "subgraph" }, Space { name: "topology" }] }
       转换成功: {"errors":[{"code":0}],"results":[{"spaceName":"topology","errors":{"code":0},"latencyInUs":17951}]}

    */

    let res = session
        .execute_json(b"UPDATE VERTEX ON port \"10.16.128.37__100GE1/0/1\" SET name=\"100GE1/0/7\",dev_ip=\"192.168.0.36\",admin_status=1,operation_status=2,speed=10000,mode=\"system\",status=1,u_time=\"2024-01-24 20:10:23\""
            .to_vec().as_ref())
        .await
        .unwrap();
    /*
       GraphQueryOutput { latency: 2.29ms, space_name: Some([116, 111, 112, 111, 108, 111, 103, 121]), data_set: [Space { name: "basketballplayer" }, Space { name: "plan_topology" }, Space { name: "real_topology" }, Space { name: "subgraph" }, Space { name: "topology" }] }
       转换成功: {"errors":[{"code":0}],"results":[{"spaceName":"topology","errors":{"code":0},"latencyInUs":13743}]}

       GraphQueryOutput { latency: 1.509ms, space_name: Some([116, 111, 112, 111, 108, 111, 103, 121]), data_set: [Space { name: "basketballplayer" }, Space { name: "plan_topology" }, Space { name: "real_topology" }, Space { name: "subgraph" }, Space { name: "topology" }] }
       转换成功: {"errors":[{"message":"Storage Error: Vertex or edge not found.","code":-1005}],"results":[{"spaceName":"topology","errors":ssage":"Storage Error: Vertex or edge not found.","code":-1005},"latencyInUs":4577}]}

    */

    match String::from_utf8(res) {
        Ok(string) => {
            println!("转换成功: {}", string)
        }
        Err(e) => println!("转换失败: {}", e),
    }
}

/**************************************** 获取  ******************************************/

async fn get_edge(
    pool: &Pool<
        GraphConnectionManager<
            TokioTcpStream,
            TokioSleep,
            GraphTransportResponseHandler,
            VersionV3,
        >,
    >,
) {
    let mut session = pool
        .get()
        .await
        .map_err(|err| format!("Unable to connect to database: {}", err))
        .unwrap();

    //match
    // let res = session
    //     .execute_json(
    //         b"match(v:device)-[e:edge_device]->(v1:device) return v,e,v1 limit 1 "
    //             .to_vec()
    //             .as_ref(),
    //     )
    //     .await
    //     .unwrap();
    /*
       GraphQueryOutput { latency: 2.342ms, space_name: Some([116, 111, 112, 111, 108, 111, 103, 121]), data_set: [Space { name: "basketballplayer" }, Space { name: "plan_topology" }, Space { name: "real_topology" }, Space { name: "subgraph" }, Space { name: "topology" }] }
       转换成功: {"errors":[{"code":0}],"results":[{"spaceName":"topology","data":[{"meta":[{"type":"vertex","id":"10.226.130.70"},{"type":"e,"id":{"ranking":4911,"type":182,"dst":"10.226.129.5","src":"10.226.130.70","name":"edge_device"}},{"type":"vertex","id":"10.226.129.5"}],"row":[{"device.d_time":"0000-00-00 00:00:00","device.c_time":"2024-04-08 20:52:47","device.status":1,"device.datacenter_name":"宿湖滨新区T4","device.isp":"","device.unit_type":1,"device.logic_name":"宿迁_京东_湖滨新区公有云机房","device.isp_type":"","device.groupe.u_time":"2024-04-09 10:09:46","device.mode":"system","device.sn":"210235A2BBH181000970","device.pod_uuid":"0d51a77e-3091-4176-a405-5865734a6768","device.pod":"POD001","device.region":"华东","device.building_name":"宿迁湖滨新区T4","device.role":"T0","device.business"{"status":1,"cn_name":"","mode":"system","dst_port":"FortyGigE1/1/7","dst_logic":"宿迁_京东_湖滨新区公有云机房","src_logic":"宿迁_京东湖滨新区公有云机房","type":3,"c_time":"2024-04-09 10:42:07","u_time":"2024-04-09 10:42:07","src_port":"FortyGigE1/0/49","speed":0,"namtime":"0000-00-00 00:00:00"},{"device.group_name":"-","device.isp_type":"","device.logic_name":"宿迁_京东_湖滨新区公有云机房","device.me":"宿迁湖滨新区T4","device.status":1,"device.d_time":"0000-00-00 00:00:00","device.unit_type":1,"device.c_time":"2024-04-08 20:52:47ice.isp":"","device.business":"公有云","device.role":"T1","device.building_name":"宿迁湖滨新区T4","device.pod_uuid":"8611b0ba-651b-45fcc0f65b742","device.region":"华东","device.u_time":"2024-04-09 10:09:45","device.sn":"210235A1SSH184000158","device.mode":"system","dece.pod":"POD003"}]}],"columns":["v","e","v1"],"errors":{"code":0},"latencyInUs":770596}]}

    */

    //lookup
    // let res = session
    //     .execute_json(
    //         b"lookup on device yield id(vertex),properties(vertex) as c |limit 1 "
    //             .to_vec()
    //             .as_ref(),
    //     )
    //     .await
    //     .unwrap();
    /*
       GraphQueryOutput { latency: 4.018ms, space_name: Some([116, 111, 112, 111, 108, 111, 103, 121]), data_set: [Space { name: "basketballplayer" }, Space { name: "plan_topology" }, Space { name: "real_topology" }, Space { name: "subgraph" }, Space { name: "topology" }] }
       转换成功: {"errors":[{"code":0}],"results":[{"spaceName":"topology","data":[{"meta":[null,[null,null,null,null,null,null,null,null,nulll,null,null,null,null,null,null,null,null]],"row":["10.208.64.158",{"isp":"","group_name":"","business":"","sn":"2102350GTW6TJ6000612","pod":"POD000","d_time":"0000-00-00 00:00:00","datacenter_name":"北京互联港湾","status":1,"mode":"system","logic_name":"","isp_type"uilding_name":"","role":"T0","pod_uuid":"","unit_type":1,"c_time":"2024-04-08 20:52:47","region":"","u_time":"2024-04-08 20:52:47"}]}],"columns":["id(VERTEX)","c"],"errors":{"code":0},"latencyInUs":2511}]}

    */

    // execute
    // let res =  session
    // // .signout() //释放当前session
    // .execute(b"lookup on device yield id(vertex),properties(vertex) as c |limit 1 "
    //     .to_vec()
    //     .as_ref())
    // .await
    // .unwrap();
    // println!("{:?}", res);
    // execute  返回的是字节 但是时间快3.4ms左右
    //GraphQueryOutput { latency: 1.482ms, space_name: Some([116, 111, 112, 111, 108, 111, 103, 121]), data_set: [Space { name: "basketballplayer" }, Space { name: "plan_topology" }, Space { name: "real_topology" }, Space { name: "subgraph" }, Space { name: "topology" }] }
    // ExecutionResponse { error_code: ErrorCode::SUCCEEDED, latency_in_us: 4843, data: Some(DataSet { column_names: [[105, 100, 40, 86, 69, 82, 84, 69, 88, 41], [99]], rows: [Row { values: [sVal([49, 48, 46, 50, 48, 56, 46, 54, 52, 46, 49, 53, 56]), mVal(NMap { kvs: {[98, 117, 105, 108, 100, 105, 110, 103, 95, 110, 97, 109, 101]: sVal([]), [98, 117, 115, 105, 110, 101, 115, 115]: sVal([]), [99, 95, 116, 105, 109, 101]: sVal([50, 48, 50, 52, 45, 48, 52, 45, 48, 56, 32, 50, 48, 58, 53, 50, 58, 52, 55]), [100, 95, 116, 105, 109, 101]: sVal([48, 48, 48, 48, 45, 48, 48, 45, 48, 48, 32, 48, 48, 58, 48, 48, 58, 48, 48]), [100, 97, 116, 97, 99, 101, 110, 116, 101, 114, 95, 110, 97, 109, 101]: sVal([229, 140, 151, 228, 186, 172, 228, 186, 146, 232, 129, 148, 230, 184, 175, 230, 185, 190]), [103, 114, 111, 117, 112, 95, 110, 97, 109, 101]: sVal([]), [105, 115, 112]: sVal([]), [105, 115, 112, 95, 116, 121, 112, 101]: sVal([]), [108, 111, 103, 105, 99, 95, 110, 97, 109, 101]: sVal([]), [109, 111, 100, 101]: sVal([115, 121, 115, 116, 101, 109]), [112, 111, 100]: sVal([80, 79, 68, 48, 48, 48]), [112, 111, 100, 95, 117, 117, 105, 100]: sVal([]), [114, 101, 103, 105, 111, 110]: sVal([]), [114, 111, 108, 101]: sVal([84, 48]), [115, 110]: sVal([50, 49, 48, 50, 51, 53, 48, 71, 84, 87, 54, 84, 74, 54, 48, 48, 48, 54, 49, 50]), [115, 116, 97, 116, 117, 115]: iVal(1), [117, 95, 116, 105, 109, 101]: sVal([50, 48, 50, 52, 45, 48, 52, 45, 48, 56, 32, 50, 48, 58, 53, 50, 58, 52, 55]), [117, 110, 105, 116, 95, 116, 121, 112, 101]: iVal(1)} })] }] }), space_name: Some([116, 111, 112, 111, 108, 111, 103, 121]), error_msg: None, plan_desc: None, comment: None }

    // query 查询的是space_name
    // let res =  session
    //     // .signout() //释放当前session
    //     .query(b"lookup on device yield id(vertex),properties(vertex) as c |limit 1 "
    //         .to_vec()
    //         .as_ref())
    //     .await
    //     .unwrap();
    // println!("{:?}", res);
    /*
       GraphQueryOutput { latency: 1.284ms, space_name: Some([116, 111, 112, 111, 108, 111, 103, 121]), data_set: [Space { name: "basketballplayer" }, Space { name: "plan_topology" }, Space { name: "real_topology" }, Space { name: "subgraph" }, Space { name: "topology" }] }
       GraphQueryOutput { latency: 5.685ms, space_name: Some([116, 111, 112, 111, 108, 111, 103, 121]), data_set: [()] }
    */

    let res = session.execute_json(b"lookup on port  where port.dev_ip == '10.16.128.1' yield id(vertex) as Vid | go from $-.Vid over edge_port yield properties($$)as d".to_vec().as_ref()).await.unwrap();
    // 尝试将 Vec<u8> 转换为 String
    match String::from_utf8(res) {
        Ok(string) => {
            println!("转换成功: {}", string)
        }
        Err(e) => println!("转换失败: {}", e),
    }
}

/**************************************** rpc  ******************************************/
// 创建链接池
async fn make_cli() -> Pool<
    GraphConnectionManager<TokioTcpStream, TokioSleep, GraphTransportResponseHandler, VersionV3>,
> {
    let client_configuration = GraphClientConfiguration::new(
        "127.0.0.1".to_string(),
        9669,
        "root".to_string(),
        "nebula".to_string(),
        Some("topology".to_string()),
    );

    let mut transport_configuration =
        AsyncTransportConfiguration::new(GraphTransportResponseHandler);

    let buf_size = transport_configuration.get_buf_size();
    println!("get_buf_size:{buf_size}");
    //get_buf_size:1024

    let buf_size = transport_configuration.get_max_buf_size();
    println!("get_max_buf_size:{buf_size}");
    //get_max_buf_size:4096

    let buf_size = transport_configuration.get_max_parse_response_bytes_count();
    println!("get_max_parse_response_bytes_count:{buf_size}");
    //get_max_parse_response_bytes_count:3

    transport_configuration.set_max_buf_size(1024 * 1024 * 4);
    transport_configuration.set_buf_size(1024 * 1024 * 2);
    transport_configuration.set_read_timeout(Duration::from_millis(3000).as_secs() as u32);
    transport_configuration.set_max_parse_response_bytes_count(64);

    let buf_size = transport_configuration.get_max_buf_size();
    println!("get_max_buf_size:{buf_size}");
    //get_max_buf_size:4194304

    let manager = new_graph_connection_manager(client_configuration, transport_configuration);
    let pool = bb8::Pool::builder()
        .max_size(1)
        .build(manager)
        .await
        .unwrap();

    //测试是否成功创建链接池
    //这块报错 是因为 github的和create的不一致 github的将http port注释了 create没有
    // {
    //     let mut session = pool
    //         .get()
    //         .await
    //         .map_err(|err| format!(" pool.get():{err}"))
    //         .unwrap();
    //     let res = session
    //         .show_hosts()
    //         .await
    //         .map_err(|err| format!("session.show_hosts():{err}"))
    //         .unwrap();
    //     println!("{res:?}");
    // }

    //
    {
        let mut session = pool
            .get()
            .await
            .map_err(|err| format!(" pool.get():{err}"))
            .unwrap();

        let res = session
            .show_spaces()
            .await
            .map_err(|err| format!("session.show_spaces():{err}"))
            .unwrap();
        println!("{res:?}");
    }

    pool
}

trait MyAsyncTrait {
    async fn perform_async_task(&self) -> Result<(), Box<dyn std::error::Error>>;
}

struct MyStruct;

impl MyAsyncTrait for MyStruct {
    async fn perform_async_task(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 异步任务的实现
        Ok(())
    }
}
