
#![allow(unused)]
// https://mp.weixin.qq.com/s/2scj8OGekFHc8ZvXUrY09w

use std::collections::{HashSet, VecDeque};
use std::hash::RandomState;
use hashbrown::HashMap; // 明确使用 hashbrown
use petgraph::{Directed, Direction, Undirected};
use petgraph::dot::{Dot, Config};
use serde_derive::{Deserialize, Serialize};
use petgraph::Graph;
use petgraph::algo::{astar, dijkstra, toposort, Cycle, DfsSpace, spfa, connected_components, all_simple_paths, k_shortest_path};
use petgraph::graph::NodeIndex;
use petgraph::prelude::EdgeRef;
use petgraph::visit::Bfs;

#[derive(Debug,Clone)]
struct Device {
    name: &'static str,
    ip: &'static str,
    device_type: &'static str,
}


pub fn test_pet_graph() {
    // graph()

    //有向图 -- 无向图
    // direction()

    //拓扑排序
    // topology_sort()

    // 加权图的最短路径算法
    // dijkstra_shortest_path_example();

    // 使用 astar 直接获取最短路径
    // shortest_path_with_astar();


    //无权图直接获取最短路径
    // no_weight()



    // 无向图的连通分量算法
    // connected_components_example()

    //获取所有可达路径
    find_all_path()



}

/**************************** 获取所有最短路径 ************************************/

/**************************** 获取所有最短路径 ************************************/

fn find_all_path(){
    use petgraph::graph::Graph;

    let mut graph = Graph::<Device1, GraphEdge>::new();

    // 添加节点（与你的代码相同）
    let router = graph.add_node(Device1 {
        name: "Router",
        ip: "192.168.1.1",
        device_type: "router",
        position: (0, 0),
    });

    let switch = graph.add_node(Device1 {
        name: "Switch",
        ip: "192.168.1.2",
        device_type: "switch",
        position: (1, 0),
    });

    let server = graph.add_node(Device1 {
        name: "Server",
        ip: "192.168.1.3",
        device_type: "server",
        position: (2, 0),
    });

    let pc = graph.add_node(Device1 {
        name: "PC",
        ip: "192.168.1.4",
        device_type: "pc",
        position: (3, 0),
    });

    let iot = graph.add_node(Device1 {
        name: "IoT",
        ip: "192.168.1.5",
        device_type: "iot",
        position: (4, 0),
    });

    // 添加边（与你的代码相同）
    graph.add_edge(router, switch, GraphEdge { from: 1, to: 2, weight: 1 });
    graph.add_edge( switch, router,GraphEdge { from: 1, to: 2, weight: 1 });

    graph.add_edge(switch, server, GraphEdge { from: 1, to: 2, weight: 1 });
    graph.add_edge( server,switch, GraphEdge { from: 1, to: 2, weight: 1 });

    graph.add_edge(switch, pc, GraphEdge { from: 1, to: 2, weight: 1 });
    graph.add_edge( pc,switch, GraphEdge { from: 1, to: 2, weight: 1 });

    graph.add_edge(switch, iot, GraphEdge { from: 1, to: 2, weight: 1 });
    graph.add_edge( iot,switch, GraphEdge { from: 1, to: 2, weight: 1 });

    graph.add_edge(router, pc, GraphEdge { from: 1, to: 2, weight: 1 });
    graph.add_edge(pc, router,GraphEdge { from: 1, to: 2, weight: 1 });

    graph.add_edge(pc, server, GraphEdge { from: 1, to: 2, weight: 1 });
    graph.add_edge( server,pc, GraphEdge { from: 1, to: 2, weight: 1 });

    let start = router;
    let end = server;

    let res = all_shortest_paths_no_cycles(&graph,start,end);

    println!("{:?}",res);
}


