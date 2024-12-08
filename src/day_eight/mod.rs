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

    fn read_input(file_path: &str) -> Vec<Vec<char>> {
        let data = read_file_contents(file_path);
        data.lines().map(|line| line.chars().collect()).collect()
    }

    fn find_stations(grid: &[Vec<char>]) -> HashMap<char, Vec<(i32, i32)>> {
        let mut stations = HashMap::new();
        for (i, row) in grid.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                if val.is_alphanumeric() {
                    stations
                        .entry(val)
                        .or_insert_with(Vec::new)
                        .push((i as i32, j as i32));
                }
            }
        }
        stations
    }

    fn find_antinodes(
        grid: &mut Vec<Vec<char>>,
        stations: &HashMap<char, Vec<(i32, i32)>>,
        part2: bool,
    ) -> Vec<(i32, i32)> {
        let mut anti: Vec<(i32, i32)> = vec![];
        for (_station, coords) in stations.iter() {
            for i1 in 0..coords.len() {
                for i2 in 0..coords.len() {
                    if i1 == i2 {
                        continue;
                    }
                    let d = (coords[i1].0 - coords[i2].0, coords[i1].1 - coords[i2].1);
                    let nrange = if part2 { 0..=100 } else { 1..=1 };

                    for n in nrange {
                        for (i, s) in [(i1, 1), (i2, -1)].iter() {
                            let a = (coords[*i].0 + n * s * d.0, coords[*i].1 + n * s * d.1);
                            if let Some(cell) = grid
                                .get_mut(a.0 as usize)
                                .and_then(|row| row.get_mut(a.1 as usize))
                            {
                                *cell = '#';
                                if !anti.contains(&a) {
                                    anti.push(a);
                                }
                            }
                        }
                    }
                }
            }
        }
        anti
    }

    pub fn solve_part1(file_path: &str) -> i32 {
        let mut grid = read_input(file_path);
        let stations = find_stations(&grid);
        let anti = find_antinodes(&mut grid, &stations, false);
        anti.len() as i32
    }

    pub fn solve_part2(file_path: &str) -> i32 {
        let mut grid = read_input(file_path);
        let stations = find_stations(&grid);
        let anti = find_antinodes(&mut grid, &stations, true);
        anti.len() as i32
    }

    pub fn solve() -> (i32, i32) {
        let part1_result = solve_part1("./input/day_eight.txt");
        let part2_result = solve_part2("./input/day_eight.txt");
        (part1_result, part2_result)
    }
}
