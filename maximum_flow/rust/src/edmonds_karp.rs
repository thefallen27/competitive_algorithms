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

    pub fn edmonds_karp(&mut self, source: i32, target: i32) -> i32 {
        let mut max_flow = 0;

        let mut parent = HashMap::new();
        while self.bfs(&mut parent, source, target) {
            let mut path_flow = std::i32::MAX;
            let mut s = target;

            while s != source {
                let &prev = parent.get(&s).unwrap();
                path_flow = std::cmp::min(path_flow, self.graph[&prev][&s]);
                s = prev;
            }

            max_flow += path_flow;

            let mut v = target;
            while v != source {
                let &u = parent.get(&v).unwrap();
                *self.graph.get_mut(&u).unwrap().get_mut(&v).unwrap() -= path_flow;
                *self.graph.get_mut(&v).unwrap().get_mut(&u).unwrap() += path_flow;
                v = u;
            }
        }

        max_flow
    }

    fn bfs(&self, parent: &mut HashMap<i32, i32>, source: i32, target: i32) -> bool {
        let mut visited = HashMap::new();
        visited.insert(source, true);

        let mut queue = VecDeque::new();
        queue.push_back(source);

        while let Some(&node) = queue.front() {
            queue.pop_front();

            if node == target {
                return true;
            }

            if let Some(neighbors) = self.graph.get(&node) {
                for (&neighbor, &capacity) in neighbors.iter() {
                    if !visited.contains_key(&neighbor) && capacity > 0 {
                        parent.insert(neighbor, node);
                        visited.insert(neighbor, true);
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        false
    }
}
