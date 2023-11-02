use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::Instant;

#[derive(Debug)]
struct MonkeyJob {
    monkey: String,
    job: String,
}

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

fn parse(lines: &Vec<String>) -> Vec<MonkeyJob> {
    return lines.iter().map(|line| {
        let mut monkey_job = line.split(": ");
        let monkey = monkey_job.next().unwrap().to_owned();
        let job = monkey_job.next().unwrap().to_owned();
        MonkeyJob { monkey, job }
    }).collect::<Vec<MonkeyJob>>();
}

fn get_data(input_file: &str) -> Vec<MonkeyJob> {
    let lines = read_file(input_file);
    return parse(&lines);
}

fn create_original_ordered_list(monkey_to_job: &HashMap<String, String>) -> Vec<String> {
    let mut ordered_monkey_list: Vec<String> = vec!["root".to_string()];
    let mut current_monkey_number = 0;

    while current_monkey_number < ordered_monkey_list.len() {
        let monkey_name = &ordered_monkey_list[current_monkey_number];
        let job = monkey_to_job.get(monkey_name).unwrap();

        if !job.parse::<i64>().is_ok() {
            let mut split_job = job.split_whitespace();
            let first_monkey = split_job.next().unwrap().to_owned();
            split_job.next();
            let second_monkey = split_job.next().unwrap().to_owned();
            ordered_monkey_list.push(first_monkey);
            ordered_monkey_list.push(second_monkey);
        }
        current_monkey_number += 1;
    }

    ordered_monkey_list.reverse();
    return ordered_monkey_list;
}

fn calculate_original_monkey_numbers(monkey_to_job: &HashMap<String, String>,
                                     ordered_monkey_names: &Vec<String>) -> HashMap<String, i64> {
    let mut monkey_to_number: HashMap<String, i64> = HashMap::new();
    for monkey_name in ordered_monkey_names {
        let job = monkey_to_job.get(monkey_name).unwrap();

        if job.parse::<i64>().is_ok() {
            monkey_to_number.insert(monkey_name.to_owned(), job.parse().unwrap());
        } else {
            let mut split_job = job.split_whitespace();
            let first_monkey = split_job.next().unwrap();
            let first_number = monkey_to_number.get(first_monkey).unwrap();

            let operator = split_job.next().unwrap().chars().next().unwrap();

            let second_monkey = split_job.next().unwrap();
            let second_number = monkey_to_number.get(second_monkey).unwrap();

            let monkey_number = match operator {
                '+' => first_number + second_number,
                '-' => first_number - second_number,
                '*' => first_number * second_number,
                '/' => first_number / second_number,
                _ => panic!("{} is not a valid operation", operator)
            };

            monkey_to_number.insert(monkey_name.to_string(), monkey_number);
        }
    }

    return monkey_to_number;
}

fn create_revised_ordered_list(monkey_to_job: &HashMap<String, String>) -> Vec<String> {
    let mut original_ordered_monkey_list: Vec<String> = vec!["root".to_string()];
    let mut current_monkey_number = 0;

    let mut monkey_to_parent: HashMap<String, String> = HashMap::new();

    while current_monkey_number < original_ordered_monkey_list.len() {
        let monkey_name = original_ordered_monkey_list[current_monkey_number].to_string();
        let job = monkey_to_job.get(&monkey_name).unwrap().to_string();

        if !job.parse::<i64>().is_ok() {
            let mut split_job = job.split_whitespace();
            let first_monkey = split_job.next().unwrap().to_string();
            split_job.next();
            let second_monkey = split_job.next().unwrap().to_string();
            original_ordered_monkey_list.push(first_monkey.to_owned());
            original_ordered_monkey_list.push(second_monkey.to_owned());
            monkey_to_parent.insert(first_monkey.to_owned(), monkey_name.to_owned());
            monkey_to_parent.insert(second_monkey.to_owned(), monkey_name.to_owned());
        }
        current_monkey_number += 1;
    }

    let mut ordered_monkey_list: Vec<String> = vec![];
    let mut reversed_monkey_list: Vec<String> = vec![];
    let mut next_reversed_monkey = "humn";

    while !original_ordered_monkey_list.is_empty(){
        let monkey_name = original_ordered_monkey_list.pop().unwrap().to_string();
        if monkey_name.ne(next_reversed_monkey) {
            ordered_monkey_list.push(monkey_name);
        } else {
            reversed_monkey_list.push(monkey_name.to_owned());
            if monkey_name.ne("root") {
                next_reversed_monkey = monkey_to_parent.get(&*monkey_name).unwrap();
            }
        }
    }

    reversed_monkey_list.reverse();
    ordered_monkey_list.append(&mut reversed_monkey_list);

    return ordered_monkey_list;
}

