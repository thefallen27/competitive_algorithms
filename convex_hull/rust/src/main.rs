use std::io::{self};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

fn orientation(p: Point, q: Point, r: Point) -> i32 {
    let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);

    match val {
        val if val == 0 => 0,
        val if val > 1 => 1,
        _ => 2,
    }
}

fn convex_hull(mut points: Vec<Point>) -> Vec<Point> {
    let n = points.len();
    if n < 3 {
        return points;
    }

    points.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));

    let mut hull = Vec::new();

    for p in &points {
        while hull.len() >= 2 && orientation(hull[hull.len() - 2], hull[hull.len() - 1], *p) != 2 {
            hull.pop();
        }
        hull.push(*p);
    }

    let upper_start = hull.len() + 1;
    for p in points.iter().rev() {
        while hull.len() >= upper_start
            && orientation(hull[hull.len() - 2], hull[hull.len() - 1], *p) != 2
        {
            hull.pop();
        }
        hull.push(*p);
    }

    hull.pop();

    hull
}

fn main() -> io::Result<()> {
    let file_content = std::fs::read_to_string("../convex_hull_input.txt")?;
    let mut lines = file_content.lines();

    if let Some(first_line) = lines.next() {
        let num_points = first_line.trim().parse::<usize>().unwrap();
        let points: Vec<Point> = lines
            .take(num_points)
            .filter_map(|line| {
                let mut coords = line.split_whitespace().map(|x| x.parse::<i32>().ok());
                match (coords.next()?, coords.next()?) {
                    (Some(x), Some(y)) => Some(Point::new(x, y)),
                    _ => None,
                }
            })
            .collect();

        let convex_hull_points = convex_hull(points);

        if convex_hull_points.is_empty() {
            eprintln!("Convex hull could not be formed.");
            return Ok(());
        }

        println!("Convex Hull points:");
        for p in &convex_hull_points {
            println!("{} {}", p.x, p.y);
        }
    }

    Ok(())
}
