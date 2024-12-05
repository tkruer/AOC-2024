use std::collections::{HashMap, HashSet};
use std::fs;

pub mod solution {
    use super::*;

    fn read_file_contents(file_path: &str) -> String {
        fs::read_to_string(file_path).unwrap_or_else(|_| {
            eprintln!("Failed to read the file: {}", file_path);
            String::new()
        })
    }

    pub fn solve() -> (u32, u32) {
        let input = read_file_contents("./input/day_five.txt");
        let (ordering_input, pages_input) = input.split_once("\n\n").unwrap();

        let mut orderings: HashMap<usize, HashSet<usize>> = HashMap::new();

        for line in ordering_input.lines() {
            let (x, y) = line.split_once('|').unwrap();
            orderings
                .entry(y.parse().unwrap())
                .or_default()
                .insert(x.parse().unwrap());
        }

        let pages: Vec<Vec<usize>> = pages_input
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|word| word.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        let (mut part1_sum, mut part2_sum) = (0, 0);

        for mut page in pages {
            if is_sorted_by_custom(&page, &orderings) {
                part1_sum += middle_value(&page) as u32;
            } else {
                page.sort_by(|a, b| orderings[b].contains(a).cmp(&true));
                part2_sum += middle_value(&page) as u32;
            }
        }

        (part1_sum, part2_sum)
    }

    fn is_sorted_by_custom(page: &[usize], orderings: &HashMap<usize, HashSet<usize>>) -> bool {
        for window in page.windows(2) {
            if let [a, b] = window {
                if !orderings[b].contains(a) {
                    return false;
                }
            }
        }
        true
    }

    fn middle_value(page: &[usize]) -> usize {
        page[page.len() / 2]
    }
}
