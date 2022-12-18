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

fn solve_1(lines: &Vec<String>) -> i32 {
    let mut signal_strength = 0;
    let mut register_x = 1;
    let mut clock_cycle = 0;

    for line in lines {
        let mut split_line = line.split_whitespace();
        let command = split_line.next().unwrap();

        match command {
            "noop" => {
                clock_cycle += 1;
                if clock_cycle % 40 == 20 {
                    signal_strength += clock_cycle * register_x;
                }
            }
            "addx" => {
                let number: i32 = split_line.next().unwrap().parse().unwrap();
                clock_cycle += 1;
                if clock_cycle % 40 == 20 {
                    signal_strength += clock_cycle * register_x;
                }
                clock_cycle += 1;
                if clock_cycle % 40 == 20 {
                    signal_strength += clock_cycle * register_x;
                }
                register_x += number;
            }
            _ => panic!("{} is not a valid command", command)
        }
    }
    return signal_strength;
}

fn solve_2(lines: &Vec<String>) -> String {
    let mut register_x = 1;
    let mut clock_cycle = 0;
    let mut display = String::new();

    for line in lines {
        let clock_cycle_line = clock_cycle % 40;
        if clock_cycle_line == 0 {
            display.push('\n');
        }

        let pixel = if register_x >= clock_cycle_line - 1 && register_x <= clock_cycle_line + 1 { '#' } else { '.' };
        display.push(pixel);

        let mut split_line = line.split_whitespace();
        let command = split_line.next().unwrap();

        match command {
            "noop" => {
                clock_cycle += 1;
            }
            "addx" => {
                let number: i32 = split_line.next().unwrap().parse().unwrap();
                clock_cycle += 1;
                let clock_cycle_line = clock_cycle % 40;
                if clock_cycle % 40 == 0 {
                    display.push('\n');
                }
                let pixel = if register_x >= clock_cycle_line - 1 && register_x <= clock_cycle_line + 1 { '#' } else { '.' };
                display.push(pixel);
                clock_cycle += 1;
                register_x += number;
            }
            _ => panic!("{} is not a valid command", command)
        }
    }
    return display;
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
        assert_eq!(solve_1(&lines), 13140);
    }

    #[test]
    fn test_2() {
        let lines = read_file("test.txt");
        assert_eq!(solve_2(&lines), "\n##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....");
    }
}
