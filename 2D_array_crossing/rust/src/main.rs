use std::collections::hash_set::HashSet;
use std::collections::BinaryHeap;
use std::fs;
use std::path::Path;

#[derive(Default, Eq, PartialEq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn neighbours(&self, rows: usize, cols: usize) -> Vec<Self> {
        let mut neigh = Vec::new();

        if self.y < cols - 1 {
            neigh.push(Coord {
                x: self.x,
                y: self.y + 1,
            });
        }

        if self.x < rows - 1 {
            neigh.push(Coord {
                x: self.x + 1,
                y: self.y,
            });
        }

        neigh
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Node {
    cost: u32,
    coord: Coord,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Maze {
    start: Coord,
    end: Coord,
    map: Vec<Vec<u32>>,
    rows: usize,
    cols: usize,
}

fn shortest_path(maze: &Maze) -> Option<(Vec<Coord>, u32)> {
    let mut visited = HashSet::new();
    let mut possible_nodes = BinaryHeap::new();

    possible_nodes.push(Node {
        cost: maze.map[maze.start.x][maze.start.y],
        coord: maze.start,
    });
    visited.insert(maze.start);

    let mut path = vec![vec![Coord::default(); maze.end.y + 1]; maze.end.x + 1];

    while let Some(Node { cost, coord }) = possible_nodes.pop() {
        if coord == maze.end {
            let mut crossed_path = vec![];
            let mut current = maze.end;
            crossed_path.push(current);
            while current != maze.start {
                current = path[current.x][current.y];
                crossed_path.push(current);
            }
            crossed_path.reverse();

            return Some((crossed_path, cost));
        }

        let neigh = coord.neighbours(maze.rows, maze.cols);

        let possible_steps: Vec<_> = neigh
            .iter()
            .filter(|coord| {
                let cost = maze.map[coord.x][coord.y];
                cost != 0 && coord.x <= maze.end.x && coord.y <= maze.end.y
            })
            .collect();

        for step in possible_steps {
            if visited.insert(*step) {
                path[step.x][step.y] = Coord {
                    x: coord.x,
                    y: coord.y,
                };

                possible_nodes.push(Node {
                    cost: cost + maze.map[step.x][step.y],
                    coord: *step,
                })
            }
        }
    }

    None
}

fn main() {
    let file = Path::new("../../2D_array_crossing_input.txt");
    let input = fs::read_to_string(file).expect("Failed to read file");

    let input = input.replace(' ', "");
    let rows = input.lines().count() - 1;
    let cols = input.lines().last().unwrap().len();

    let start = Coord { x: 0, y: 0 };
    let end = Coord { x: 11, y: 19 };

    let mut map = vec![vec![0; cols]; rows];

    for (row, line) in input.trim().lines().skip(1).enumerate() {
        for (col, cost) in line.trim().chars().enumerate() {
            if cost.is_whitespace() {
                continue;
            }
            map[row][col] = cost.to_digit(10).unwrap();
        }
    }

    let maze = Maze {
        start,
        end,
        map,
        rows,
        cols,
    };

    if let Some(res) = shortest_path(&maze) {
        println!("Shortest path:");
        for coord in res.0 {
            println!("({},{})", coord.x, coord.y);
        }

        println!("Cost of crossing: {}", res.1);
    } else {
        println!("No valid path found");
    }
}