fn all_shortest_paths_no_cycles(
    graph: &Graph<Device1, GraphEdge>,
    start: NodeIndex,
    end: NodeIndex,
) -> Vec<Vec<NodeIndex>> {
    // 1. 计算从起点到所有节点的最短距离
    let node_map = dijkstra(graph, start, Some(end), |e| e.weight().weight as usize);
    let total_cost = node_map[&end];

    // 2. 从终点回溯所有最短路径
    let mut paths = Vec::new();
    let mut stack = vec![(vec![end], total_cost)];

    while let Some((path, remaining_cost)) = stack.pop() {
        let last_node = *path.last().unwrap();

        if last_node == start {
            paths.push(path.into_iter().rev().collect()); // 反转路径
            continue;
        }

        // 遍历所有可能的前驱节点
        for neighbor in graph.neighbors_directed(last_node, Direction::Incoming) {
            if let Some(edge) = graph.find_edge(neighbor, last_node) {
                let edge_cost = graph[edge].weight as usize;

                // 检查是否构成最短路径的一部分
                if let Some(&pred_cost) = node_map.get(&neighbor) {
                    if pred_cost + edge_cost == remaining_cost && !path.contains(&neighbor) {
                        let mut new_path = path.clone();
                        new_path.push(neighbor);
                        stack.push((new_path, pred_cost));
                    }
                }
            }
        }
    }

    paths
}


// 查找所有最短路径的函数
fn find_all_shortest_paths<N, E>(
    graph: &petgraph::Graph<N, E>,
    start: petgraph::prelude::NodeIndex,
    end: petgraph::prelude::NodeIndex,
    edge_weight: impl Fn(&E) -> i32,
) -> Vec<Vec<petgraph::prelude::NodeIndex>> {
    let mut paths = Vec::new();
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    // (当前节点, 路径, 总权重)
    queue.push_back((start, vec![start], 0));
    visited.insert(start, 0);

    let mut min_cost = None;

    while let Some((node, path, cost)) = queue.pop_front() {
        if node == end {
            match min_cost {
                Some(c) if cost == c => paths.push(path),
                None => {
                    min_cost = Some(cost);
                    paths.push(path);
                }
                _ => break,
            }
            continue;
        }

        // 如果已经找到最小成本，且当前成本大于最小成本，跳过
        if let Some(c) = min_cost {
            if cost > c {
                continue;
            }
        }

        for edge in graph.edges_directed(node, Direction::Outgoing) {
            let next_node = edge.target();
            let new_cost = cost + edge_weight(edge.weight());

            // 如果没访问过，或者找到了更短的路径
            if !visited.contains_key(&next_node) || new_cost <= *visited.get(&next_node).unwrap() {
                visited.insert(next_node, new_cost);
                let mut new_path = path.clone();
                new_path.push(next_node);
                queue.push_back((next_node, new_path, new_cost));
            }
        }
    }

    paths
}


/**************************** 无向图的连通分量算法 ************************************/
fn connected_components_example() {
    // 创建一个无向图
    let mut graph: Graph<&str, ()> = Graph::new();

    // 添加节点
    let node_a = graph.add_node("A");
    let node_b = graph.add_node("B");
    let node_c = graph.add_node("C");
    let node_d = graph.add_node("D");
    let node_e = graph.add_node("E");
    let node_f = graph.add_node("F");

    // 添加边，形成两个连通分量
    graph.add_edge(node_a, node_b, ());
    graph.add_edge(node_b, node_c, ()); // A, B, C 形成一个连通分量

    graph.add_edge(node_d, node_e, ());
    graph.add_edge(node_e, node_f, ()); // D, E, F 形成一个连通分量

    // 计算连通分量
    let mut component_mapping = vec![0; graph.node_count()];
    let num_components = connected_components(&graph);

    // 输出连通分量数量
    println!("连通分量数量: {}", num_components);

    // 获取每个连通分量的节点集合
    let mut components = vec![Vec::new(); num_components];
    for (node, component) in graph.node_indices().zip(component_mapping) {
        components[component].push(graph[node]);
    }

    // 输出每个连通分量的节点集合
    for (i, component) in components.into_iter().enumerate() {
        println!("连通分量 {}: {:?}", i, component);
    }
}

/**************************** 无权图直接获取最短路径 ************************************/
// BFS 算法实现
fn shortest_path_bfs(
    graph: &Graph<Device1, GraphEdge>,
    start: NodeIndex,
    end: NodeIndex,
) -> Option<(Vec<NodeIndex>, Vec<GraphEdge>)> {
    // 检查起点和终点是否相同
    if start == end {
        return Some((vec![start], vec![]));
    }

    // 初始化队列和已访问集合
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    // 将起点加入队列并标记为已访问
    queue.push_back(start);
    visited.insert(start, None);

    // BFS 遍历
    while let Some(current) = queue.pop_front() {
        // 遍历当前节点的所有邻居
        for neighbor in graph.neighbors(current) {
            // 如果邻居未被访问过
            if !visited.contains_key(&neighbor) {
                // 记录前驱节点
                visited.insert(neighbor, Some(current));

                // 检查是否到达终点
                if neighbor == end {
                    // 重建路径
                    return reconstruct_path(graph, &visited, start, end);
                }

                // 将邻居加入队列
                queue.push_back(neighbor);
            }
        }
    }

    // 未找到路径
    None
}

