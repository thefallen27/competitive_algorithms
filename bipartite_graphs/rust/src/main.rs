use std::collections::{HashMap, VecDeque};

type Graph = HashMap<usize, Vec<usize>>;

#[derive(Clone, PartialEq)]
enum Colour {
    Default,
    A,
    B,
}

fn is_bipartite(graph: &Graph, start: usize) -> bool {
    let mut colour_map = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back(start);
    colour_map.insert(start, Colour::Default);

    while let Some(node) = queue.pop_front() {
        let neighbour_colour = match colour_map[&node] {
            Colour::A => Colour::B,
            Colour::B => Colour::A,
            Colour::Default => Colour::A,
        };

        if let Some(neighbours) = graph.get(&node) {
            for &neighbour in neighbours {
                if let std::collections::hash_map::Entry::Vacant(e) = colour_map.entry(neighbour) {
                    e.insert(neighbour_colour.clone());
                    queue.push_back(neighbour);
                } else if colour_map[&neighbour] == colour_map[&node] {
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    let file_content =
        std::fs::read_to_string("../bipartite_graph_input_3.txt").expect("Unable to read file");

    let mut lines = file_content.lines();

    let res = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let num_nodes = res[0];
    let _num_edges = res[1];

    let mut graph: Graph = Default::default();
    for edge_info in lines {
        let edge_info: Vec<_> = edge_info
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let from = edge_info[0];
        let to = edge_info[1];

        graph.entry(from).or_insert(Vec::new()).push(to);
    }

    let mut bipartite = true;
    let colour = vec![Colour::Default; num_nodes];
    for (i, item) in colour.iter().enumerate() {
        if *item == Colour::Default && !is_bipartite(&graph, i) {
            bipartite = false;
            break;
        }
    }

    if bipartite {
        println!("The graph is bipartite");
    } else {
        println!("The graph is not bipartite");
    }
}
