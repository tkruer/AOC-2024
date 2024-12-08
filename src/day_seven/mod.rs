use std::fs;

pub mod solution {

    use super::*;

    fn read_file_contents(file_path: &str) -> String {
        fs::read_to_string(file_path).unwrap_or_else(|_| {
            eprintln!("Failed to read the file: {}", file_path);
            String::new()
        })
    }

    fn read_input(file_path: &str) -> (Vec<i64>, Vec<Vec<i64>>) {
        let data = read_file_contents(file_path);
        let mut totals: Vec<i64> = vec![];
        let mut all_terms: Vec<Vec<i64>> = vec![];

        for line in data.lines() {
            let mut fields = line.split_whitespace();
            let mut total = fields.next().unwrap().to_string();
            total.pop(); // remove the trailing colon
            totals.push(total.parse().unwrap());
            let terms: Vec<i64> = fields.map(|term| term.parse().unwrap()).collect();
            all_terms.push(terms);
        }

        (totals, all_terms)
    }

    fn generate_permutations(n: usize, ops: &[char]) -> Vec<Vec<char>> {
        let mut results = vec![];
        let mut current = vec![ops[0]; n];

        loop {
            results.push(current.clone());

            let mut i = n;
            while i > 0 {
                i -= 1;
                if let Some(next_index) = ops.iter().position(|&op| op == current[i]) {
                    if next_index + 1 < ops.len() {
                        current[i] = ops[next_index + 1];
                        break;
                    } else {
                        current[i] = ops[0];
                    }
                }
            }

            if i == 0 && current.iter().all(|&op| op == ops[0]) {
                break;
            }
        }

        results
    }

    fn check(total: i64, terms: &Vec<i64>, ops: &[char]) -> bool {
        let n_op = terms.len() - 1;
        let perms = generate_permutations(n_op, ops);

        for perm in perms {
            let mut nums = terms.clone().into_iter();
            let mut result = nums.next().unwrap();
            for op in perm {
                result = match op {
                    '+' => result + nums.next().unwrap(),
                    '*' => result * nums.next().unwrap(),
                    '|' => (result.to_string() + &nums.next().unwrap().to_string())
                        .parse::<i64>()
                        .unwrap(),
                    _ => panic!("Unsupported operator"),
                };
            }
            if result == total {
                return true;
            }
        }
        false
    }

    fn solve_part1(file_path: &str) -> i64 {
        let (totals, all_terms) = read_input(file_path);
        let mut sum_valid = 0;

        for (total, terms) in totals.into_iter().zip(all_terms.into_iter()) {
            if check(total, &terms, &['+', '*']) {
                sum_valid += total;
            }
        }

        sum_valid
    }

    fn solve_part2(file_path: &str) -> i64 {
        let (totals, all_terms) = read_input(file_path);
        let mut sum_valid = 0;

        for (total, terms) in totals.into_iter().zip(all_terms.into_iter()) {
            if check(total, &terms, &['+', '*', '|']) {
                sum_valid += total;
            }
        }

        sum_valid
    }

    pub fn solve() -> (i64, i64) {
        let part1_result = solve_part1("./input/day_seven.txt");
        let part2_result = solve_part2("./input/day_seven.txt");
        (part1_result, part2_result)
    }
}