// 路径重建函数
fn reconstruct_path(
    graph: &Graph<Device1, GraphEdge>,
    visited: &HashMap<NodeIndex, Option<NodeIndex>>,
    start: NodeIndex,
    end: NodeIndex,
) -> Option<(Vec<NodeIndex>, Vec<GraphEdge>)> {
    let mut path_nodes = Vec::new();
    let mut path_edges = Vec::new();
    let mut current = end;

    // 从终点回溯到起点
    while current != start {
        path_nodes.push(current);

        // 获取前驱节点
        let prev = match visited.get(&current) {
            Some(Some(prev)) => *prev,
            _ => return None, // 路径不完整，理论上不会发生
        };

        // 查找对应的边
        let edge_found = graph
            .edges_directed(prev, petgraph::Direction::Outgoing)
            .find(|e| e.target() == current)
            .map(|e| *e.weight());

        // 将边添加到路径中
        if let Some(edge) = edge_found {
            path_edges.push(edge);
        } else {
            return None; // 图结构异常，理论上不会发生
        }

        current = prev;
    }

    // 添加起点
    path_nodes.push(start);

    // 反转路径以获得正确顺序
    path_nodes.reverse();
    path_edges.reverse();

    Some((path_nodes, path_edges))
}
// 双向BFS算法
// 双向BFS算法
fn bidirectional_bfs(
    graph: &Graph<Device1, GraphEdge>,
    start: NodeIndex,
    end: NodeIndex,
) -> Option<(Vec<NodeIndex>, Vec<GraphEdge>)> {
    // 起点和终点相同的情况
    if start == end {
        return Some((vec![start], vec![]));
    }

    // 定义两个方向的队列和距离记录
    let mut forward_queue = VecDeque::new();
    let mut backward_queue = VecDeque::new();
    let mut forward_dist = HashMap::new();
    let mut backward_dist = HashMap::new();
    let mut forward_pred = HashMap::new();
    let mut backward_pred = HashMap::new();

    // 初始化起点和终点的距离
    forward_dist.insert(start, 0);
    backward_dist.insert(end, 0);
    forward_queue.push_back(start);
    backward_queue.push_back(end);
    forward_pred.insert(start, None);
    backward_pred.insert(end, None);

    // 交替扩展两个方向的搜索
    let mut meet_node: Option<NodeIndex> = None;
    let mut min_dist = usize::MAX;

    while !forward_queue.is_empty() || !backward_queue.is_empty() {
        // 扩展正向搜索
        if !forward_queue.is_empty() {
            let current = forward_queue.pop_front().unwrap();
            let current_dist = forward_dist[&current];

            // 如果当前距离已经超过最小相遇距离，停止搜索
            if current_dist >= min_dist {
                continue;
            }

            for neighbor in graph.neighbors(current) {
                let new_dist = current_dist + 1;

                // 如果在反向搜索中已经访问过该节点
                if let Some(back_dist) = backward_dist.get(&neighbor) {
                    let total_dist = new_dist + *back_dist;
                    if total_dist < min_dist {
                        min_dist = total_dist;
                        meet_node = Some(neighbor);
                    }
                }

                // 更新正向搜索距离
                if !forward_dist.contains_key(&neighbor) || new_dist < forward_dist[&neighbor] {
                    forward_dist.insert(neighbor, new_dist);
                    forward_pred.insert(neighbor, Some(current));
                    forward_queue.push_back(neighbor);
                }
            }
        }

        // 扩展反向搜索
        if !backward_queue.is_empty() {
            let current = backward_queue.pop_front().unwrap();
            let current_dist = backward_dist[&current];

            // 如果当前距离已经超过最小相遇距离，停止搜索
            if current_dist >= min_dist {
                continue;
            }

            for neighbor in graph.neighbors(current) {
                let new_dist = current_dist + 1;

                // 如果在正向搜索中已经访问过该节点
                if let Some(forward_dist) = forward_dist.get(&neighbor) {
                    let total_dist = new_dist + *forward_dist;
                    if total_dist < min_dist {
                        min_dist = total_dist;
                        meet_node = Some(neighbor);
                    }
                }

                // 更新反向搜索距离
                if !backward_dist.contains_key(&neighbor) || new_dist < backward_dist[&neighbor] {
                    backward_dist.insert(neighbor, new_dist);
                    backward_pred.insert(neighbor, Some(current));
                    backward_queue.push_back(neighbor);
                }
            }
        }

        // 如果已经找到相遇点且两个队列的当前层都处理完，退出循环
        if meet_node.is_some() && forward_queue.is_empty() && backward_queue.is_empty() {
            break;
        }
    }

    // 如果没有相遇点，返回None
    if let Some(meet) = meet_node {
        // 重建路径
        reconstruct_path1(graph, &forward_pred, &backward_pred, start, end, meet)
    } else {
        None
    }
}

