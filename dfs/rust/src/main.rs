use std::{collections::HashSet, fs, path::Path};

fn dfs(start_node: usize, graph: &[Vec<usize>]) {
    let mut visited = HashSet::new();
    let mut st = Vec::new();
    st.push(start_node);

    while !st.is_empty() {
        let current_node = st.pop().unwrap();

        if !visited.contains(&current_node) {
            visited.insert(current_node);
            print!("{current_node} ");

            for neigh in graph[current_node].iter() {
                if !visited.contains(neigh) {
                    st.push(*neigh)
                }
            }
        }
    }
}

fn main() {
    let file = Path::new("../dfs_input.txt");
    let input = fs::read_to_string(file).expect("Failed to read file");

    let (nodes, _edges) = input.lines().next().unwrap().split_once(' ').unwrap();
    let nodes = nodes.parse::<usize>().unwrap();

    let mut graph: Vec<Vec<_>> = vec![vec![]; nodes];
    for i in input.lines().skip(1) {
        let (node, neighbor) = i.split_once(' ').unwrap();
        let node = node.parse::<usize>().unwrap();
        let neighbor = neighbor.parse::<usize>().unwrap();

        graph[node].push(neighbor);
        graph[neighbor].push(node);
    }
    println!("graph: {graph:?}");

    for start_node in [4, 5, 3, 2, 1, 0].iter() {
        println!("Starting node: {start_node}");
        print!("DFS traversal: ");
        dfs(*start_node, &graph);
        println!();
    }
}
