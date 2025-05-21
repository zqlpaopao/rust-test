

# 1、基础图建设

有向图VS无向图

```
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


有向图---》 digraph {
    0 [ label = "GraphNode {\l    id: 1,\l    name: \"server\",\l}\l" ]
    1 [ label = "GraphNode {\l    id: 2,\l    name: \"tor\",\l}\l" ]
    2 [ label = "GraphNode {\l    id: 3,\l    name: \"t1\",\l}\l" ]
    0 -> 1 [ ]
    1 -> 2 [ ]
}

无向图---》 graph {
    0 [ label = "GraphNode {\l    id: 1,\l    name: \"server\",\l}\l" ]
    1 [ label = "GraphNode {\l    id: 2,\l    name: \"tor\",\l}\l" ]
    2 [ label = "GraphNode {\l    id: 3,\l    name: \"t1\",\l}\l" ]
    0 -- 1 [ ]
    1 -- 2 [ ]
}

```



# 2、拓扑排序

在图论中，**拓扑排序（Topological Sorting）** 是对 **有向无环图（DAG, Directed Acyclic Graph）** 的顶点进行排序的一种算法。排序结果需满足：**对于图中的任意一条有向边 (u → v)，顶点 u 在排序中必须出现在顶点 v 之前**。


### 拓扑排序的核心依据
拓扑排序的顺序主要基于图的 **依赖关系** 和 **入度（In-degree）**：
1. **依赖关系**：如果存在一条边从 A 指向 B，表示 B 依赖于 A。因此，A 必须排在 B 之前。
2. **入度**：顶点的入度是指指向该顶点的边的数量。拓扑排序通常从 **入度为 0 的顶点** 开始（即没有前置依赖的顶点），逐步移除这些顶点并更新剩余顶点的入度，直到所有顶点被排序或发现环。


### 常见算法实现
#### 1. Kahn 算法（基于 BFS）
- **步骤**：
  1. 计算每个顶点的入度。
  2. 将所有入度为 0 的顶点加入队列。
  3. 从队列中取出一个顶点，将其加入排序结果，并移除该顶点的所有出边（更新相邻顶点的入度）。
  4. 如果相邻顶点的入度变为 0，将其加入队列。
  5. 重复步骤 3-4，直到队列为空。

- **特点**：可以检测图中是否存在环（若排序结果的顶点数少于图的总顶点数，则存在环）。

#### 2. 深度优先搜索（DFS）
- **步骤**：
  1. 对图进行深度优先遍历。
  2. 当一个顶点的所有邻接顶点都被访问后，将该顶点压入栈中。
  3. 最终，逆序弹出栈中的顶点，即为拓扑排序结果。

- **特点**：排序结果可能不唯一（取决于遍历起点和顺序）。


### 示例说明
假设有如下依赖关系图：
```
A → B → D
↓       ↑
C ──────┘
```
- **入度分析**：
  - A 的入度为 0（无前驱）。
  - B 的入度为 1（依赖 A）。
  - C 的入度为 0（无前驱）。
  - D 的入度为 2（依赖 B 和 C）。

- **拓扑排序可能的结果**：
  - `[A, C, B, D]`（先处理 A 和 C，再处理 B，最后处理 D）。
  - `[C, A, B, D]`（先处理 C 和 A，再处理 B，最后处理 D）。


### 关键性质
1. **有向无环图（DAG）**：只有 DAG 才能进行拓扑排序。如果图中存在环，则无法完成排序（因为环中的顶点相互依赖，形成循环）。
2. **结果不唯一**：如果图中存在多个入度为 0 的顶点，排序结果可能不唯一。例如，上述示例中 A 和 C 的顺序可以互换。
3. **应用场景**：常用于任务调度、编译顺序、课程依赖等需要处理依赖关系的场景。


