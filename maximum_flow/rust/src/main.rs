mod dinic;
mod edmonds_karp;
mod ford_fulkerson;

fn main() {
    let file_content =
        std::fs::read_to_string("../maximum_flow_input.txt").expect("Unable to read file");
    let mut lines = file_content.lines();

    let res = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let _num_nodes = res[0];
    let num_edges = res[1];

    let mut dinic = dinic::MaxFlow::new();
    let mut edmond = edmonds_karp::MaxFlow::new();
    let mut fulkerson = ford_fulkerson::MaxFlow::new();

    for _ in 0..num_edges {
        let edge_info: Vec<i32> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let from = edge_info[0];
        let to = edge_info[1];
        let capacity = edge_info[2];

        dinic.add_edge(from, to, capacity);
        edmond.add_edge(from, to, capacity);
        fulkerson.add_edge(from, to, capacity);
    }

    for pair in [(0, 99), (5, 88), (90, 3)] {
        println!(
            "Maximum flow with dinic algorithm:\n start: {}, sink: {}, result: {}",
            pair.0,
            pair.1,
            dinic.clone().dinic(pair.0, pair.1)
        );
    }

    let pair = (5, 10);
    println!(
        "Maximum flow with edmond_karps algorithm:\n start: {}, sink: {}, result: {}",
        pair.0,
        pair.1,
        edmond.clone().edmonds_karp(pair.0, pair.1)
    );

    println!(
        "Maximum flow with ford_fulkerson algorithm:\n start: {}, sink: {}, result: {}",
        pair.0,
        pair.1,
        fulkerson.clone().ford_fulkerson(pair.0, pair.1)
    );
}
