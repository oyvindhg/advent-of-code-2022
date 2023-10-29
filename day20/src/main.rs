use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::Instant;

fn get_path(input_file: &str) -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    return root.join("input").join(input_file);
}

fn read_file(input_file: &str) -> Vec<String> {
    let file_path = get_path(input_file);
    let file = fs::File::open(file_path).expect("Could not read file");

    let raw_lines = BufReader::new(file).lines();
    return raw_lines.map(|raw_line| raw_line.unwrap()).collect::<Vec<String>>();
}

fn parse(lines: &Vec<String>) -> Vec<(usize, i64)> {
    return lines.iter().enumerate().map(|(line_number, line)| {
        let number = line.parse::<i64>().unwrap();
        (line_number, number)
    }).collect::<Vec<(usize, i64)>>();
}

fn get_data(input_file: &str) -> Vec<(usize, i64)> {
    let lines = read_file(input_file);
    return parse(&lines);
}

fn mix(original_code: &Vec<(usize, i64)>, decryption_key: i64, mix_count: u32) -> Vec<(usize, i64)> {
    let mut mixed_code = original_code.clone();
    let length = mixed_code.len();

    for order in 0..length {
        mixed_code[order] = (mixed_code[order].0, mixed_code[order].1 * decryption_key);
    }

    for _ in 0..mix_count {
        for order in 0..length {
            let (index, number) = mixed_code[order];

            let mut raw_updated_index = index as i64 + number;
            let mut updated_index;

            if raw_updated_index < 0 || raw_updated_index > (length - 1) as i64 {
                updated_index = raw_updated_index.rem_euclid((length - 1) as i64) as usize;
            } else {
                updated_index = raw_updated_index as usize;
            }

            mixed_code[order] = (updated_index, number);

            for adjustment_order in 0..length {
                if adjustment_order != order {
                    let not_adjusted_index = mixed_code[adjustment_order].0;
                    if updated_index > index {
                        if not_adjusted_index > index && not_adjusted_index <= updated_index {
                            mixed_code[adjustment_order] = (not_adjusted_index - 1, mixed_code[adjustment_order].1);
                        }
                    } else if updated_index < index {
                        if not_adjusted_index >= updated_index && not_adjusted_index < index {
                            mixed_code[adjustment_order] = (not_adjusted_index + 1, mixed_code[adjustment_order].1);
                        }
                    }
                }
            }
        }
    }
    return mixed_code;
}

fn find_coordinates(mixed_code: &Vec<(usize, i64)>) -> i64 {
    let length = mixed_code.len();

    for code_entry in mixed_code {
        if code_entry.1 == 0 {
            let zero_position = code_entry.0;

            let thousandth = (zero_position + 1000) % length;
            let twothousandth = (zero_position + 2000) % length;
            let threethousandth = (zero_position + 3000) % length;

            let mut coordinates = 0;
            for coordinate_candidate in mixed_code {
                if coordinate_candidate.0 == thousandth ||
                    coordinate_candidate.0 == twothousandth ||
                    coordinate_candidate.0 == threethousandth {
                    coordinates += coordinate_candidate.1;
                }
            }

            return coordinates;
        }
    }

    panic!("Could not find the 0 in the code")
}

fn solve_1(code: &Vec<(usize, i64)>) -> i64 {
    let mixed_code = mix(code, 1, 1);
    return find_coordinates(&mixed_code);
}

fn solve_2(code: &Vec<(usize, i64)>) -> i64 {
    let mixed_code = mix(code, 811589153, 10);
    return find_coordinates(&mixed_code);
}

fn main() {
    let coordinates = get_data("input.txt");

    println!("---Task 1---");
    let now = Instant::now();
    println!("Solution: {}", solve_1(&coordinates));
    println!("Time: {} ms", now.elapsed().as_millis());

    println!("\n---Task 2---");
    let now = Instant::now();
    println!("Solution: {}", solve_2(&coordinates));
    println!("Time: {} ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use crate::{get_data, solve_1, solve_2};

    #[test]
    fn test_1() {
        let coordinates = get_data("test.txt");
        assert_eq!(solve_1(&coordinates), 3);
    }

    #[test]
    fn test_2() {
        let coordinates = get_data("test.txt");
        assert_eq!(solve_2(&coordinates), 1623178306);
    }
}
