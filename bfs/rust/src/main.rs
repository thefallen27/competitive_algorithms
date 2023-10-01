use std::{
    collections::{HashMap, VecDeque},
    fs,
    path::Path,
};

fn bfs(start_node: usize, end_node: usize, graph: &[Vec<(usize, usize)>]) -> (Vec<usize>, usize) {
    let mut visited = HashMap::new();

    let mut distances = vec![usize::MAX; graph.len()];
    distances[start_node] = 0;

    let mut previous = vec![None; graph.len()];

    let mut queue = VecDeque::new();
    queue.push_back(start_node);

    while let Some(current_node) = queue.pop_front() {
        for (destination, weight) in &graph[current_node] {
            if !visited.contains_key(destination) {
                visited.insert(*destination, ());
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
    let input = fs::read_to_string(file).expect("Failed to read the input file");

    let (nodes, _edges) = input.lines().next().unwrap().split_once(' ').unwrap();

    let nodes = nodes.parse::<usize>().unwrap();

    let graph: Vec<Vec<(usize, usize)>> = input
        .lines()
        .skip(1)
        .map(|line| {
            let mut line_split = line.split(' ');
            let source = line_split.next().unwrap().parse::<usize>().unwrap();
            let destination = line_split.next().unwrap().parse::<usize>().unwrap();
            let weight = line_split.next().unwrap().parse::<usize>().unwrap();
            (source, destination, weight)
        })
        .fold(
            vec![vec![]; nodes],
            |mut acc, (source, destination, weight)| {
                acc[source].push((destination, weight));
                acc
            },
        );

    for &(start, end) in &[
        (0, 99),
        (8, 66),
        (4, 88),
        (12, 64),
        (7, 77),
        (1, 23),
        (5, 78),
    ] {
        println!("Starting node: {}", start);
        println!("End node: {}", end);
        let (shortest_path, shortest_distance) = bfs(start, end, &graph);

        print!("Shortest path: ");
        for node in &shortest_path {
            print!("{:?} ", node);
        }

        println!();
        println!("Shortest distance: {}\n", shortest_distance);
    }
}
