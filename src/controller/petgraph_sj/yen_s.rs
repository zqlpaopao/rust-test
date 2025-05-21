
use petgraph::algo::dijkstra;
use petgraph::graph::{Graph, NodeIndex, EdgeIndex};
use std::collections::{BinaryHeap, HashSet, HashMap};
use petgraph::prelude::EdgeRef;

#[derive(Debug, Clone)]
struct Device {
    name: String,
    ip: String,
    device_type: String,
    position: (i32, i32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Path {
    nodes: Vec<NodeIndex>,
    weight: i32,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weight.cmp(&self.weight)
            .then_with(|| self.nodes.len().cmp(&other.nodes.len()))
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn yen_k_shortest_paths(
    graph: &Graph<Device, i32>,
    start: NodeIndex,
    end: NodeIndex,
    k: usize,
) -> Vec<Path> {
    let mut paths = Vec::new();
    let mut heap = BinaryHeap::new();
    let mut used_paths = HashSet::new();

    if let Some(first_path) = find_shortest_path(graph, start, end) {
        heap.push(first_path);
    }

    while let Some(path) = heap.pop() {
        if paths.len() >= k {
            break;
        }

        paths.push(path.clone());
        used_paths.insert(path.nodes.clone());

        for i in 0..path.nodes.len() - 1 {
            let spur_node = path.nodes[i];
            let root_path = &path.nodes[..=i];

            // 创建临时图副本
            let mut temp_graph = graph.clone();

            // 查找并移除特定边
            for p in &paths {
                if p.nodes.starts_with(root_path) && p.nodes.len() > i + 1 {
                    if let Some(edge) = temp_graph.find_edge(p.nodes[i], p.nodes[i + 1]) {
                        temp_graph.remove_edge(edge);
                    }
                }
            }

            if let Some(spur_path) = find_shortest_path(&temp_graph, spur_node, end) {
                let mut new_nodes = root_path.to_vec();
                new_nodes.extend(spur_path.nodes.iter().skip(1));
                let new_weight = calculate_path_weight(graph, &new_nodes);

                let new_path = Path {
                    nodes: new_nodes,
                    weight: new_weight,
                };

                if !used_paths.contains(&new_path.nodes) {
                    heap.push(new_path);
                }
            }
        }
    }

    paths
}

fn find_shortest_path(
    graph: &Graph<Device, i32>,
    start: NodeIndex,
    end: NodeIndex,
) -> Option<Path> {
    let node_map = dijkstra(graph, start, Some(end), |e| *e.weight());
    println!("{:?}", node_map);
    let shortest_len = *node_map.get(&end)?;

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

    paths.into_iter().next()
}

fn calculate_path_weight(
    graph: &Graph<Device, i32>,
    nodes: &[NodeIndex],
) -> i32 {
    nodes.windows(2)
        .map(|w| graph.edges_connecting(w[0], w[1]).next().unwrap().weight())
        .sum()
}

fn dfs_find_paths(
    graph: &Graph<Device, i32>,
    target: NodeIndex,
    target_len: i32,
    current_len: i32,
    current_path: &mut Vec<NodeIndex>,
    visited: &mut HashSet<NodeIndex>,
    paths: &mut Vec<Path>,
) {
    let last_node = *current_path.last().unwrap();

    if last_node == target {
        if current_len == target_len {
            paths.push(Path {
                nodes: current_path.clone(),
                weight: current_len,
            });
        }
        return;
    }

    visited.insert(last_node);

    for edge in graph.edges(last_node) {
        let next_node = edge.target();
        if !visited.contains(&next_node) {
            let new_len = current_len + edge.weight();
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


pub fn test_yen_s() {
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

    // 添加边
    graph.add_edge(router, switch, 1);
    graph.add_edge(switch, server, 1);
    graph.add_edge(switch, pc, 1);
    graph.add_edge(switch, iot, 1);
    graph.add_edge(router, pc, 1);
    graph.add_edge(pc, server, 1);

    let start = router;
    let end = server;

    let all_paths = yen_k_shortest_paths(&graph, start, end, 10);

    println!("Found {} shortest paths:", all_paths.len());
    for (i, path) in all_paths.iter().enumerate() {
        println!("Path {} (weight: {}, hops: {}):",
                 i + 1,
                 path.weight,
                 path.nodes.len() - 1
        );
        for node in &path.nodes {
            println!("  {}", graph[*node].name);
        }
    }
}