// 路径重建函数
fn reconstruct_path1(
    graph: &Graph<Device1, GraphEdge>,
    forward_pred: &HashMap<NodeIndex, Option<NodeIndex>>,
    backward_pred: &HashMap<NodeIndex, Option<NodeIndex>>,
    start: NodeIndex,
    end: NodeIndex,
    meet_node: NodeIndex,
) -> Option<(Vec<NodeIndex>, Vec<GraphEdge>)> {
    let mut path_nodes = Vec::new();
    let mut path_edges = Vec::new();

    // 从相遇节点向前重建到起点的路径
    let mut current = meet_node;
    while current != start {
        path_nodes.push(current);

        // 查找前驱节点
        let prev = match forward_pred.get(&current) {
            Some(Some(prev)) => *prev,
            _ => return None, // 路径不完整
        };

        // 查找对应的边
        let edge = find_edge(graph, prev, current)?;
        path_edges.push(edge);

        current = prev;
    }
    path_nodes.push(start);
    path_nodes.reverse(); // 反转路径顺序
    path_edges.reverse(); // 反转边的顺序

    // 从相遇节点向后重建到终点的路径
    let mut current = meet_node;
    while current != end {
        // 查找后继节点
        let next = match backward_pred.get(&current) {
            Some(Some(next)) => *next,
            _ => return None, // 路径不完整
        };

        // 查找对应的边
        let edge = find_edge(graph, current, next)?;
        path_edges.push(edge);

        path_nodes.push(next);
        current = next;
    }

    Some((path_nodes, path_edges))
}

// 辅助函数：查找两个节点之间的边
fn find_edge(graph: &Graph<Device1, GraphEdge>, from: NodeIndex, to: NodeIndex) -> Option<GraphEdge> {
    for edge in graph.edges_directed(from, petgraph::Direction::Outgoing) {
        if edge.target() == to {
            return Some(*edge.weight());
        }
    }
    None
}



// 简化Dijkstra算法 - 无加权图最短路径
fn shortest_path_dijkstra(
    graph: &Graph<Device1, GraphEdge>,
    start: NodeIndex,
    end: NodeIndex,
) -> Option<(Vec<NodeIndex>, Vec<GraphEdge>)> {
    let mut distances = HashMap::new();
    let mut predecessors = HashMap::new();
    let mut queue = VecDeque::new();

    distances.insert(start, 0);
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        for neighbor in graph.neighbors(current) {
            let new_distance = distances[&current] + 1;

            if let Some(existing_distance) = distances.get(&neighbor) {
                if new_distance < *existing_distance {
                    distances.insert(neighbor, new_distance);
                    predecessors.insert(neighbor, current);
                    queue.push_back(neighbor);
                }
            } else {
                distances.insert(neighbor, new_distance);
                predecessors.insert(neighbor, current);
                queue.push_back(neighbor);
            }
        }
    }

    // 重建路径
    let mut path_nodes = Vec::new();
    let mut path_edges = Vec::new();
    let mut current = end;

    if let Some(_) = distances.get(&end) {
        while current != start {
            path_nodes.push(current);
            if let Some(prev) = predecessors.get(&current) {
                for edge in graph.edges_directed(*prev, petgraph::Direction::Outgoing) {
                    if edge.target() == current {
                        path_edges.push(*edge.weight());
                        break;
                    }
                }
                current = *prev;
            }
        }
        path_nodes.push(start);
        path_nodes.reverse();
        path_edges.reverse();

        return Some((path_nodes, path_edges));
    }

    None
}