### 代码中的拓扑排序
在你的 Rust 代码中，`toposort` 函数实现了 Kahn 算法：
```rust
use petgraph::algo::toposort;
use petgraph::Graph;

fn topological_sort_example() {
    let mut graph: Graph<&str, (), petgraph::Directed> = Graph::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    
    graph.add_edge(a, b, ()); // A → B
    graph.add_edge(a, c, ()); // A → C
    graph.add_edge(b, d, ()); // B → D
    graph.add_edge(c, d, ()); // C → D
    
    match toposort(&graph, None) {
        Ok(order) => println!("排序结果: {:?}", order.iter().map(|n| graph[*n]).collect::<Vec<_>>()),
        Err(_) => println!("图中存在环!"),
    }
}
```
- **输出结果**：可能为 `[A, B, C, D]` 或 `[A, C, B, D]`，具体取决于内部队列的处理顺序。


### 总结
拓扑排序的核心是 **尊重图中的依赖关系**，确保每个顶点在其所有前置依赖之后出现。排序结果的具体顺序可能因算法实现或顶点处理顺序而异，但必须满足依赖关系的约束。



```
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


拓扑排序结果:
Device { name: "Router", ip: "192.168.1.1", device_type: "router" }
Device { name: "Switch", ip: "192.168.1.2", device_type: "switch" }
Device { name: "IoT", ip: "192.168.1.5", device_type: "iot" }
Device { name: "PC", ip: "192.168.1.4", device_type: "pc" }
Device { name: "Server", ip: "192.168.1.3", device_type: "server" }
```



# 3、加权图最短路径

| 算法           | 权重要求   | 适用场景                     | 时间复杂度                    |
| -------------- | ---------- | ---------------------------- | ----------------------------- |
| Dijkstra       | 非负权重   | 单源最短路径                 | O((V+E) log V)                |
| Bellman-Ford   | 允许负权重 | 单源最短路径，检测负权环     | O(V*E)                        |
| Floyd-Warshall | 允许负权重 | 所有节点对最短路径           | O(V³)                         |
| Johnson        | 允许负权重 | 所有节点对最短路径（稀疏图） | O(V*E + V² log V)             |
| A*             | 非负权重   | 单对最短路径（有启发式）     | 优于 Dijkstra（取决于启发式） |





### 总结

- **单源最短路径**：优先使用 Dijkstra（非负权重）或 Bellman-Ford（允许负权重）。
- **所有节点对**：Floyd-Warshall（密集图）或 Johnson（稀疏图）。
- **已知终点且有启发式**：A* 算法。

### 启发式函数的影响

- **精确启发式**：若启发式函数总能返回真实代价（如在 DAG 中已知确切路径），A* 会直接沿最优路径搜索，时间复杂度接近 O (E)。
- **零启发式**：若启发式函数恒为 0，A* 退化为 Dijkstra 算法。
- **高估启发式**：若启发式函数可能高估代价，A* 可能找不到最优解，但搜索速度可能更快。



```
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

从 Router 到 Server 的最短距离: 11
路径:
  0: "Router"
  1: "Switch"
  2: "Server"
经过的边:
  Router -> Switch (权重: 10)
  Switch -> Server (权重: 1)

```





1. **曼哈顿距离（Manhattan Distance）**：
   - 适用于网格图，只能上下左右移动。
   - 公式：`|x1 - x2| + |y1 - y2|`
2. **欧几里得距离（Euclidean Distance）**：
   - 适用于可沿任意方向移动的场景。
   - 公式：`√((x1 - x2)² + (y1 - y2)²)`
3. **对角线距离（Diagonal Distance）**：
   - 适用于网格图，允许对角线移动。

在上述代码中：

定义了 `manhattan_distance` 函数来计算两个设备之间的曼哈顿距离。

1. 在调用 `astar` 算法时，使用 `|node| manhattan_distance(&graph[*node], &graph[target_node])` 作为启发式函数，这样 A* 算法会根据曼哈顿距离来优先探索可能的路径，而不是像 Dijkstra 算法那样盲目搜索。



# 4、无加权最短路径算法



