use std::fs;

pub mod solution {
    use super::*;

    fn read_file_contents(file_path: &str) -> String {
        fs::read_to_string(file_path).unwrap_or_else(|_| {
            eprintln!("Failed to read the file: {}", file_path);
            String::new()
        })
    }

    fn parse(input: String) -> (u32, u32) {
        let bytes = input.as_bytes();
        let mut index = 0;
        let mut is_enabled = true;
        let mut total_product = 0;
        let mut enabled_product = 0;

        while index < bytes.len() {
            if !matches!(bytes[index], b'm' | b'd') {
                index += 1;
                continue;
            }

            match bytes.get(index..) {
                Some(slice) if slice.starts_with(b"mul(") => {
                    index += 4; // Skip "mul("
                }
                Some(slice) if slice.starts_with(b"do()") => {
                    index += 4; // Skip "do()"
                    is_enabled = true;
                    continue;
                }
                Some(slice) if slice.starts_with(b"don't()") => {
                    index += 7; // Skip "don't()"
                    is_enabled = false;
                    continue;
                }
                _ => {
                    index += 1;
                    continue;
                }
            }

            let first_number = parse_number(bytes, &mut index);
            if bytes.get(index) != Some(&b',') {
                continue;
            }
            index += 1;

            let second_number = parse_number(bytes, &mut index);
            if bytes.get(index) != Some(&b')') {
                continue;
            }
            index += 1;

            let product = first_number * second_number;
            total_product += product;
            if is_enabled {
                enabled_product += product;
            }
        }

        (total_product, enabled_product)
    }

    fn parse_number(bytes: &[u8], index: &mut usize) -> u32 {
        let mut number = 0;
        while let Some(&digit) = bytes.get(*index) {
            if digit.is_ascii_digit() {
                number = number * 10 + (digit - b'0') as u32;
                *index += 1;
            } else {
                break;
            }
        }
        number
    }

    pub fn solve() -> (u32, u32) {
        return parse(read_file_contents("./input/day_three.txt"));
    }
}
