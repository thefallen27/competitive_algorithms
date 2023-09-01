use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct MaxFlow {
    pub graph: HashMap<i32, HashMap<i32, i32>>,
    pub residual: HashMap<i32, HashMap<i32, i32>>,
}

impl MaxFlow {
    pub fn new() -> Self {
        MaxFlow {
            graph: HashMap::new(),
            residual: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: i32, to: i32, capacity: i32) {
        if let std::collections::hash_map::Entry::Vacant(e) = self.graph.entry(from) {
            e.insert(HashMap::new());
            self.graph.get_mut(&from).unwrap().insert(to, capacity);
        } else {
            self.graph.get_mut(&from).unwrap().insert(to, capacity);
        }
    }

    pub fn ford_fulkerson(&mut self, source: i32, target: i32) -> i32 {
        self.residual = self.graph.clone();

        let mut max_flow = 0;

        loop {
            let mut visited = HashSet::new();
            let augmenting_path_capacity = self.augmenting_path(source, target, &mut visited);

            if augmenting_path_capacity == 0 {
                break;
            }

            max_flow += augmenting_path_capacity;

            let mut node = target;
            while node != source {
                let prev_node = visited
                    .iter()
                    .find(|&&n| self.residual[&n].contains_key(&node))
                    .unwrap();

                *self
                    .residual
                    .get_mut(prev_node)
                    .unwrap()
                    .get_mut(&node)
                    .unwrap() -= augmenting_path_capacity;

                if self.residual.contains_key(prev_node)
                    && self
                        .residual
                        .get_mut(prev_node)
                        .unwrap()
                        .contains_key(&node)
                {
                    *self
                        .residual
                        .get_mut(prev_node)
                        .unwrap()
                        .get_mut(&node)
                        .unwrap() += augmenting_path_capacity;
                }

                node = *prev_node;
            }
        }

        max_flow
    }

    fn augmenting_path(&self, node: i32, target: i32, visited: &mut HashSet<i32>) -> i32 {
        if node == target {
            return std::i32::MAX;
        }
        visited.insert(node);

        if let Some(neighbors) = self.graph.get(&node) {
            for (&neighbor, &capacity) in neighbors.iter() {
                if !visited.contains(&neighbor) && capacity > 0 {
                    let min_capacity =
                        std::cmp::min(capacity, self.augmenting_path(neighbor, target, visited));

                    if min_capacity > 0 {
                        return min_capacity;
                    }
                }
            }
        }

        0
    }
}