fn no_weight() {
    // 创建一个有向图
    let mut graph = Graph::<Device1, GraphEdge>::new();

    // 添加节点
    let router = graph.add_node(Device1 {
        name: "Router",
        ip: "192.168.1.1",
        device_type: "router",
        position: (0, 0),
    });

    let switch = graph.add_node(Device1 {
        name: "Switch",
        ip: "192.168.1.2",
        device_type: "switch",
        position: (1, 0),
    });

    let server = graph.add_node(Device1 {
        name: "Server",
        ip: "192.168.1.3",
        device_type: "server",
        position: (2, 0),
    });

    let pc = graph.add_node(Device1 {
        name: "PC",
        ip: "192.168.1.4",
        device_type: "pc",
        position: (3, 0),
    });

    let iot = graph.add_node(Device1 {
        name: "IoT",
        ip: "192.168.1.5",
        device_type: "iot",
        position: (4, 0),
    });

    // 添加边（无向图中权重仅用于展示）
    graph.add_edge(router, switch, GraphEdge{
        from: 1,
        to: 2,
        weight: 1,
    });
    graph.add_edge(switch, server, GraphEdge{
        from: 1,
        to: 2,
        weight: 1,
    });
    graph.add_edge(switch, pc, GraphEdge{
        from: 1,
        to: 2,
        weight: 1,
    });
    graph.add_edge(switch, iot, GraphEdge{
        from: 1,
        to: 2,
        weight: 1,
    });

    graph.add_edge(router, pc, GraphEdge{
        from: 1,
        to: 2,
        weight: 1,
    });
    graph.add_edge(pc, server, GraphEdge{
        from: 1,
        to: 2,
        weight: 1,
    });


    let start = router;
    let end = server;

    if let Some((nodes, edges)) = bidirectional_bfs(&graph, start, end) {
        println!("最短路径（{}步）:", nodes.len() - 1);
        for (i, node) in nodes.iter().enumerate() {
            println!("  {}: {}", i, graph[*node].name);
        }
        println!("路径边:");
        for (i, edge) in edges.iter().enumerate() {
            println!("  {}: 权重={}", i, edge.weight);
        }
    } else {
        println!("无法从起点到达终点");
    }

}


/**************************** 使用 astar 直接获取最短路径 ************************************/
// 定义设备的结构体
#[derive(Debug)]
struct Device1 {
    name: &'static str,
    ip: &'static str,
    device_type: &'static str,
    // 假设设备有坐标信息，这里简单用元组表示
    position: (i32, i32),
}
fn shortest_path_with_astar() {
    // 创建一个有向图
    let mut graph = Graph::<Device1, GraphEdge>::new();

    // 添加节点
    let router = graph.add_node(Device1 {
        name: "Router",
        ip: "192.168.1.1",
        device_type: "router",
        position: (0, 0),
    });

    let switch = graph.add_node(Device1 {
        name: "Switch",
        ip: "192.168.1.2",
        device_type: "switch",
        position: (1, 0),
    });

    let server = graph.add_node(Device1 {
        name: "Server",
        ip: "192.168.1.3",
        device_type: "server",
        position: (2, 0),
    });

    let pc = graph.add_node(Device1 {
        name: "PC",
        ip: "192.168.1.4",
        device_type: "pc",
        position: (3, 0),
    });

    let iot = graph.add_node(Device1 {
        name: "IoT",
        ip: "192.168.1.5",
        device_type: "iot",
        position: (4, 0),
    });

    // 添加边
    graph.add_edge(router, switch, GraphEdge{
        from: 1,
        to: 2,
        weight: 10,
    });
    graph.add_edge(switch, server, GraphEdge{
        from: 1,
        to: 2,
        weight: 1,
    });
    graph.add_edge(switch, pc, GraphEdge{
        from: 1,
        to: 2,
        weight: 2,
    });
    graph.add_edge(switch, iot, GraphEdge{
        from: 1,
        to: 2,
        weight: 3,
    });

    // 计算从 Router 到 Server 的最短路径
    let start_node = router;
    let target_node = server;

    // 使用 astar 算法，启发式函数设为 |e| 0 表示退化为 Dijkstra 算法
    let result = astar(
        &graph,
        start_node,
        |node| node == target_node, // 目标节点判断
        |e| e.weight().weight,             // 边权重
        // |_| 0                        // 启发式函数（设为 0 等价于 Dijkstra）
        |node| manhattan_distance(&graph[node], &graph[target_node]).try_into().unwrap(), //曼哈顿算法
    );

    // 处理结果
    if let Some((distance, path)) = result {
        println!("从 Router 到 Server 的最短距离: {}", distance);
        println!("路径:");

        // 输出路径中的节点
        for (i, node) in path.iter().enumerate() {
            println!("  {}: {:?}", i, graph[*node].name);
        }

        // 输出路径中的边
        println!("经过的边:");
        for i in 0..path.len() - 1 {
            let src = path[i];
            let dst = path[i + 1];

            // 查找边权重
            for edge in graph.edges(src) {
                if edge.target() == dst {
                    println!("  {} -> {} (权重: {})",
                             graph[src].name,
                             graph[dst].name,
                             edge.weight().weight);
                    break;
                }
            }
        }
    } else {
        println!("无法从 Router 到达 Server");
    }
}


