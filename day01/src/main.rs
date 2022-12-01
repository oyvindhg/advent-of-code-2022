use io::BufReader;
use std::{fs, io};
use std::cmp::max;
use std::env;
use std::io::BufRead;

fn main() {
    let root = env::current_dir().unwrap();
    let day = env!("CARGO_PKG_NAME");
    let file_path = root.join(day).join("input/input.txt");

    let file = fs::File::open(file_path).expect("Could not read file");
    let raw_lines = BufReader::new(file).lines();

    let lines = raw_lines.map(|raw_line| raw_line.unwrap()).collect::<Vec<String>>();

    let mut most_calories = 0;
    let mut current_calories = 0;
    for line in lines.clone() {
        if line.trim().is_empty() {
            most_calories = max(most_calories, current_calories);
            current_calories = 0;
        } else {
            let new_calories: i32 = line.parse().unwrap();
            current_calories += new_calories;
        }
    }
    println!("Task 1: {most_calories}");

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
    let total_calories = most_calories + second_most_calories + third_most_calories;
    println!("Task 2: {total_calories}");
}
