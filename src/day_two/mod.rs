use std::fs;

pub mod solution {
    use super::*;

    fn read_file_contents(file_path: &str) -> String {
        fs::read_to_string(file_path).unwrap_or_else(|_| {
            eprintln!("Failed to read the file: {}", file_path);
            String::new()
        })
    }

    fn safe(levels: &[u32]) -> bool {
        let mut up = true;
        let mut down = true;
        let mut range = true;

        for w in levels.windows(2) {
            up &= w[0] < w[1];
            down &= w[0] > w[1];
            range &= (1..=3).contains(&w[0].abs_diff(w[1]));
        }

        (up ^ down) && range
    }

    pub fn solve() -> (u32, u32) {
        let str = read_file_contents("./input/day_two.txt");
        let mut row = Vec::new();
        let mut part_one = 0;
        let mut part_two = 0;

        for line in str.lines() {
            row.extend(
                line.split_whitespace()
                    .filter_map(|s| s.parse::<u32>().ok()),
            );
            if safe(&row) {
                part_one += 1;
                part_two += 1;
            } else {
                for i in 0..row.len() {
                    let level = row.remove(i);

                    if safe(&row) {
                        part_two += 1;
                        break;
                    }

                    row.insert(i, level);
                }
            }

            row.clear();
        }

        (part_one, part_two)
    }
}
