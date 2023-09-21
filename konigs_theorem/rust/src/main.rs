use std::io::{self, BufRead};

type Coordinates = (f64, f64);

fn dfs(
    adjacency_list: &Vec<Vec<usize>>,
    u_set: &mut Vec<Option<usize>>,
    v_set: &mut Vec<Option<usize>>,
    visited_nodes: &mut Vec<bool>,
    current: usize,
) -> bool {
    if visited_nodes[current] {
        return false;
    }

    visited_nodes[current] = true;

    for &neighbour in &adjacency_list[current] {
        if v_set[neighbour].is_none()
            || dfs(
                adjacency_list,
                u_set,
                v_set,
                visited_nodes,
                v_set[neighbour].unwrap(),
            )
        {
            u_set[current] = Some(neighbour);
            v_set[neighbour] = Some(current);
            return true;
        }
    }

    false
}

fn maximum_matching(adjacency_list: &Vec<Vec<usize>>, u_nodes: usize, v_nodes: usize) -> i64 {
    let mut u_set = vec![None; u_nodes];
    let mut v_set = vec![None; v_nodes];
    let mut visited = vec![false; u_nodes];
    let mut match_found = true;

    while match_found {
        match_found = false;
        for i in 0..u_nodes {
            if u_set[i].is_none() {
                match_found |= dfs(adjacency_list, &mut u_set, &mut v_set, &mut visited, i);
            }
        }
    }

    let result = u_set.iter().filter(|&&value| value.is_some()).count();
    if result < 1 {
        return -1;
    }

    result as i64
}

fn bfs(midway: f64, distance: &[Vec<f64>], n: usize, r: usize, b: usize) -> bool {
    let mut adjacency_list: Vec<Vec<usize>> = vec![vec![]; r];
    for i in 0..r {
        for j in 0..b {
            if distance[i][j] < midway {
                adjacency_list[i].push(j);
            }
        }
    }

    (r + b) as i64 - maximum_matching(&adjacency_list, r, b) >= n as i64
}

fn euclidean_distance(p1: Coordinates, p2: Coordinates) -> f64 {
    let d1 = (p1.0 - p2.0).powi(2);
    let d2 = (p1.1 - p2.1).powi(2);

    (d1 + d2).sqrt()
}

fn read_coordinates(len: usize) -> Vec<Coordinates> {
    let mut line = String::new();
    let stdin = io::stdin();
    let mut input = stdin.lock();

    let mut coordinates: Vec<Coordinates> = vec![(0.0, 0.0); len];

    for coord in coordinates.iter_mut().take(len) {
        line.clear();
        input.read_line(&mut line).unwrap();
        let mut coords = line.split_whitespace().map(|x| x.parse::<f64>().unwrap());
        let x: f64 = coords.next().unwrap();
        let y: f64 = coords.next().unwrap();
        *coord = (x, y);
    }

    coordinates
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut words = input.split_whitespace();
    let n: usize = words.next().unwrap().parse().unwrap();
    let r: usize = words.next().unwrap().parse().unwrap();
    let b: usize = words.next().unwrap().parse().unwrap();

    let redberry = read_coordinates(r);
    let blueberry = read_coordinates(b);

    let mut distances: Vec<Vec<f64>> = vec![vec![0.0; b]; r];
    for i in 0..r {
        for (j, &berry) in blueberry.iter().enumerate().take(b) {
            distances[i][j] = euclidean_distance(redberry[i], berry);
        }
    }

    let epsilon = 1e-9;
    let mut low: f64 = 0.0;
    let mut high: f64 = 1e19;

    while (high - low) > epsilon {
        let midway = (high + low) / 2.0;
        if bfs(midway, &distances, n, r, b) {
            low = midway;
        } else {
            high = midway;
        }
    }

    println!("{:.8}", low);
}
