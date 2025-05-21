
use petgraph::{
    algo::dijkstra,
    graph::{Graph, NodeIndex},
    visit::EdgeRef,
};
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Device {
    name: String,
    ip: String,
    device_type: String,
    position: (i32, i32),
}

fn find_all_shortest_paths(
    graph: &Graph<Device, i32>,
    start: NodeIndex,
    end: NodeIndex,
) -> Vec<Vec<NodeIndex>> {
    let mut predecessors: HashMap<NodeIndex, Vec<NodeIndex>> = HashMap::new();
    let distances = dijkstra(graph, start, Some(end), |e| *e.weight());

    for edge in graph.edge_references() {
        let (u, v) = (edge.source(), edge.target());
        let cost = *edge.weight();

        if distances.get(&u).and_then(|d_u| distances.get(&v).map(|d_v| d_u + cost == *d_v)).unwrap_or(false) {
            predecessors.entry(v).or_default().push(u);
        }
    }

    let mut paths = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(vec![end]);

    while let Some(mut path) = queue.pop_front() {
        let current = *path.first().unwrap();

        if current == start {
            path.reverse();
            paths.push(path);
            continue;
        }

        if let Some(preds) = predecessors.get(&current) {
            for &pred in preds {
                let mut new_path = path.clone();
                new_path.insert(0, pred);
                queue.push_back(new_path);
            }
        }
    }

    paths
}

pub fn test_cache() {
    let mut graph = Graph::<Device, i32>::new();

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

    graph.add_edge(router, switch, 1);
    graph.add_edge(switch, server, 1);
    graph.add_edge(switch, pc, 1);
    graph.add_edge(switch, iot, 1);
    graph.add_edge(router, pc, 1);
    graph.add_edge(pc, server, 1);

    let start = router;
    let end = server;

    let all_paths = find_all_shortest_paths(&graph, start, end);

    println!("Found {} equivalent shortest paths:", all_paths.len());
    for (i, path) in all_paths.iter().enumerate() {
        println!("Path {} ({} hops):", i + 1, path.len() - 1);
        for node in path {
            println!("  {}", graph[*node].name);
        }
    }
}
