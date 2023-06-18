use std::{collections::BinaryHeap, fs, path::Path};

fn dijkstra(start_node: usize, graph: &[Vec<(usize, usize)>]) -> Vec<usize> {
    let mut distances = vec![usize::MAX; graph.len()];
    distances[start_node] = 0;

    let mut pr = BinaryHeap::new();
    pr.push((0, start_node));

    while let Some(node) = pr.pop() {
        if node.0 > distances[node.1] {
            continue;
        }

        for edge in &graph[node.1] {
            let v = edge.0;
            let weight = edge.1;

            if node.0 + weight < distances[v] {
                distances[v] = node.0 + weight;
                pr.push((distances[v], v));
            }
        }
    }

    distances
}

fn main() {
    let file = Path::new("../dijkstra_input.txt");
    let input = fs::read_to_string(file).expect("Failed to read file");

    let nodes = input.lines().next().unwrap().parse::<usize>().unwrap();

    let mut graph: Vec<Vec<_>> = vec![vec![]; nodes];
    for i in input.lines().skip(1) {
        let mut line_split = i.split(' ');
        let source = line_split.next().unwrap().parse::<usize>().unwrap();
        let destination = line_split.next().unwrap().parse::<usize>().unwrap();
        let weight = line_split.next().unwrap().parse::<usize>().unwrap();

        graph[source].push((destination, weight));
        // Uncomment below line if you want the graph to be undirected
        //graph[destination].push((source, wheight));
    }

    for start_node in [3, 15, 42] {
        println!("Starting node: {start_node}");
        let distances = dijkstra(start_node, &graph);
        for (node, value) in distances.iter().enumerate() {
            if distances[node] == usize::MAX {
                println!("No path from node {start_node} to node {node}");
            } else {
                println!("Node {node}: Distance = {value}");
            }
        }
    }
}