fn calculate_revised_monkey_numbers(monkey_to_job: &HashMap<String, String>,
                                     ordered_monkey_names: &Vec<String>) -> HashMap<String, i64> {
    let mut monkey_to_number: HashMap<String, i64> = HashMap::new();
    let mut is_original_order = true;
    let mut previous_monkey = &ordered_monkey_names[0];
    for monkey_name in ordered_monkey_names {
        if monkey_name == "root" || previous_monkey == "root" {
            monkey_to_number.insert(monkey_name.to_string(), *monkey_to_number.get(previous_monkey).unwrap());
            is_original_order = false;
        } else {
            let job = if is_original_order {
                monkey_to_job.get(monkey_name).unwrap()
            } else {
                monkey_to_job.get(previous_monkey).unwrap()
            };

            if job.parse::<i64>().is_ok() {
                monkey_to_number.insert(monkey_name.to_owned(), job.parse().unwrap());
            } else {
                let mut split_job = job.split_whitespace();
                let first_monkey = split_job.next().unwrap();
                let operator = split_job.next().unwrap().chars().next().unwrap();
                let second_monkey = split_job.next().unwrap();

                let monkey_number = if is_original_order {
                    let first_number = monkey_to_number.get(first_monkey).unwrap();
                    let second_number = monkey_to_number.get(second_monkey).unwrap();

                    match operator {
                        '+' => first_number + second_number,
                        '-' => first_number - second_number,
                        '*' => first_number * second_number,
                        '/' => first_number / second_number,
                        _ => panic!("{} is not a valid operation", operator)
                    }
                } else {
                    let previous_monkey_number = monkey_to_number.get(previous_monkey).unwrap();
                    if first_monkey == monkey_name {
                        let second_number = monkey_to_number.get(second_monkey).unwrap();
                        match operator {
                            '+' => previous_monkey_number - second_number,
                            '-' => previous_monkey_number + second_number,
                            '*' => previous_monkey_number / second_number,
                            '/' => previous_monkey_number * second_number,
                            _ => panic!("{} is not a valid operation", operator)
                        }
                    } else {
                        let first_number = monkey_to_number.get(first_monkey).unwrap();
                        match operator {
                            '+' => previous_monkey_number - first_number,
                            '-' => first_number - previous_monkey_number,
                            '*' => previous_monkey_number / first_number,
                            '/' => first_number / previous_monkey_number,
                            _ => panic!("{} is not a valid operation", operator)
                        }
                    }
                };

                monkey_to_number.insert(monkey_name.to_string(), monkey_number);
            }
        }

        previous_monkey = monkey_name;
    }

    return monkey_to_number;
}

fn solve_1(monkey_jobs: &Vec<MonkeyJob>) -> i64 {
    let mut monkey_to_job: HashMap<String, String> = HashMap::new();

    for monkey_job in monkey_jobs {
        monkey_to_job.insert(monkey_job.monkey.to_owned(), monkey_job.job.to_owned());
    }

    let ordered_monkey_names = create_original_ordered_list(&monkey_to_job);
    let monkey_to_number = calculate_original_monkey_numbers(&monkey_to_job, &ordered_monkey_names);

    return monkey_to_number["root"];
}

fn solve_2(monkey_jobs: &Vec<MonkeyJob>) -> i64 {
    let mut monkey_to_job: HashMap<String, String> = HashMap::new();

    for monkey_job in monkey_jobs {
        monkey_to_job.insert(monkey_job.monkey.to_owned(), monkey_job.job.to_owned());
    }

    let ordered_monkey_names = create_revised_ordered_list(&monkey_to_job);
    let monkey_to_number = calculate_revised_monkey_numbers(&monkey_to_job, &ordered_monkey_names);

    return monkey_to_number["humn"];
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
        assert_eq!(solve_1(&data), 152);
    }

    #[test]
    fn test_2() {
        let data = get_data("test.txt");
        assert_eq!(solve_2(&data), 301);
    }
}
