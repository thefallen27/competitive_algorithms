use std::{fs, path::Path};

fn kmp(input: String, pattern: String) {
    let pattern = pattern.into_bytes();
    let mut lps = vec![0; pattern.len()];
    let mut j = 0;
    let mut i = 1;

    while i < pattern.len() {
        if pattern[i] == pattern[j] {
            j += 1;
            lps[i] = j;
            i += 1;
        } else if j != 0 {
            j = lps[j - 1];
        } else {
            lps[i] = 0;
            i += 1;
        }
    }

    for (l, line) in input.lines().enumerate() {
        let mut j = 0;
        let mut i = 0;
        let line = line.to_string().into_bytes();

        while i < line.len() {
            if pattern[j] == line[i] {
                i += 1;
                j += 1;
            }

            if j == pattern.len() {
                println!("Pattern found at line {}, index {}", l + 1, i - j);
                j = lps[j - 1];
            } else if i < line.len() && pattern[j] != line[i] {
                if j != 0 {
                    j = lps[j - 1];
                } else {
                    i += 1;
                }
            }
        }
    }
}

fn main() {
    let file = Path::new("../kmp_lorem_ipsum_input.txt");
    let input = fs::read_to_string(file).expect("Failed to read file");

    for pattern in ["ligula", "cursus", "eros"] {
        println!("Pattern to search: {pattern}");
        kmp(input.clone(), pattern.to_string());
    }
}