fn manhattan_distance(src: &Device1, dst: &Device1) -> i32 {
    let (x1, y1) = src.position;
    let (x2, y2) = dst.position;
    (x1 - x2).abs() + (y1 - y2).abs()
}




/**************************** 加权图的最短路径算法 ************************************/

fn dijkstra_shortest_path_example() {
    // 创建一个有向图
    let mut topo = Graph::<Device, i32>::new();

    // 添加节点
    let router = topo.add_node(Device {
        name: "Router",
        ip: "192.168.1.1",
        device_type: "router",
    });

    let switch = topo.add_node(Device {
        name: "Switch",
        ip: "192.168.1.2",
        device_type: "switch",
    });

    let server = topo.add_node(Device {
        name: "Server",
        ip: "192.168.1.3",
        device_type: "server",
    });

    let pc = topo.add_node(Device {
        name: "PC",
        ip: "192.168.1.4",
        device_type: "pc",
    });

    let iot = topo.add_node(Device {
        name: "IoT",
        ip: "192.168.1.5",
        device_type: "iot",
    });

    // 添加边
    topo.add_edge(router, switch, 10);
    topo.add_edge(switch, server, 1);
    topo.add_edge(switch, pc, 2);
    topo.add_edge(switch, iot, 3);

    // 计算从Router到其他所有节点的最短路径
    let start_node = router;
    let distances = dijkstra(&topo, start_node, None, |e| *e.weight());

    // 输出最短路径结果
    println!("从节点 {:?} 到其他节点的最短路径:", topo[start_node]);
    for (node_index, distance) in distances.iter() {
        println!("到节点 {:?} 的距离: {}", topo[*node_index], distance);

        // 重建路径
        if let Some(path) = build_path(&topo, &distances, start_node, *node_index) {
            println!("  路径: {}", format_path(&topo, &path));
        } else {
            println!("  无路径");
        }
    }

    // 查找特定节点之间的最短路径
    let target_node = server;
    if let Some(dist) = distances.get(&target_node) {
        println!("从节点 {:?} 到节点 {:?} 的最短距离: {}",
                 topo[start_node], topo[target_node], dist);

        if let Some(path) = build_path(&topo, &distances, start_node, target_node) {
            println!("  路径: {}", format_path(&topo, &path));

            // 输出路径中的边
            if let Some(edges) = get_path_edges(&topo, &path) {
                println!("  经过的边:");
                for (src, dst, weight) in edges {
                    println!("    {} -> {} (权重: {})", src, dst, weight);
                }
            }
        }
    } else {
        println!("无法从节点 {:?} 到达节点 {:?}", topo[start_node], topo[target_node]);
    }
}

// 重建最短路径（使用边松弛信息）
fn build_path(
    graph: &Graph<Device, i32>,
    distances: &HashMap<NodeIndex, i32>,
    start: NodeIndex,
    end: NodeIndex,
) -> Option<Vec<NodeIndex>> {
    if start == end {
        return Some(vec![start]);
    }

    let mut path = Vec::new();
    let mut current = end;

    path.push(current);

    while current != start {
        let mut found = false;

        // 查找哪个前驱节点可以到达当前节点并形成最短路径
        for edge in graph.edges_directed(current, petgraph::Direction::Incoming) {
            let source = edge.source();
            let weight = *edge.weight();

            if let Some(&src_dist) = distances.get(&source) {
                if src_dist + weight == distances[&current] {
                    path.push(source);
                    current = source;
                    found = true;
                    break;
                }
            }
        }

        if !found {
            return None; // 无法构建路径
        }
    }

    path.reverse();
    Some(path)
}

