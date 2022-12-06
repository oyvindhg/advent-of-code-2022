use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};
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

fn find_signal_marker(signal: &Vec<char>, lookback: usize) -> usize {
    for current_pos in lookback - 1..signal.len() {
        let mut found_repeated_letter = false;
        let mut seen_letters = HashSet::new();
        for pos in current_pos - (lookback - 1)..current_pos + 1 {
            let character = signal[pos];
            if seen_letters.contains(&character){
                found_repeated_letter = true;
            }
            seen_letters.insert(character);
        }
        if !found_repeated_letter {
            return current_pos + 1
        }
    }
    panic!("There was no packet marker")
}

fn solve_1(lines: &Vec<String>) -> usize {
    let line = lines.iter().next().unwrap();
    let signal = line.chars().collect::<Vec<char>>();
    return find_signal_marker(&signal, 4);
}

fn solve_2(lines: &Vec<String>) -> usize {
    let line = lines.iter().next().unwrap();
    let signal = line.chars().collect::<Vec<char>>();
    return find_signal_marker(&signal, 14);
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
        assert_eq!(solve_1(&lines), 7);
    }

    #[test]
    fn test_2() {
        let lines = read_file("test.txt");
        assert_eq!(solve_2(&lines), 19);
    }
}