1. **BFS 算法**：
   - 逐层遍历图，确保找到边数最少的路径
   - 使用队列和前驱节点记录路径信息
   - 时间复杂度：O (V + E)
2. **双向 BFS 算法**：
   - 同时从起点和终点开始 BFS，相遇时合并路径
   - 大幅减少搜索空间，适用于大规模图
   - 时间复杂度：O (V^(d/2))，其中 d 是最短路径长度
3. **简化 Dijkstra 算法**：
   - 使用 Petgraph 的 Dijkstra 实现，所有边权重设为 1
   - 适用于兼容带权图的场景
   - 时间复杂度：O ((V + E) log V)



以下是 BFS、双向 BFS 和 Dijkstra 算法在无权图中寻找最短路径的复杂度分析及适用场景：

### 1. **BFS（广度优先搜索）**
- **时间复杂度**：$O(V + E)$  
  - $V$ 是节点数，$E$ 是边数。每个节点和边仅被访问一次。
- **空间复杂度**：$O(V)$  
  - 主要用于队列和已访问集合，最坏情况下需存储所有节点。
- **适用场景**：
  - **无权图**的单源最短路径问题。
  - 当目标节点较近或平均路径长度较短时效率高。
  - 实现简单，适合快速开发。

### 2. **双向 BFS（Bidirectional BFS）**
- **时间复杂度**：$O(V^{d/2})$（优化后）  
  - $d$ 是最短路径长度。相比 BFS 的 $O(V^d)$ 有显著提升。
- **空间复杂度**：$O(V^{d/2})$  
  - 需维护两个队列和访问集合，空间需求与时间复杂度相似。
- **适用场景**：
  - **无权图**中已知起点和终点的最短路径问题。
  - 当图的分支因子较大且路径较长时，双向 BFS 比 BFS 快指数级。
  - 例如：社交网络中的用户关系路径查找。

### 3. **Dijkstra 算法（无权图简化版）**
- **时间复杂度**：$O((V + E) \log V)$  
  - 使用优先队列（二叉堆）实现。在无权图中，可简化为 $O(V + E)$（类似 BFS）。
- **空间复杂度**：$O(V)$  
  - 用于距离数组和优先队列。
- **适用场景**：
  - **带权图**的单源最短路径问题（原算法支持非负权重）。
  - 在无权图中，简化版 Dijkstra 退化为 BFS，无性能优势。
  - 当图的边权重不同且均为非负时，Dijkstra 是首选。

### 对比总结

| 算法         | 时间复杂度（无权图） | 空间复杂度   | 适用场景                                     |
| ------------ | -------------------- | ------------ | -------------------------------------------- |
| **BFS**      | $O(V + E)$           | $O(V)$       | 无权图，起点到终点路径查找，实现简单         |
| **双向 BFS** | $O(V^{d/2})$         | $O(V^{d/2})$ | 无权图，已知起点和终点，路径较长且分支因子大 |
| **Dijkstra** | $O((V + E) \log V)$  | $O(V)$       | 带权图（非负权重），若用于无权图则退化为 BFS |

### 选择建议

1. **优先用双向 BFS**：  
   - 当无权图中已知起点和终点，且路径可能较长时（如社交网络、迷宫）。

2. **其次用 BFS**：  
   - 当只知道起点或终点，或路径较短，或实现简单性更重要时。

3. **仅在带权图用 Dijkstra**：  
   - 若图的边有权重且非负，必须使用 Dijkstra 或其他带权图算法（如 A*）。

### 代码实现选择

- **BFS**：适合快速实现，无需优先队列。
- **双向 BFS**：需维护两个队列和访问集合，实现复杂度较高但性能显著提升。
- **Dijkstra**：在无权图中无需使用，除非代码需同时支持带权图场景。



## 4.1 Dijkstra 算法

