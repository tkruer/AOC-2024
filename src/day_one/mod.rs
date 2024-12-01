use std::collections::HashMap;
use std::fs;

pub mod solution {
    use super::*;

    fn read_file_contents(file_path: &str) -> String {
        fs::read_to_string(file_path).unwrap_or_else(|_| {
            eprintln!("Failed to read the file: {}", file_path);
            String::new()
        })
    }

    pub fn solve() -> (u32, i32) {
        let str: String = read_file_contents("./input/day_one.txt");
        let mut left: Vec<i32> = Vec::new();
        let mut right: Vec<i32> = Vec::new();
        let mut right_map: HashMap<i32, i32> = HashMap::new();

        for line in str.lines() {
            let (a, b) = line.split_once("   ").unwrap();
            let av: i32 = a.parse().unwrap();
            let bv: i32 = b.parse().unwrap();
            left.push(av);
            right.push(bv);
            *right_map.entry(bv).or_default() += 1;
        }

        left.sort();
        right.sort();

        let sol1: u32 = left
            .iter()
            .zip(right.iter())
            .map(|(&a, &b)| i32::abs_diff(a, b))
            .sum();

        let sol2: i32 = left
            .iter()
            .map(|&v| v * right_map.get(&v).cloned().unwrap_or_default())
            .sum();

        (sol1, sol2)
    }
}
