use std::{
    collections::{HashSet, VecDeque},
    fs,
    path::Path,
};

fn bfs(start_node: usize, end_node: usize, graph: &[Vec<(usize, usize)>]) -> (Vec<usize>, usize) {
    let mut visited = HashSet::new();

    let mut distances = vec![usize::MAX; graph.len()];
    distances[start_node] = 0;

    let mut previous = vec![None; graph.len()];

    let mut queue = VecDeque::new();
    queue.push_back(start_node);

    while let Some(current_node) = queue.pop_front() {
        for (destination, weight) in &graph[current_node] {
            if !visited.contains(&destination) {
                visited.insert(destination);
                queue.push_back(*destination);
            }

            if distances[current_node] + weight < distances[*destination] {
                distances[*destination] = distances[current_node] + weight;
                previous[*destination] = Some(current_node);
            }
        }
    }

    let mut path = vec![];
    let mut current = Some(end_node);
    while let Some(value) = current {
        path.push(value);
        current = previous[value];
    }

    path.reverse();
    (path, distances[end_node])
}

fn main() {
    let file = Path::new("../bfs_input.txt");
    let input = fs::read_to_string(file).expect("Failed to read file");

    let (nodes, _edges) = input.lines().next().unwrap().split_once(' ').unwrap();
    let nodes = nodes.parse::<usize>().unwrap();

    let mut graph = vec![vec![]; nodes];
    for i in input.lines().skip(1) {
        let mut line_split = i.split(' ');
        let source = line_split.next().unwrap().parse::<usize>().unwrap();
        let destination = line_split.next().unwrap().parse::<usize>().unwrap();
        let wheight = line_split.next().unwrap().parse::<usize>().unwrap();

        graph[source].push((destination, wheight));
    }

    for pair_node in [
        (0, 99),
        (8, 66),
        (4, 88),
        (12, 64),
        (7, 77),
        (1, 23),
        (5, 78),
    ]
    .iter()
    {
        println!("Starting node: {}", pair_node.0);
        println!("End node: {}", pair_node.1);
        let (shortest_path, shortest_distance) = bfs(pair_node.0, pair_node.1, &graph);

        print!("Shortest path: ");
        for node in shortest_path {
            print!("{node:?} ");
        }

        println!();
        println!("Shortest distance {shortest_distance}\n");
    }
}
