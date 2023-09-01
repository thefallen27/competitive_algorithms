use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct MaxFlow {
    pub graph: HashMap<i32, HashMap<i32, i32>>,
}

impl MaxFlow {
    pub fn new() -> Self {
        MaxFlow {
            graph: HashMap::new(),
        }
    }
    pub fn add_edge(&mut self, from: i32, to: i32, capacity: i32) {
        self.graph
            .entry(from)
            .or_insert(HashMap::new())
            .insert(to, capacity);
        self.graph
            .entry(to)
            .or_insert(HashMap::new())
            .insert(from, 0);
    }

    pub fn dinic(&mut self, source: i32, target: i32) -> i32 {
        let mut level = vec![0; self.graph.len()];
        let mut max_flow = 0;

        while self.bfs(&mut level, source, target) {
            let mut flow = self.dfs(&level, source, target, std::i32::MAX);
            while flow > 0 {
                max_flow += flow;
                flow = self.dfs(&level, source, target, std::i32::MAX);
            }
        }

        max_flow
    }

    fn bfs(&self, level: &mut [i32], source: i32, target: i32) -> bool {
        level.iter_mut().for_each(|l| *l = -1);
        level[source as usize] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(source);

        while let Some(node) = queue.pop_front() {
            if node == target {
                return true;
            }

            if let Some(neighbors) = self.graph.get(&node) {
                for (&neighbor, &capacity) in neighbors.iter() {
                    if level[neighbor as usize] < 0 && capacity > 0 {
                        level[neighbor as usize] = level[node as usize] + 1;
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        false
    }

    fn dfs(&mut self, level: &[i32], current: i32, target: i32, flow: i32) -> i32 {
        if current == target {
            return flow;
        }

        if let Some(neighbors) = self.graph.get(&current) {
            let neighbors_clone = neighbors.clone();
            for (&neighbor, capacity) in neighbors_clone.iter() {
                if capacity > &mut 0 && level[neighbor as usize] == level[current as usize] + 1 {
                    let min_capacity = std::cmp::min(flow, *capacity);
                    let flow = self.dfs(level, neighbor, target, min_capacity);

                    if flow > 0 {
                        *self
                            .graph
                            .get_mut(&current)
                            .and_then(|n| n.get_mut(&neighbor))
                            .unwrap() -= flow;
                        *self
                            .graph
                            .get_mut(&neighbor)
                            .and_then(|n| n.get_mut(&current))
                            .unwrap() += flow;
                        return flow;
                    }
                }
            }
        }

        0
    }
}
