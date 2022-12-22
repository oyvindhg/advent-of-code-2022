use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use crate::WorryLevelManagementType::{DivideByThree, ModByCommonMultiple};

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

#[derive(Clone)]
#[derive(Debug)]
struct MonkeyOperation {
    operator: char,
    new_worry_level: String,
}

#[derive(Clone)]
#[derive(Debug)]
struct MonkeyTest {
    divisor: u64,
    if_true_monkey_number: usize,
    if_false_monkey_number: usize,
}

#[derive(Clone)]
#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: MonkeyOperation,
    test: MonkeyTest,
    inspections: u32,
}

fn parse(lines: &Vec<String>) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    for (line_number, line) in lines.iter().enumerate() {
        if line.starts_with("Monkey") {
            // Parse the starting items
            let item_line = &lines[line_number + 1];
            let item_start_index = item_line.find(|c: char| c.is_digit(10)).unwrap();
            let split_items = item_line[item_start_index..item_line.len()].split(", ");
            let items = split_items.map(|item| item.parse::<u64>().unwrap()).collect::<Vec<u64>>();

            // Parse the operation
            let operation_line = &lines[line_number + 2];
            let operation_start_index = operation_line.find("old").unwrap() + 4;
            let mut split_operation = operation_line[operation_start_index..operation_line.len()].split_whitespace();
            let operator = split_operation.next().unwrap().chars().next().unwrap();
            let new_worry_level = split_operation.next().unwrap().to_string();
            let operation = MonkeyOperation { operator, new_worry_level };

            // Parse the test
            let test_line = &lines[line_number + 3];
            let test_divisor_index = test_line.find(|c: char| c.is_digit(10)).unwrap();
            let divisor: u64 = test_line[test_divisor_index..test_line.len()].parse::<u64>().unwrap();

            let true_line = &lines[line_number + 4];
            let true_monkey_index = true_line.find(|c: char| c.is_digit(10)).unwrap();
            let if_true = true_line[true_monkey_index..true_line.len()].parse::<usize>().unwrap();

            let false_line = &lines[line_number + 5];
            let false_monkey_index = false_line.find(|c: char| c.is_digit(10)).unwrap();
            let if_false = false_line[false_monkey_index..false_line.len()].parse::<usize>().unwrap();

            let test = MonkeyTest { divisor, if_true_monkey_number: if_true, if_false_monkey_number: if_false };

            // Add the new monkey to the list of monkeys
            monkeys.push(Monkey { items, operation, test, inspections: 0 })
        }
    }
    return monkeys;
}

fn get_data(input_file: &str) -> Vec<Monkey> {
    let lines = read_file(input_file);
    return parse(&lines);
}

enum WorryLevelManagementType {
    DivideByThree,
    ModByCommonMultiple,
}

fn run_monkey_in_the_middle(raw_monkeys: &Vec<Monkey>,
                            rounds: u32,
                            worry_management_type: WorryLevelManagementType) -> u64 {
    let mut monkeys = raw_monkeys.to_owned();

    let mut common_multiple = 1;
    for monkey in &monkeys {
        common_multiple *= monkey.test.divisor;
    }

    for _ in 0..rounds {
        for monkey_number in 0..monkeys.len() {
            for item_number in 0..monkeys[monkey_number].items.len() {
                let item = monkeys[monkey_number].items[item_number];

                // Let the monkey inspect the item, and calculate the next worry level
                monkeys[monkey_number].inspections += 1;

                let operation = &monkeys[monkey_number].operation;
                let new_level = match operation.new_worry_level.as_str() {
                    "old" => item.to_owned(),
                    x => x.to_owned().parse().unwrap()
                };
                let after_inspection_worry = match operation.operator {
                    '+' => item + new_level,
                    '*' => item * new_level,
                    _ => panic!("{} is not a valid operation", operation.operator)
                };
                let next_worry_level = match worry_management_type {
                    DivideByThree => after_inspection_worry / 3,
                    ModByCommonMultiple => after_inspection_worry % common_multiple
                };

                // Find the next monkey according to the test
                let test = &monkeys[monkey_number].test;
                let next_monkey = if next_worry_level % test.divisor == 0 {
                    test.if_true_monkey_number
                } else {
                    test.if_false_monkey_number
                };

                // Pass item to the next monkey
                monkeys[next_monkey].items.push(next_worry_level);
            }
            // Remove all items of the current monkey because they have been thrown away
            monkeys[monkey_number].items = Vec::new();
        }
    }

    let mut most_inspections = 0;
    let mut second_most_inspections = 0;

    for monkey in monkeys {
        if monkey.inspections > most_inspections {
            second_most_inspections = most_inspections;
            most_inspections = monkey.inspections;
        } else if monkey.inspections > second_most_inspections {
            second_most_inspections = monkey.inspections
        }
    }

    return most_inspections as u64 * second_most_inspections as u64;
}

fn solve_1(monkeys: &Vec<Monkey>) -> u64 {
    return run_monkey_in_the_middle(monkeys,
                                    20,
                                    DivideByThree);
}

fn solve_2(monkeys: &Vec<Monkey>) -> u64 {
    return run_monkey_in_the_middle(monkeys,
                                    10000,
                                    ModByCommonMultiple);
}

fn main() {
    let monkeys = get_data("input.txt");
    println!("Task 1: {}", solve_1(&monkeys));
    println!("Task 2: {}", solve_2(&monkeys));
}

#[cfg(test)]
mod tests {
    use crate::{get_data, solve_1, solve_2};

    #[test]
    fn test_1() {
        let monkeys = get_data("test.txt");
        assert_eq!(solve_1(&monkeys), 10605);
    }

    #[test]
    fn test_2() {
        let monkeys = get_data("test.txt");
        assert_eq!(solve_2(&monkeys), 2713310158);
    }
}
