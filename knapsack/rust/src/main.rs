use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
struct Item {
    weight: usize,
    value: usize,
}

impl FromStr for Item {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();
        let value = parts[0].parse()?;
        let weight = parts[1].parse()?;
        Ok(Item { value, weight })
    }
}

fn knapsack(ks_weight: usize, items: &[Item]) -> usize {
    items
        .iter()
        .enumerate()
        .filter_map(|(index, item)| {
            if item.weight > ks_weight {
                None
            } else {
                Some(knapsack(ks_weight - item.weight, &items[index + 1..]) + item.value)
            }
        })
        .max()
        .unwrap_or_default()
}

fn knapsack_dynamic(ks_weight: usize, items: &[Item]) -> usize {
    let mut results = vec![0; ks_weight + 1];

    for item in items {
        for weight in (item.weight..=ks_weight).rev() {
            if weight >= item.weight {
                results[weight] = results[weight].max(results[weight - item.weight] + item.value);
            }
        }
    }

    results[ks_weight]
}

fn main() {
    let file = Path::new("../knapsack_input.txt");

    if let Ok(input) = fs::read_to_string(file) {
        let items: Vec<(usize, Vec<Item>)> = input
            .split("\n\n")
            .map(|block| {
                let lines: Vec<&str> = block.lines().collect();
                let ks_weight = lines[1].parse().expect("Invalid weight");
                let items: Vec<Item> = lines[2..]
                    .iter()
                    .map(|line| line.parse().expect("Invalid item"))
                    .collect();
                (ks_weight, items)
            })
            .collect();

        for (index, item) in items.iter().enumerate() {
            println!(
                "Maximum profit with recursion for section: {}: \n{}",
                index,
                knapsack(item.0, &item.1)
            );
        }

        for (index, item) in items.iter().enumerate() {
            println!(
                "Maximum profit with dynamic programming for section: {}: \n{}",
                index,
                knapsack_dynamic(item.0, &item.1)
            );
        }
    } else {
        eprintln!("Failed to read file: {:?}", file);
    }
}
