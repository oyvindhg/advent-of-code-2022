use io::BufReader;
use std::{fs, io};
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

fn parse(lines: Vec<String>) -> Vec<(char, char)> {
    return lines.iter().map(|line| {
        let mut results = line.split_whitespace();
        let opponent_move = results.next().unwrap().chars().next().unwrap();
        let your_move = results.next().unwrap().chars().next().unwrap();
        (opponent_move, your_move)
    }).collect::<Vec<(char, char)>>();
}

fn get_data(input_file: &str) -> Vec<(char, char)> {
    let lines = read_file(input_file);
    return parse(lines);
}

fn score_match(opponent_hand: char, your_hand: char) -> i32 {
    let shape_score = match your_hand {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("{} is not a valid move!", your_hand)
    };

    let outcome_score = match (opponent_hand, your_hand) {
        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6, // Won
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3, // Draw
        ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0, // Lost
        _ => panic!("{:?} is not a valid match!", (opponent_hand, your_hand))
    };
    return shape_score + outcome_score;
}

fn solve_1(rounds: &Vec<(char, char)>) -> i32 {
    let mut total_score = 0;
    for round in rounds {
        let (opponent_move, your_move) = round;
        total_score += score_match(*opponent_move, *your_move);
    }
    return total_score;
}

// Encode from outcome char into the char for hand used in task 1
fn find_hand(opponent_hand: char, outcome: char) -> char {
    return match (opponent_hand, outcome) {
        ('B', 'X') | ('A', 'Y') | ('C', 'Z') => 'X',
        ('C', 'X') | ('B', 'Y') | ('A', 'Z') => 'Y',
        ('A', 'X') | ('C', 'Y') | ('B', 'Z') => 'Z',
        _ => panic!("{:?} is not a valid match!", (opponent_hand, outcome))
    };
}

fn solve_2(rounds: &Vec<(char, char)>) -> i32 {
    let mut total_score = 0;
    for round in rounds {
        let (opponent_move, outcome) = round;
        let your_move = find_hand(*opponent_move, *outcome);
        total_score += score_match(*opponent_move, your_move);
    }
    return total_score;
}

fn main() {
    let rounds = get_data("input.txt");
    println!("Task 1: {}", solve_1(&rounds));
    println!("Task 2: {}", solve_2(&rounds));
}

#[cfg(test)]
mod tests {
    use crate::{get_data, solve_1, solve_2};

    #[test]
    fn test_1() {
        let rounds = get_data("test.txt");
        assert_eq!(solve_1(&rounds), 15);
    }

    #[test]
    fn test_2() {
        let rounds = get_data("test.txt");
        assert_eq!(solve_2(&rounds), 12);
    }
}
