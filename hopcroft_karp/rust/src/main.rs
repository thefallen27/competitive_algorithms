use std::collections::{HashMap, VecDeque};
use std::fs;

const INF: usize = usize::MAX;

struct HopcroftKarp {
    u: usize,
    adjacency: HashMap<usize, Vec<usize>>,
    pair_matching_from_u: Vec<isize>,
    pair_matching_from_v: Vec<isize>,
    distance_from_free_nodes: Vec<usize>,
}

impl HopcroftKarp {
    fn new(set_u: usize, set_v: usize) -> Self {
        HopcroftKarp {
            u: set_u,
            adjacency: HashMap::new(),
            pair_matching_from_u: vec![-1; set_u],
            pair_matching_from_v: vec![-1; set_v],
            distance_from_free_nodes: vec![0; set_u + 1],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.adjacency.entry(u).or_insert(Vec::new()).push(v);
    }

    fn maximum_matching(&mut self) -> usize {
        let mut matching = 0;
        while self.bfs() {
            for u in 0..self.u {
                if self.pair_matching_from_u[u] == -1 && self.dfs(u) {
                    matching += 1;
                }
            }
        }

        matching
    }

    fn bfs(&mut self) -> bool {
        let mut q = VecDeque::new();
        for u in 0..self.u {
            if self.pair_matching_from_u[u] == -1 {
                self.distance_from_free_nodes[u] = 0;
                q.push_back(u);
            } else {
                self.distance_from_free_nodes[u] = INF;
            }
        }

        self.distance_from_free_nodes[self.u] = INF;

        while let Some(u) = q.pop_front() {
            if self.distance_from_free_nodes[u] < self.distance_from_free_nodes[self.u] {
                if let Some(neighbors) = self.adjacency.get(&u) {
                    for &v in neighbors {
                        if self.pair_matching_from_v[v] == -1
                            || self.distance_from_free_nodes[self.pair_matching_from_v[v] as usize]
                                == INF
                        {
                            let next_index = if self.pair_matching_from_v[v] == -1 {
                                self.u
                            } else {
                                self.pair_matching_from_v[v] as usize
                            };
                            self.distance_from_free_nodes[next_index] =
                                self.distance_from_free_nodes[u] + 1;
                            q.push_back(next_index);
                        }
                    }
                }
            }
        }

        self.distance_from_free_nodes[self.u] != INF
    }

    fn dfs(&mut self, u: usize) -> bool {
        if u == usize::MAX {
            return true;
        }

        if let Some(neighbors) = self.adjacency.clone().get(&u) {
            for &v in neighbors {
                if self.pair_matching_from_v[v] == -1
                    || (self.distance_from_free_nodes[self.pair_matching_from_v[v] as usize]
                        == self.distance_from_free_nodes[u] + 1
                        && self.dfs(self.pair_matching_from_v[v] as usize))
                {
                    self.pair_matching_from_v[v] = u as isize;
                    self.pair_matching_from_u[u] = v as isize;
                    return true;
                }
            }
        }

        self.distance_from_free_nodes[u] = INF;
        false
    }
}

fn main() {
    let file_content =
        fs::read_to_string("../hopcroft_karp_input.txt").expect("Unable to read file");

    let mut lines = file_content.lines();

    let res: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let u = res[0];
    let v = res[1];

    let mut hk = HopcroftKarp::new(u, v);

    for data in lines {
        let data_info: Vec<usize> = data
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let from = data_info[0];
        let to = data_info[1];

        hk.add_edge(from, to);
    }

    println!("Maximum matching: {}", hk.maximum_matching());
}
