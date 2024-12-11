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

    pub fn solve() -> (u64, u64) {
        let input = read_file_contents("./input/day_eleven.txt")
            .split_whitespace() // Split by whitespace
            .filter_map(|num| num.parse::<u64>().ok()) // Parse each number
            .collect::<Vec<u64>>();
        let part_one = count(&input, 25);
        let part_two = count(&input, 75);

        (part_one, part_two)
    }

    fn count(input: &[u64], blinks: usize) -> u64 {
        println!("Starting count with input: {:?}, blinks: {}", input, blinks);

        let mut stones = Vec::new();
        let mut indices = HashMap::new();
        let mut todo = Vec::new();
        let mut current = Vec::new();

        for &number in input {
            if let Some(&index) = indices.get(&number) {
                current[index] += 1;
            } else {
                indices.insert(number, indices.len());
                todo.push(number);
                current.push(1);
            }
        }
        for _ in 0..blinks {
            let numbers = todo.clone();
            todo.clear();

            let mut index_of = |number: u64| {
                let size = indices.len();
                *indices.entry(number).or_insert_with(|| {
                    todo.push(number);
                    size
                })
            };

            for number in numbers {
                let (first, second) = if number == 0 {
                    (index_of(1), usize::MAX)
                } else {
                    let digits = number.ilog10() + 1;
                    if digits % 2 == 0 {
                        let power = 10_u64.pow(digits / 2);
                        (index_of(number / power), index_of(number % power))
                    } else {
                        (index_of(number * 2024), usize::MAX)
                    }
                };

                stones.push((first, second));
            }

            let mut next = vec![0; indices.len()];

            for (&(first, second), &amount) in stones.iter().zip(&current) {
                next[first] += amount;
                if second != usize::MAX {
                    next[second] += amount;
                }
            }

            current = next;
        }

        let result = current.iter().sum();
        result
    }
}
