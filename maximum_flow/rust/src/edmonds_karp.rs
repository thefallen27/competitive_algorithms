use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    to: usize,
    flow: i32,
    capacity: i32,
    reverse_edge_index: usize,
}

#[derive(Clone, Debug)]
pub struct MaxFlow {
    pub graph: Vec<Vec<Edge>>,
    vertices: usize,
}

impl MaxFlow {
    pub fn new(vertices: usize) -> Self {
        MaxFlow {
            graph: vec![vec![]; vertices],
            vertices,
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, capacity: usize) {
        let index = self.graph[to].len();
        self.graph[from].push(Edge {
            to,
            flow: 0,
            capacity: capacity as i32,
            reverse_edge_index: index,
        });

        let index = self.graph[from].len() - 1;
        self.graph[to].push({
            Edge {
                to: from,
                flow: 0,
                capacity: 0,
                reverse_edge_index: index,
            }
        });
    }

    pub fn edmonds_karp(&mut self, source: usize, sink: usize) -> i32 {
        let mut maximum_flow = 0;

        let mut flow = self.augmenting_path(source, sink);
        while flow > 0 {
            maximum_flow += flow;

            flow = self.augmenting_path(source, sink);
        }

        maximum_flow
    }

    fn augmenting_path(&mut self, source: usize, sink: usize) -> i32 {
        let mut parent: Vec<usize> = vec![0; self.vertices];
        let mut minimum_capacity = vec![std::i32::MAX; self.vertices];

        let mut q: VecDeque<_> = VecDeque::new();
        q.push_back(source);
        parent[source] = source;

        while let Some(current) = q.pop_front() {
            for edge in &self.graph[current] {
                if parent[edge.to] == 0 && edge.capacity > edge.flow {
                    parent[edge.to] = current;

                    println!("{}, {}", edge.capacity, edge.flow);
                    minimum_capacity[edge.to] =
                        std::cmp::min(minimum_capacity[current], edge.capacity - edge.flow);

                    if edge.to == sink {
                        let path_capacity = minimum_capacity[sink];
                        let mut node = sink;

                        while node != source {
                            let previous_node = parent[node];
                            let index = self.graph[node][0].reverse_edge_index;
                            self.graph[previous_node][index].flow -= path_capacity;
                            self.graph[node][0].flow += path_capacity;
                            node = previous_node;
                        }

                        return path_capacity;
                    }

                    q.push_back(edge.to);
                }
            }
        }

        0
    }
}
