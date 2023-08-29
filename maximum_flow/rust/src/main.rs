mod dinic;

fn main() {
    let file_content =
        std::fs::read_to_string("../maximum_flow_input.txt").expect("Unable to read file");
    let mut lines = file_content.lines();

    let res = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let num_nodes = res[0];
    let num_edges = res[1];
    let mut dinic = dinic::MaxFlow::new(num_nodes);

    for _ in 0..num_edges {
        let edge_info: Vec<usize> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let from = edge_info[0];
        let to = edge_info[1];
        let capacity = edge_info[2];

        dinic.add_edge(from, to, capacity.try_into().unwrap());
    }

    for pair in [(0, 99), (5, 88), (90, 3)] {
        println!("Maximum flow: {}", dinic.clone().dinic(pair.0, pair.1));
    }
}
