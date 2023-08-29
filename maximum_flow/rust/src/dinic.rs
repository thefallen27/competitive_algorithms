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
    levels: Vec<i32>,
    next_edge_tracking: Vec<usize>,
}

impl MaxFlow {
    pub fn new(vertices: usize) -> Self {
        MaxFlow {
            graph: vec![vec![]; vertices],
            levels: vec![-1; vertices],
            next_edge_tracking: vec![0; vertices],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, capacity: i32) {
        let index = self.graph[to].len();
        self.graph[from].push(Edge {
            to,
            flow: 0,
            capacity,
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

    pub fn dinic(&mut self, source: usize, sink: usize) -> i32 {
        let mut maximum_flow = 0;

        while self.bfs(source, sink) {
            self.next_edge_tracking.fill(0);
            let mut flow = self.dfs(source, sink, std::i32::MAX);
            while flow > 0 {
                maximum_flow += flow;

                flow = self.dfs(source, sink, std::i32::MAX);
            }
        }

        maximum_flow
    }

    fn bfs(&mut self, source: usize, sink: usize) -> bool {
        self.levels.fill(-1);
        let mut q: VecDeque<_> = VecDeque::new();
        q.push_back(source);
        self.levels[source] = 0;

        while !q.is_empty() {
            let current = q.pop_front().unwrap();

            for edge in &self.graph[current] {
                if self.levels[edge.to] == -1 && edge.capacity > edge.flow {
                    self.levels[edge.to] = self.levels[current] + 1;
                    q.push_back(edge.to);
                }
            }
        }

        self.levels[sink] != -1
    }

    fn dfs(&mut self, current: usize, sink: usize, minimum_capacity: i32) -> i32 {
        if current == sink {
            return minimum_capacity;
        }

        for i in self.next_edge_tracking[current]..self.graph[current].len() {
            let edge = self.graph[current][i];

            if self.levels[edge.to] == self.levels[current] + 1 && edge.capacity > edge.flow {
                let flow = self.dfs(
                    edge.to,
                    sink,
                    std::cmp::min(minimum_capacity, edge.capacity - edge.flow),
                );

                if flow > 0 {
                    self.graph[current][i].flow += flow;
                    self.graph[edge.to][edge.reverse_edge_index].flow -= flow;
                    return flow;
                }
            }
        }

        0
    }
}
