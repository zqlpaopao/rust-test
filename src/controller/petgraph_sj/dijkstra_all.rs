
use petgraph::algo::dijkstra;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::EdgeRef;
use std::collections::HashSet;
use std::ops::Add;
use std::cmp::PartialOrd;
use std::default::Default;

#[derive(Debug)]
struct Device {
    name: String,
    ip: String,
    device_type: String,
    position: (i32, i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Connection {
    from: i32,
    to: i32,
    weight: i32,
}

impl Connection {
    fn weight(&self) -> i32 {
        self.weight
    }
}

impl Add for Connection {
    type Output = i32;

    fn add(self, rhs: Self) -> i32 {
        self.weight + rhs.weight
    }
}

fn find_all_equivalent_paths<N>(
    graph: &Graph<N, i32>,
    start: NodeIndex,
    end: NodeIndex,
) -> Vec<Vec<NodeIndex>> {
    // 1. 使用Dijkstra获取最短路径长度
    let node_map = dijkstra(graph, start, Some(end), |e| *e.weight());
    let shortest_len = *node_map.get(&end).unwrap_or(&i32::MAX);

    // 2. 递归查找所有路径
    let mut paths = Vec::new();
    let mut visited = HashSet::new();
    let mut current_path = vec![start];

    dfs_find_paths(
        graph,
        end,
        shortest_len,
        0,
        &mut current_path,
        &mut visited,
        &mut paths,
    );

    paths
}

fn dfs_find_paths<N>(
    graph: &Graph<N, i32>,
    target: NodeIndex,
    target_len: i32,
    current_len: i32,
    current_path: &mut Vec<NodeIndex>,
    visited: &mut HashSet<NodeIndex>,
    paths: &mut Vec<Vec<NodeIndex>>,
) {
    let last_node = *current_path.last().unwrap();

    if last_node == target {
        if current_len == target_len {
            paths.push(current_path.clone());
        }
        return;
    }

    visited.insert(last_node);

    for edge in graph.edges(last_node) {
        let next_node = edge.target();
        if !visited.contains(&next_node) {
            let new_len = current_len + *edge.weight();
            if new_len <= target_len {
                current_path.push(next_node);
                dfs_find_paths(
                    graph,
                    target,
                    target_len,
                    new_len,
                    current_path,
                    visited,
                    paths,
                );
                current_path.pop();
            }
        }
    }

    visited.remove(&last_node);
}

fn test_pet_graph() {
    let mut graph = Graph::<Device, i32>::new();

    // 添加节点
    let router = graph.add_node(Device {
        name: "Router".to_string(),
        ip: "192.168.1.1".to_string(),
        device_type: "router".to_string(),
        position: (0, 0),
    });

    let switch = graph.add_node(Device {
        name: "Switch".to_string(),
        ip: "192.168.1.2".to_string(),
        device_type: "switch".to_string(),
        position: (1, 0),
    });

    let server = graph.add_node(Device {
        name: "Server".to_string(),
        ip: "192.168.1.3".to_string(),
        device_type: "server".to_string(),
        position: (2, 0),
    });

    let pc = graph.add_node(Device {
        name: "PC".to_string(),
        ip: "192.168.1.4".to_string(),
        device_type: "pc".to_string(),
        position: (3, 0),
    });

    let iot = graph.add_node(Device {
        name: "IoT".to_string(),
        ip: "192.168.1.5".to_string(),
        device_type: "iot".to_string(),
        position: (4, 0),
    });

    // 添加边（直接使用i32作为权重）
    graph.add_edge(router, switch, 1);
    graph.add_edge(switch, server, 1);
    graph.add_edge(switch, pc, 1);
    graph.add_edge(switch, iot, 1);
    graph.add_edge(router, pc, 1);
    graph.add_edge(pc, server, 1);

    let start = router;
    let end = server;

    let all_paths = find_all_equivalent_paths(&graph, start, end);

    println!("Found {} equivalent shortest paths:", all_paths.len());
    for (i, path) in all_paths.iter().enumerate() {
        println!("Path {} ({} hops):", i + 1, path.len() - 1);
        for node in path {
            println!("  {}", graph[*node].name);
        }
    }
}
