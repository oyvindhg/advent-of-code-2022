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

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct CargoCrane {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

fn parse(lines: &Vec<String>) -> CargoCrane {
    let mut setup_end = 0;
    for (line_number, line) in lines.iter().enumerate() {
        if line.is_empty() {
            setup_end = line_number;
            break;
        }
    }
    let crate_count: usize = lines[setup_end - 1].chars().last().unwrap().to_digit(10).unwrap() as usize;
    let mut stacks = vec![Vec::<char>::new(); crate_count];

    // Parse the initial state
    for line_num in (0..setup_end - 1).rev() {
        let mut chars = lines[line_num].chars();
        let mut is_first_character = true;
        for char_num in 0..crate_count {
            let character_location = if is_first_character { 1 } else { 3 };
            is_first_character = false;
            let raw_character = chars.nth(character_location);
            if raw_character.is_some() {
                let character = raw_character.unwrap();
                if !character.is_whitespace() {
                    stacks[char_num].push(character);
                }
            }
        }
    }

    // Parse the instructions
    let mut instructions = Vec::new();
    for line_num in setup_end + 1..lines.len() {
        let words = lines[line_num].split_whitespace();

        let mut amount: usize = 0;
        let mut from: usize = 0;
        let mut to: usize = 0;

        for (word_number, word) in words.enumerate() {
            match word_number {
                1 => amount = word.parse().unwrap(),
                3 => from = word.parse().unwrap(),
                5 => to = word.parse().unwrap(),
                _ => ()
            }
        }
        instructions.push(Instruction { from, to, amount });
    }
    return CargoCrane { stacks, instructions };
}

fn get_data(input_file: &str) -> CargoCrane {
    let lines = read_file(input_file);
    return parse(&lines);
}

fn solve_1(cargo_crane: &CargoCrane) -> String {
    let mut stacks = cargo_crane.stacks.to_vec();
    for instruction in &cargo_crane.instructions {
        for _ in 0..instruction.amount {
            let crate_value = stacks[instruction.from - 1].pop().unwrap();
            stacks[instruction.to - 1].push(crate_value)
        }
    }
    let mut word = String::from("");
    for mut stack in stacks {
        word.push(stack.pop().unwrap());
    }
    return word;
}

fn solve_2(cargo_crane: &CargoCrane) -> String {
    let mut stacks = cargo_crane.stacks.to_vec();
    let mut temporary_stack = Vec::<char>::new();
    for instruction in &cargo_crane.instructions {
        for _ in 0..instruction.amount {
            let crate_value = stacks[instruction.from - 1].pop().unwrap();
            temporary_stack.push(crate_value)
        }
        for _ in 0..instruction.amount {
            let crate_value = temporary_stack.pop().unwrap();
            stacks[instruction.to - 1].push(crate_value)
        }
    }
    let mut word = String::from("");
    for mut stack in stacks {
        word.push(stack.pop().unwrap());
    }
    return word;
}

fn main() {
    let cargo_crane = get_data("input.txt");
    println!("Task 1: {}", solve_1(&cargo_crane));
    println!("Task 2: {}", solve_2(&cargo_crane));
}

#[cfg(test)]
mod tests {
    use crate::{get_data, solve_1, solve_2};

    #[test]
    fn test_1() {
        let lines = get_data("test.txt");
        assert_eq!(solve_1(&lines), "CMZ");
    }

    #[test]
    fn test_2() {
        let lines = get_data("test.txt");
        assert_eq!(solve_2(&lines), "MCD");
    }
}
