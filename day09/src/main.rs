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

fn run_knot_simulation(lines: &Vec<String>, knot_number: usize) -> usize {
    let mut knot_positions = vec![(0, 0); knot_number];
    let mut last_knot_position_set: HashSet<(i32, i32)> = HashSet::from([knot_positions[knot_number - 1]]);

    for line in lines {
        let mut split_line = line.split_whitespace();
        let direction = split_line.next().unwrap().chars().next().unwrap();
        let steps: u32 = split_line.next().unwrap().parse().unwrap();

        for _ in 0..steps {
            match direction {
                'R' => knot_positions[0].0 += 1,
                'U' => knot_positions[0].1 += 1,
                'L' => knot_positions[0].0 -= 1,
                'D' => knot_positions[0].1 -= 1,
                _ => panic!("{} is not a valid direction", direction)
            }

            for head in 0..knot_number - 1 {
                let tail = head + 1;
                let x_diff = knot_positions[head].0 - knot_positions[tail].0;
                let y_diff = knot_positions[head].1 - knot_positions[tail].1;

                let x_diff_size = x_diff.abs();
                let y_diff_size = y_diff.abs();

                if x_diff_size >= 2 || y_diff_size >= 2 {
                    let x_change = x_diff.signum();
                    let y_change = y_diff.signum();
                    knot_positions[tail] = (knot_positions[tail].0 + x_change, knot_positions[tail].1 + y_change);
                }
            }
            last_knot_position_set.insert(knot_positions[knot_number - 1]);
        }
    }
    return last_knot_position_set.len();
}

fn solve_1(lines: &Vec<String>) -> usize {
   return run_knot_simulation(lines, 2);
}

fn solve_2(lines: &Vec<String>) -> usize {
    return run_knot_simulation(lines, 10);
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
        assert_eq!(solve_1(&lines), 13);
    }

    #[test]
    fn test_2() {
        let lines = read_file("test.txt");
        assert_eq!(solve_2(&lines), 1);
    }
}
