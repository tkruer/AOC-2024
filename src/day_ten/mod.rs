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
        let input = read_file_contents("./input/day_ten.txt");
        let grid = parse(&input);

        let part1_result = runner(&grid, false);
        let part2_result = runner(&grid, true);

        (part1_result, part2_result)
    }

    fn parse(input: &str) -> Vec<Vec<u8>> {
        input.lines().map(|line| line.bytes().collect()).collect()
    }

    fn runner(grid: &[Vec<u8>], distinct: bool) -> u32 {
        let mut result = 0;
        let mut seen = vec![vec![-1; grid[0].len()]; grid.len()];

        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] == b'9' {
                    let id = (y * grid[0].len() + x) as i32;
                    result += dfs(grid, distinct, &mut seen, id, (x, y));
                }
            }
        }

        result
    }

    fn dfs(
        grid: &[Vec<u8>],
        distinct: bool,
        seen: &mut Vec<Vec<i32>>,
        id: i32,
        point: (usize, usize),
    ) -> u32 {
        let (x, y) = point;
        let mut result = 0;

        let directions = [
            (0, 1),  // Down
            (0, -1), // Up
            (1, 0),  // Right
            (-1, 0), // Left
        ];

        for &(dx, dy) in &directions {
            let next_x = x as isize + dx;
            let next_y = y as isize + dy;

            if next_x >= 0
                && next_x < grid[0].len() as isize
                && next_y >= 0
                && next_y < grid.len() as isize
            {
                let next = (next_x as usize, next_y as usize);
                if grid[next.1][next.0] + 1 == grid[y][x]
                    && (distinct || seen[next.1][next.0] != id)
                {
                    seen[next.1][next.0] = id;

                    if grid[next.1][next.0] == b'0' {
                        result += 1;
                    } else {
                        result += dfs(grid, distinct, seen, id, next);
                    }
                }
            }
        }

        result
    }
}
