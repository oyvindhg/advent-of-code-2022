extern crate core;

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

fn find_item_value(item: char) -> u32 {
    let item_value = item as u32;
    let uppercase_start = 'A' as u32 - 1;
    let lowercase_start = 'a' as u32 - 1;

    if item_value < lowercase_start {
        return item_value - uppercase_start + 26;
    }
    return item_value - lowercase_start;
}

fn find_compartment_priority(first_items: &str, second_items: &str) -> u32 {
    let mut first_items_set = HashSet::new();
    for item in first_items.chars() {
        first_items_set.insert(item);
    }

    for item in second_items.chars() {
        if first_items_set.contains(&item) {
            return find_item_value(item);
        }
    }

    panic!("No matching items!")
}

fn solve_1(lines: &Vec<String>) -> u32 {
    let mut priority_sum = 0;
    for line in lines {
        let middle_idx = line.len() / 2;
        let (first_items, second_items) = line.split_at(middle_idx);
        priority_sum += find_compartment_priority(first_items, second_items);
    }
    return priority_sum;
}

fn find_group_priority(first_items: &str, second_items: &str, third_items: &str) -> u32 {
    let mut first_items_set = HashSet::new();
    for item in first_items.chars() {
        first_items_set.insert(item);
    }

    let mut second_items_set = HashSet::new();
    for item in second_items.chars() {
        if first_items_set.contains(&item) {
            second_items_set.insert(item);
        }
    }

    for item in third_items.chars() {
        if second_items_set.contains(&item) {
            return find_item_value(item);
        }
    }

    panic!("No matching items!")
}

fn solve_2(lines: &Vec<String>) -> u32 {
    let mut priority_sum = 0;

    let mut first_items = "";
    let mut second_items = "";
    for (line_number, line) in lines.iter().enumerate() {
        let group_turn = (line_number + 1) % 3;
        match group_turn {
            1 => first_items = line,
            2 => second_items = line,
            0 => {
                priority_sum += find_group_priority(first_items, second_items, line);
            },
            _ => panic!("Not valid group")
        }
    }
    return priority_sum;
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
        assert_eq!(solve_1(&lines), 157);
    }

    #[test]
    fn test_2() {
        let lines = read_file("test.txt");
        assert_eq!(solve_2(&lines), 70);
    }
}
