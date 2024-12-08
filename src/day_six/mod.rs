use std::collections::HashSet;
use std::fs;

pub mod solution {
    use super::*;

    fn read_file_contents(file_path: &str) -> String {
        fs::read_to_string(file_path).unwrap_or_else(|_| {
            eprintln!("Failed to read the file: {}", file_path);
            String::new()
        })
    }

    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    struct Pos(isize, isize);

    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    enum Dir {
        Up,
        Right,
        Down,
        Left,
    }

    impl Dir {
        fn turn_right(self) -> Self {
            match self {
                Dir::Up => Dir::Right,
                Dir::Right => Dir::Down,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Up,
            }
        }

        fn offset(self) -> (isize, isize) {
            match self {
                Dir::Up => (-1, 0),
                Dir::Right => (0, 1),
                Dir::Down => (1, 0),
                Dir::Left => (0, -1),
            }
        }
    }

    pub fn solve() -> (u32, u32) {
        let input = read_file_contents("./input/day_six.txt");

        let mut grid: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();

        let start_pos = grid
            .iter()
            .enumerate()
            .find_map(|(x, row)| {
                row.iter()
                    .position(|&c| c == b'^')
                    .map(|y| Pos(x as isize, y as isize))
            })
            .expect("Start position not found");

        let visited = walk(&mut grid, start_pos, Dir::Up);
        let mut critical_count = 0;

        for obstacle in visited.clone() {
            if creates_loop(&mut grid, start_pos, obstacle) {
                critical_count += 1;
            }
        }

        (visited.len() as u32, critical_count as u32)
    }

    fn walk(grid: &mut [Vec<u8>], start: Pos, start_dir: Dir) -> HashSet<Pos> {
        let mut visited = HashSet::new();
        let mut pos = start;
        let mut dir = start_dir;

        loop {
            visited.insert(pos);

            let (dx, dy) = dir.offset();
            let next_pos = Pos(pos.0 + dx, pos.1 + dy);

            match grid
                .get(next_pos.0 as usize)
                .and_then(|row| row.get(next_pos.1 as usize))
            {
                Some(b'#') => {
                    dir = dir.turn_right();
                }
                Some(_) => {
                    pos = next_pos;
                }
                None => break,
            }
        }

        visited
    }

    fn creates_loop(grid: &mut [Vec<u8>], start: Pos, obstacle: Pos) -> bool {
        let mut visited = HashSet::new();
        let mut pos = start;
        let mut dir = Dir::Up;

        grid[obstacle.0 as usize][obstacle.1 as usize] = b'O';

        let mut looping = false;
        loop {
            if !visited.insert((pos, dir)) {
                looping = true;
                break;
            }

            let (dx, dy) = dir.offset();
            let next_pos = Pos(pos.0 + dx, pos.1 + dy);

            match grid
                .get(next_pos.0 as usize)
                .and_then(|row| row.get(next_pos.1 as usize))
            {
                Some(b'#') | Some(b'O') => {
                    dir = dir.turn_right();
                }
                Some(_) => {
                    pos = next_pos;
                }
                None => break,
            }
        }

        grid[obstacle.0 as usize][obstacle.1 as usize] = b'.';

        looping
    }
}
