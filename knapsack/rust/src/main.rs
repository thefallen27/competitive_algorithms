use std::fs;
use std::path::Path;

#[derive(Debug)]
struct Item {
    weight: usize,
    value: usize,
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

fn main() {
    let file = Path::new("../input.txt");
    let input = fs::read_to_string(file).expect("Failed to read file");

    let items: Vec<(usize, Vec<Item>)> = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|block| block.split('\n').collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
        .iter()
        .map(|b| {
            let ks_weight = b.get(1).unwrap().parse().unwrap();
            let items = b
                .iter()
                .skip(2)
                .flat_map(|l| {
                    l.split_once(' ').map(|(first, second)| {
                        let value = first.parse().unwrap();
                        let weight = second.parse().unwrap();
                        Item { value, weight }
                    })
                })
                .collect();
            (ks_weight, items)
        })
        .collect();

    for (index, item) in items.iter().enumerate() {
        println!(
            "Maximum profit for section: {}: \n{}",
            index,
            knapsack(item.0, &item.1)
        );
    }
}