```
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
    graph.add_edge(pc, server, GraphEdge{
        from: 1,
        to: 2,
        weight: 2,
    });

    // 计算从 Router 到 Server 的最短路径
    let start_node = router;
    let target_node = server;
    
    // 3. 简化Dijkstra 算法
    println!("\n=== 简化Dijkstra 算法 ===");
    if let Some((nodes, edges)) = shortest_path_dijkstra(&graph, start_node, target_node) {
        println!("路径长度: {}", nodes.len() - 1);
        println!("路径节点:");
        for (i, node) in nodes.iter().enumerate() {
            println!("  {}: {}", i, graph[*node].name);
        }
        println!("路径边:");
        for (i, edge) in edges.iter().enumerate() {
            println!("  {}: 权重={}", i, edge.weight);
        }
    } else {
        println!("无路径");
    }
}

=== 简化Dijkstra 算法 ===
路径长度: 2
路径节点:
  0: Router
  1: Switch
  2: Server
路径边:
  0: 权重=10
  1: 权重=1


```



## 4.2 **BFS 算法**

```
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

```

## 4.2 双向BFS

双向广度优先搜索（Bidirectional BFS）是对普通广度优先搜索（BFS）的优化，它从起点和终点同时开始进行搜索，当两个搜索相遇时，就找到了最短路径。双向 BFS 在大规模图中通常比普通 BFS 更快，因为它减少了搜索空间。

```
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


```



# 5、有向图的强连通分量算法

强连通分量（SCC）算法用于将有向图分解为多个强连通子图，每个子图内部的任意两个顶点都相互可达。

```
use petgraph::algo::kosaraju_scc;
use petgraph::Graph;

fn strong_connected_components_example() {
    // 创建一个有向图
    let mut graph: Graph<&str, (), petgraph::Directed> = Graph::new();
    
    // 添加节点
    let node_a = graph.add_node("A");
    let node_b = graph.add_node("B");
    let node_c = graph.add_node("C");
    let node_d = graph.add_node("D");
    let node_e = graph.add_node("E");
    let node_f = graph.add_node("F");
    
    // 添加有向边，形成强连通分量
    graph.add_edge(node_a, node_b, ());
    graph.add_edge(node_b, node_c, ());
    graph.add_edge(node_c, node_a, ()); // A, B, C 形成一个强连通分量
    
    graph.add_edge(node_d, node_e, ());
    graph.add_edge(node_e, node_f, ());
    graph.add_edge(node_f, node_d, ()); // D, E, F 形成一个强连通分量
    
    // 添加一个连接两个强连通分量的边
    graph.add_edge(node_c, node_d, ());
    
    // 计算强连通分量
    let scc = kosaraju_scc(&graph);
    
    // 输出每个强连通分量
    println!("强连通分量数量: {}", scc.len());
    for (i, component) in scc.iter().enumerate() {
        println!("强连通分量 {}: {:?}", i + 1, component.iter().map(|n| graph[*n]).collect::<Vec<_>>());
    }
    
    // 检查两个节点是否在同一个强连通分量中
    let node_in_same_component = |n1, n2| {
        scc.iter().any(|component| component.contains(&n1) && component.contains(&n2))
    };
    
    println!("节点 A 和 B 是否在同一个强连通分量中: {}", 
             node_in_same_component(node_a, node_b));
    println!("节点 A 和 D 是否在同一个强连通分量中: {}", 
             node_in_same_component(node_a, node_d));
}
```







# 6、无向图的连通分量算法

连通分量算法用于将无向图分解为多个连通子图，每个子图内部的任意两个顶点都可以通过边相互到达。

```
use petgraph::algo::connected_components;
use petgraph::Graph;

fn connected_components_example() {
    // 创建一个无向图
    let mut graph: Graph<&str, (), petgraph::Undirected> = Graph::new();
    
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
    let num_components = connected_components(&graph);
    
    // 输出连通分量数量
    println!("连通分量数量: {}", num_components);
    
    // 你可以使用更高级的方法来获取每个连通分量的节点集合
    // 这里仅展示连通分量的数量
}
```

