extern crate core;

use std::cmp::{max, min};
use std::collections::{HashSet, VecDeque};
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

fn find_start_position(lines: &Vec<String>) -> (usize, usize) {
    for (row, line) in lines.iter().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == 'S' {
                return (row, col);
            }
        }
    }
    panic!("There is no starting square");
}

fn find_bottom_positions(lines: &Vec<String>) -> Vec<(usize, usize)> {
    let mut positions: Vec<(usize, usize)> = Vec::new();
    for (row, line) in lines.iter().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == 'a' {
                positions.push((row, col));
            }
        }
    }
    return positions;
}

#[derive(Debug)]
struct Square {
    shortest_length: u32,
    position: (usize, usize),
    path: Vec<(usize, usize)>,
}

fn find_shortest_path(start_position: (usize, usize), lines: &Vec<String>) -> u32 {
    let max_row = lines.len();
    let max_col = lines[0].len();

    let mut square_queue: VecDeque<Square> = VecDeque::new();
    let mut visited_squares: HashSet<(usize, usize)> = HashSet::new();

    square_queue.push_back(
        Square { shortest_length: 0, position: start_position, path: Vec::new() }
    );
    visited_squares.insert(start_position);

    while !square_queue.is_empty() {
        let square = square_queue.pop_front().unwrap();
        let (row, col) = square.position;

        let new_row_min = max(0, row as i32 - 1) as usize;
        let new_col_min = max(0, col as i32 - 1) as usize;
        for new_row in new_row_min..min(row + 2, max_row) {
            for new_col in new_col_min..min(col + 2, max_col) {
                if (new_row != row || new_col != col) && (new_row == row || new_col == col) {
                    let current_char = lines[row].chars().nth(col).unwrap();
                    let new_char = lines[new_row].chars().nth(new_col).unwrap();
                    if new_char == 'E' && current_char as u32 >= 'y' as u32 {
                        return square.shortest_length + 1;
                    }
                    if new_char != 'E' &&
                        (new_char == 'a' || new_char as u32 <= current_char as u32 + 1) &&
                        !visited_squares.contains(&(new_row, new_col))
                    {
                        let mut path = square.path.to_owned();
                        path.push((new_row, new_col));
                        visited_squares.insert((new_row, new_col));
                        square_queue.push_back(
                            Square {
                                shortest_length: square.shortest_length + 1,
                                position: (new_row, new_col),
                                path,
                            }
                        );
                    }
                }
            }
        }
    }
    return u32::MAX;
}

fn solve_1(lines: &Vec<String>) -> u32 {
    let start_position = find_start_position(lines);
    return find_shortest_path(start_position, lines);
}

fn solve_2(lines: &Vec<String>) -> u32 {
    let start_position = find_start_position(lines);
    let mut shortest_path = find_shortest_path(start_position, lines);

    let bottom_positions = find_bottom_positions(lines);
    for bottom_position in bottom_positions{
        shortest_path = min(shortest_path, find_shortest_path(bottom_position, lines));
    }

    return shortest_path;
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
        assert_eq!(solve_1(&lines), 31);
    }

    #[test]
    fn test_2() {
        let lines = read_file("test.txt");
        assert_eq!(solve_2(&lines), 29);
    }
}
