use std::fs;

enum Direction {
    Horizontal,
    Vertical,
    DiagonalRight,
    DiagonalLeft,
}

pub mod solution {
    use super::*;

    fn read_file_contents(file_path: &str) -> String {
        fs::read_to_string(file_path).unwrap_or_else(|_| {
            eprintln!("Failed to read the file: {}", file_path);
            String::new()
        })
    }

    pub fn solve() -> (u32, u32) {
        let input = read_file_contents("./input/day_four.txt").to_owned();
        let xmas = "XMAS".to_string();
        let samx = "SAMX".to_string();
        let mas = "MAS".to_string();
        let sam = "SAM".to_string();
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let height = grid.len();
        let width = grid[0].len();
        let mut part_one_count = 0;
        let mut part_two_count = 0;

        for row in 0..height {
            for col in 0..width {
                // Part One Logic
                if grid[row][col] == 'X' || grid[row][col] == 'S' {
                    let can_h = col < width - 3;
                    let can_v = row < height - 3;
                    let can_nh = col >= 3;
                    if can_h {
                        let word = get_word(&grid, row, col, 4, Direction::Horizontal);
                        if word == xmas || word == samx {
                            part_one_count += 1;
                        }
                    }
                    if can_v {
                        let word = get_word(&grid, row, col, 4, Direction::Vertical);
                        if word == xmas || word == samx {
                            part_one_count += 1;
                        }
                    }
                    if can_v && can_h {
                        let word = get_word(&grid, row, col, 4, Direction::DiagonalRight);
                        if word == xmas || word == samx {
                            part_one_count += 1;
                        }
                    }
                    if can_v && can_nh {
                        let word = get_word(&grid, row, col, 4, Direction::DiagonalLeft);
                        if word == xmas || word == samx {
                            part_one_count += 1;
                        }
                    }
                }

                // Part Two Logic
                if grid[row][col] == 'M' || grid[row][col] == 'S' {
                    let can_h = col < width - 2;
                    let can_v = row < height - 2;
                    if can_h && can_v {
                        let right_word = get_word(&grid, row, col, 3, Direction::DiagonalRight);
                        if right_word == mas || right_word == sam {
                            let left_word =
                                get_word(&grid, row, col + 2, 3, Direction::DiagonalLeft);
                            if left_word == mas || left_word == sam {
                                part_two_count += 1;
                            }
                        }
                    }
                }
            }
        }

        (part_one_count, part_two_count)
    }

    fn get_word(
        grid: &[Vec<char>],
        row: usize,
        col: usize,
        size: usize,
        direction: Direction,
    ) -> String {
        let mut word = String::new();
        word.push(grid[row][col]);
        match direction {
            Direction::Horizontal => {
                for i in 1..size {
                    word.push(grid[row][col + i]);
                }
            }
            Direction::Vertical => {
                for i in 1..size {
                    word.push(grid[row + i][col]);
                }
            }
            Direction::DiagonalRight => {
                for i in 1..size {
                    word.push(grid[row + i][col + i]);
                }
            }
            Direction::DiagonalLeft => {
                for i in 1..size {
                    word.push(grid[row + i][col - i]);
                }
            }
        }
        word
    }
}
