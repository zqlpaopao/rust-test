use std::collections::HashMap;
use fast_paths::{InputGraph};
use log::debug;
use rbatis::{crud, RBatis};
use rbdc_mysql::MysqlDriver;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Xxx {
    pub local_dev_id: String,    // VARCHAR(45)
    pub local_ip: String,        // VARCHAR(45)
    pub local_if_name: String,   // VARCHAR(500)
    pub local_role: String,      // VARCHAR(45)
    pub remote_ip: String,       // VARCHAR(45)
    pub remote_if_name: String,  // VARCHAR(500)
    pub remote_role: String,     // VARCHAR(45)

}
crud!(Xxx {}); // impl_insert!($table {}) + impl_select!($table {}) + impl_update!($table {}) + impl_delete!($table {});

pub async fn test_fn_paths(){

    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)  // 设置日志级别为Debug
        .init();


    let rb = RBatis::new();
    rb.init(MysqlDriver {}, DB_URL).expect("mysql init fail");
    debug!("start...");

    let res = Xxx::select_all(&rb).await.unwrap();

    let  mut i = 0;

    let mut node = HashMap::with_capacity(res.len());

    for (_,v) in res.iter().enumerate(){
        if v.local_ip .is_empty() || v.remote_ip.is_empty() {
            continue;
        }
        if node.get(&v.local_ip).is_none(){
            node.insert(v.local_ip.clone(),i);
            i +=1;
        }
        if node.get(&v.remote_ip).is_none(){
            node.insert(v.remote_ip.clone(),i);
            i +=1;
        }
    }

    let mut input_graph = InputGraph::new();



    for (_,v) in res.into_iter().enumerate(){
        if v.local_ip.is_empty() || v.remote_ip.is_empty() {
            continue;
        }

        let local = node.get(&v.local_ip).unwrap();
        let remote = node.get(&v.remote_ip).unwrap();
        input_graph.add_edge(local.clone(), remote.clone(), 12);

    }
    debug!("start...");
    input_graph.freeze();

    let fast_graph = fast_paths::prepare(&input_graph);

    // 计算节点8到节点6的最短路径
    let shortest_path = fast_paths::calc_path(&fast_graph, 8, 6);

    match shortest_path {
        Some(p) => {
            // 获取最短路径的总权重
            let weight = p.get_weight();
            println!("weight {:?}", weight);

            // 获取最短路径经过的所有节点（包含起点和终点）
            let nodes = p.get_nodes();
            println!("nodes {:?}", nodes);
        },
        None => {
            // 未找到路径（说明两个节点在图中间不连通）
        }
    }


    // println!("{:#?}",node);
}

const DB_URL: &str = "mysql://ssss:ssss@127.0.0.1:3306/ssss";