// 获取路径中的边信息
fn get_path_edges(
    graph: &Graph<Device, i32>,
    path: &[NodeIndex],
) -> Option<Vec<(&'static str, &'static str, i32)>> {
    let mut edges = Vec::new();

    for i in 0..path.len() - 1 {
        let src = path[i];
        let dst = path[i + 1];

        // 查找这两个节点之间的边
        for edge in graph.edges(src) {
            if edge.target() == dst {
                edges.push((
                    graph[src].name,
                    graph[dst].name,
                    *edge.weight()
                ));
                break;
            }
        }
    }

    if edges.len() == path.len() - 1 {
        Some(edges)
    } else {
        None
    }
}

// 格式化路径输出
fn format_path(graph: &Graph<Device, i32>, path: &[NodeIndex]) -> String {
    let node_names: Vec<String> = path.iter()
        .map(|&node| format!("{:?}", graph[node].name))
        .collect();

    node_names.join(" -> ")
}




/**************************** 拓扑排序 ************************************/
fn topology_sort(){
    // 创建一个有向图
    let mut topo = Graph::<Device, &'static str>::new();

    // 添加节点
    let router = topo.add_node(Device {
        name: "Router",
        ip: "192.168.1.1",
        device_type: "router",
    });

    let switch = topo.add_node(Device {
        name: "Switch",
        ip: "192.168.1.2",
        device_type: "switch",
    });

    let server = topo.add_node(Device {
        name: "Server",
        ip: "192.168.1.3",
        device_type: "server",
    });

    let pc = topo.add_node(Device {
        name: "PC",
        ip: "192.168.1.4",
        device_type: "pc",
    });

    let iot = topo.add_node(Device {
        name: "IoT",
        ip: "192.168.1.5",
        device_type: "iot",
    });

    // 添加边
    topo.add_edge(router, switch, "uplink");
    topo.add_edge(switch, server, "lan");
    topo.add_edge(switch, pc, "lan");
    topo.add_edge(switch, iot, "lan");

    // space 参数：允许传入一个可重用的 DfsSpace 缓存，避免重复分配内存
    //
    // None：表示不提供缓存，每次调用都创建新的临时空间
    let mut space = DfsSpace::new(&topo);
    // match toposort(&topo,None) {
    match toposort(&topo,Some(&mut space)) {
        Ok(order) => {
            println!("拓扑排序结果:");
            for node_index in order {
                println!("{:?}", topo[node_index]);
            }
        }
        Err(cycle) => {
            println!("发现循环依赖: {:?}", cycle.node_id());
        }
    }
}

/**************************** 有向图-无向图 ************************************/
fn direction(){
    // 有向图
    let mut directed_graph = Graph::<GraphNode, GraphEdge, Directed>::new();
    let node1 = directed_graph.add_node(GraphNode{ id: 1, name: "server".to_string() });
    let node2 = directed_graph.add_node(GraphNode{ id: 2, name: "tor".to_string() });
    let node3 = directed_graph.add_node(GraphNode{ id: 3, name: "t1".to_string() });

    directed_graph.add_edge(node1, node2, GraphEdge{
        from: 1,
        to: 2,
        weight: 100,
    });

    directed_graph.add_edge(node2, node3, GraphEdge{
        from: 2,
        to: 3,
        weight: 101,
    });

    println!("有向图---》 {:#?}", Dot::with_config(&directed_graph, &[Config::EdgeNoLabel]));

    // 有向图
    let mut directed_graph = Graph::<GraphNode, GraphEdge, Undirected>::new_undirected();
    let node1 = directed_graph.add_node(GraphNode{ id: 1, name: "server".to_string() });
    let node2 = directed_graph.add_node(GraphNode{ id: 2, name: "tor".to_string() });
    let node3 = directed_graph.add_node(GraphNode{ id: 3, name: "t1".to_string() });

    directed_graph.add_edge(node1, node2, GraphEdge{
        from: 1,
        to: 2,
        weight: 100,
    });

    directed_graph.add_edge(node2, node3, GraphEdge{
        from: 2,
        to: 3,
        weight: 101,
    });

    println!("无向图---》 {:#?}", Dot::with_config(&directed_graph, &[Config::EdgeNoLabel]));

}



// 定义图的节点和边的属性
#[derive(Serialize, Deserialize, Debug)]
struct GraphNode {
    id: usize,
    name: String,
}

