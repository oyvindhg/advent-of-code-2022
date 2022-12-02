use io::BufReader;
use std::{fs, io};
use std::cmp::max;
use std::env;
use std::io::BufRead;
use std::path::{Path, PathBuf};

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

fn solve_1(lines: &Vec<String>) -> i32 {
    let mut most_calories = 0;
    let mut current_calories = 0;
    for line in lines {
        if line.trim().is_empty() {
            most_calories = max(most_calories, current_calories);
            current_calories = 0;
        } else {
            let new_calories: i32 = line.parse().unwrap();
            current_calories += new_calories;
        }
    }
    return max(most_calories, current_calories);
}

fn solve_2(lines: &Vec<String>) -> i32 {
    let mut most_calories = 0;
    let mut second_most_calories = 0;
    let mut third_most_calories = 0;
    let mut current_calories = 0;
    for line in lines {
        if line.trim().is_empty() {
            if most_calories < current_calories {
                third_most_calories = second_most_calories;
                second_most_calories = most_calories;
                most_calories = current_calories;
            } else if second_most_calories < current_calories {
                third_most_calories = second_most_calories;
                second_most_calories = current_calories;
            } else {
                third_most_calories = max(third_most_calories, current_calories);
            }
            current_calories = 0;
        } else {
            let new_calories: i32 = line.parse().unwrap();
            current_calories += new_calories;
        }
    }
    return most_calories + second_most_calories + max(third_most_calories, current_calories);
}

fn main() {
    let lines = read_file("input.txt");
    println!("Task 1: {}", solve_1(&lines));
    println!("Task 2: {}", solve_2(&lines));
}

#[cfg(test)]
mod tests {
    use crate::{read_file, solve_1, solve_2};

    #[test]
    fn test_1() {
        let lines = read_file("test.txt");
        assert_eq!(solve_1(&lines), 24000);
    }

    #[test]
    fn test_2() {
        let lines = read_file("test.txt");
        assert_eq!(solve_2(&lines), 45000);
    }
}
