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
        let input = read_file_contents("./input/day_six.txt");

        let mut grid = input
            .lines()
            .map(|line| line.as_bytes().to_vec())
            .collect::<Vec<_>>()
            .to_owned();

        let mut start_row = 0;
        let mut start_col = 0;
        'outer: for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == b'^' {
                    start_row = r;
                    start_col = c;
                    break 'outer;
                }
            }
        }

        let walk = |return_squares: bool, grid: &Vec<Vec<u8>>| -> Option<Vec<(usize, usize)>> {
            let mut seen = vec![vec![[false; 4]; grid[0].len()]; grid.len()];
            let mut row = start_row;
            let mut col = start_col;
            let mut dir = 0; // Direction: 0 = up, 1 = right, 2 = down, 3 = left

            loop {
                if seen[row][col][dir] {
                    return None;
                }
                seen[row][col][dir] = true;

                let (dr, dc) = [(-1, 0), (0, 1), (1, 0), (0, -1)][dir];
                let next_row = row.wrapping_add(dr as usize);
                let next_col = col.wrapping_add(dc as usize);

                if !(0..grid.len()).contains(&next_row) || !(0..grid[0].len()).contains(&next_col) {
                    if return_squares {
                        let mut visited = Vec::new();
                        for r in 0..grid.len() {
                            for c in 0..grid[0].len() {
                                if seen[r][c].iter().any(|&b| b) {
                                    visited.push((r, c));
                                }
                            }
                        }
                        return Some(visited);
                    }
                    return Some(Vec::new());
                }

                if grid[next_row][next_col] == b'#' {
                    dir = (dir + 1) % 4;
                } else {
                    row = next_row;
                    col = next_col;
                }
            }
        };

        let visited_squares = walk(true, &grid).unwrap();

        let mut critical_count = 0;
        for &(r, c) in &visited_squares {
            if walk(false, &grid).is_none() {
                critical_count += 1;
            }
            grid[r][c] = b'.';
        }

        (visited_squares.len() as u32, critical_count as u32)
    }
}