#[derive(Serialize, Deserialize, Debug,Copy, Clone)]
struct GraphEdge {
    from: usize,
    to: usize,
    weight: u32,
}

///// 结构支持
fn  test1(){
    // 创建一个示例图
    let mut graph = Graph::new();
    let node1 = graph.add_node(GraphNode { id: 1, name: "Node 1".to_string() });
    let node2 = graph.add_node(GraphNode { id: 2, name: "Node 2".to_string() });
    graph.add_edge(node1, node2, GraphEdge { from: 1, to: 2, weight: 42 });

    // 打印加载的图
    println!("Loaded graph: {:?}", graph);
    // Loaded graph: Graph {
    //     Ty: "Directed",
    //     node_count: 2,
    //     edge_count: 1,
    //     edges: (0, 1),
    //     node weights: {
    //         0: GraphNode {
    //             id: 1,
    //             name: "Node 1",
    //         },
    //         1: GraphNode {
    //             id: 2,
    //             name: "Node 2",
    //         },
    //     },
    //     edge weights: {
    //         0: GraphEdge {
    //             from: 1,
    //             to: 2,
    //             weight: 42,
    //         },
    //     },
    // }
}


///
fn graph() {
    // 创建一个有向图
    let mut topo = Graph::<Device, &'static str>::new();

    // 添加节点
    let router = topo.add_node(Device {
        name: "Router",
        ip: "192.168.1.1",
        device_type: "router",
    });

    let switch = topo.add_node(Device {
        name: "Switch",
        ip: "192.168.1.2",
        device_type: "switch",
    });

    let server = topo.add_node(Device {
        name: "Server",
        ip: "192.168.1.3",
        device_type: "server",
    });

    let pc = topo.add_node(Device {
        name: "PC",
        ip: "192.168.1.4",
        device_type: "pc",
    });

    let iot = topo.add_node(Device {
        name: "IoT",
        ip: "192.168.1.5",
        device_type: "iot",
    });

let res= &topo[iot];

    topo.remove_node(switch);

    // 添加边
    topo.add_edge(router, switch, "uplink");
    topo.add_edge(switch, server, "lan");
    topo.add_edge(switch, pc, "lan");
    topo.add_edge(switch, iot, "lan");

    // 打印图的DOT表示
    println!("{:#?}",topo);
    // Graph {
    //     Ty: "Directed",
    //     node_count: 5,
    //     edge_count: 4,
    //     edges: (0, 1), (1, 2), (1, 3), (1, 4),
    //     node weights: {
    //         0: Device {
    //             name: "Router",
    //             ip: "192.168.1.1",
    //             device_type: "router",
    //         },
    //         1: Device {
    //             name: "Switch",
    //             ip: "192.168.1.2",
    //             device_type: "switch",
    //         },
    //         2: Device {
    //             name: "Server",
    //             ip: "192.168.1.3",
    //             device_type: "server",
    //         },
    //         3: Device {
    //             name: "PC",
    //             ip: "192.168.1.4",
    //             device_type: "pc",
    //         },
    //         4: Device {
    //             name: "IoT",
    //             ip: "192.168.1.5",
    //             device_type: "iot",
    //         },
    //     },
    //     edge weights: {
    //         0: "uplink",
    //         1: "lan",
    //         2: "lan",
    //         3: "lan",
    //     },
    // }
    println!("{:#?}", Dot::with_config(&topo, &[Config::EdgeNoLabel]));

    // digraph {
    //     0 [ label = "Device {\l    name: \"Router\",\l    ip: \"192.168.1.1\",\l    device_type: \"router\",\l}\l" ]
    //     1 [ label = "Device {\l    name: \"Switch\",\l    ip: \"192.168.1.2\",\l    device_type: \"switch\",\l}\l" ]
    //     2 [ label = "Device {\l    name: \"Server\",\l    ip: \"192.168.1.3\",\l    device_type: \"server\",\l}\l" ]
    //     3 [ label = "Device {\l    name: \"PC\",\l    ip: \"192.168.1.4\",\l    device_type: \"pc\",\l}\l" ]
    //     4 [ label = "Device {\l    name: \"IoT\",\l    ip: \"192.168.1.5\",\l    device_type: \"iot\",\l}\l" ]
    //     0 -> 1 [ ]
    //     1 -> 2 [ ]
    //     1 -> 3 [ ]
    //     1 -> 4 [ ]
    // }
}
