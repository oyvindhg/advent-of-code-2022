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

fn parse(lines: &Vec<String>) -> Vec<String> {
    return lines.clone()
}

fn get_data(input_file: &str) -> Vec<String> {
    let lines = read_file(input_file);
    return parse(&lines);
}

fn solve_1(data: &Vec<String>) -> i32 {
    println!("{:?}", data);
    return 0;
}

fn solve_2(data: &Vec<String>) -> i32 {
    println!("{:?}", data);
    return 0;
}

fn main() {
    let data = get_data("input.txt");

    println!("---Task 1---");
    let now = Instant::now();
    println!("Solution: {}", solve_1(&data));
    println!("Time: {} ms", now.elapsed().as_millis());

    println!("\n---Task 2---");
    let now = Instant::now();
    println!("Solution: {}", solve_2(&data));
    println!("Time: {} ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use crate::{get_data, solve_1, solve_2};

    #[test]
    fn test_1() {
        let data = get_data("test.txt");
        assert_eq!(solve_1(&data), 0);
    }

    #[test]
    fn test_2() {
        let data = get_data("test.txt");
        assert_eq!(solve_2(&data), 0);
    }
}
