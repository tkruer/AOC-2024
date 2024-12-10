use std::fs;

pub mod solution {
    use super::*;

    pub const TRIANGLE: [usize; 10] = [0, 0, 1, 3, 6, 10, 15, 21, 28, 36];

    pub fn solve() -> (usize, usize) {
        let input = read_file_contents("./input/day_nine.txt");
        let disk = parse(&input);

        let part1_result = part1(&disk);
        let part2_result = part2(&disk);

        (part1_result, part2_result)
    }

    fn read_file_contents(file_path: &str) -> String {
        fs::read_to_string(file_path).unwrap_or_else(|_| {
            eprintln!("Failed to read the file: {}", file_path);
            String::new()
        })
    }

    fn parse(input: &str) -> Vec<usize> {
        input.trim().bytes().map(|b| (b - b'0') as usize).collect()
    }

    fn part1(disk: &[usize]) -> usize {
        let mut left = 0;
        let mut right = disk.len() - 2 + disk.len() % 2;
        let mut needed = disk[right];
        let mut block = 0;
        let mut checksum = 0;

        while left < right {
            (checksum, block) = update(checksum, block, left, disk[left]);
            let mut available = disk[left + 1];
            left += 2;

            while available > 0 {
                if needed == 0 {
                    if left == right {
                        break;
                    }
                    right -= 2;
                    needed = disk[right];
                }

                let size = needed.min(available);
                (checksum, block) = update(checksum, block, right, size);
                available -= size;
                needed -= size;
            }
        }

        (checksum, _) = update(checksum, block, right, needed);
        checksum
    }

    fn part2(disk: &[usize]) -> usize {
        let mut block = 0;
        let mut checksum = 0;
        let mut free: Vec<_> = (0..10).map(|_| Vec::with_capacity(1_100)).collect();

        for (index, &size) in disk.iter().enumerate() {
            if index % 2 == 1 && size > 0 {
                free[size].push(block);
            }
            block += size;
        }

        for i in 0..10 {
            free[i].push(block);
            free[i].reverse();
        }

        for (index, &size) in disk.iter().enumerate().rev() {
            block -= size;

            if index % 2 == 1 {
                continue;
            }

            let mut next_block = block;
            let mut next_index = usize::MAX;

            for i in size..free.len() {
                if let Some(top) = free[i].last() {
                    if *top < next_block {
                        next_block = *top;
                        next_index = i;
                    }
                }
            }

            if let Some(biggest) = free.last() {
                if !biggest.is_empty() {
                    if let Some(&top) = biggest.last() {
                        if top > block {
                            free.pop();
                        }
                    }
                }
            }

            let id = index / 2;
            let extra = next_block * size + TRIANGLE[size];
            checksum += id * extra;

            if next_index != usize::MAX {
                free[next_index].pop();

                let to = next_index - size;
                if to > 0 {
                    let value = next_block + size;
                    let mut i = free[to].len();

                    while i > 0 && free[to][i - 1] < value {
                        i -= 1;
                    }

                    free[to].insert(i, value);
                }
            }
        }

        checksum
    }

    fn update(checksum: usize, block: usize, index: usize, size: usize) -> (usize, usize) {
        let id = index / 2;
        let extra = block * size + TRIANGLE[size];
        (checksum + id * extra, block + size)
    }
}
