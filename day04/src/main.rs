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

struct SectionPair {
    first_min: i32,
    first_max: i32,
    second_min: i32,
    second_max: i32,
}

fn parse(lines: &Vec<String>) -> Vec<SectionPair> {
    return lines.iter().map(|line| {
        let sections = line.split(',').collect::<Vec<&str>>();
        let first_sections = sections[0].split('-').collect::<Vec<&str>>();
        let second_sections = sections[1].split('-').collect::<Vec<&str>>();
        SectionPair {
            first_min: first_sections[0].parse().unwrap(),
            first_max: first_sections[1].parse().unwrap(),
            second_min: second_sections[0].parse().unwrap(),
            second_max: second_sections[1].parse().unwrap(),
        }
    }).collect::<Vec<SectionPair>>();
}

fn get_data(input_file: &str) -> Vec<SectionPair> {
    let lines = read_file(input_file);
    return parse(&lines);
}

fn solve_1(section_pairs: &Vec<SectionPair>) -> i32 {
    let mut fully_contained_pairs = 0;
    for section_pair in section_pairs {
        let first_contains_second = section_pair.first_min <= section_pair.second_min
            && section_pair.first_max >= section_pair.second_max;

        let second_contains_first = section_pair.first_min >= section_pair.second_min
            && section_pair.first_max <= section_pair.second_max;

        if first_contains_second || second_contains_first {
            fully_contained_pairs += 1;
        }
    }
    return fully_contained_pairs;
}

fn solve_2(section_pairs: &Vec<SectionPair>) -> i32 {
    let mut overlapping_pairs = 0;
    for section_pair in section_pairs {
        let first_contains_second_min = section_pair.first_min <= section_pair.second_min
            && section_pair.first_max >= section_pair.second_min;

        let first_contains_second_max = section_pair.first_min <= section_pair.second_max
            && section_pair.first_max >= section_pair.second_max;

        let second_contains_first_min = section_pair.second_min <= section_pair.first_min
            && section_pair.second_max >= section_pair.first_min;

        let second_contains_second_min = section_pair.second_min <= section_pair.first_max
            && section_pair.second_max >= section_pair.first_max;

        if first_contains_second_min || first_contains_second_max || second_contains_first_min || second_contains_second_min {
            overlapping_pairs += 1;
        }
    }
    return overlapping_pairs;
}

fn main() {
    let section_pairs = get_data("input.txt");
    println!("Task 1: {}", solve_1(&section_pairs));
    println!("Task 2: {}", solve_2(&section_pairs));
}

#[cfg(test)]
mod tests {
    use crate::{get_data, solve_1, solve_2};

    #[test]
    fn test_1() {
        let section_pairs = get_data("test.txt");
        assert_eq!(solve_1(&section_pairs), 2);
    }

    #[test]
    fn test_2() {
        let section_pairs = get_data("test.txt");
        assert_eq!(solve_2(&section_pairs), 4);
    }
}
