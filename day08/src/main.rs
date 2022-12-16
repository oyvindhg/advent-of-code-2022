use std::cmp::max;
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

fn get_data(input_file: &str) -> Vec<Vec<u8>> {
    let lines = read_file(input_file);

    let mut tree_grid = vec![Vec::<u8>::new(); lines.len()];
    for (row, line) in lines.iter().enumerate() {
        for char in line.chars() {
            let tree_height: u8 = char.to_digit(10).unwrap() as u8;
            tree_grid[row].push(tree_height);
        }
    }

    return tree_grid;
}

fn solve_1(tree_grid: &Vec<Vec<u8>>) -> u32 {
    let mut visible_tree_count = 0;

    for row in 0..tree_grid.len() {
        for col in 0..tree_grid[0].len() {
            let height = tree_grid[row][col];
            let mut is_visible = true;

            for upper_row in 0..row {
                if tree_grid[upper_row][col] >= height {
                    is_visible = false;
                    break;
                }
            }
            if !is_visible {
                is_visible = true;
                for lower_row in row + 1..tree_grid.len() {
                    if tree_grid[lower_row][col] >= height {
                        is_visible = false;
                        break;
                    }
                }
            }
            if !is_visible {
                is_visible = true;
                for left_col in 0..col {
                    if tree_grid[row][left_col] >= height {
                        is_visible = false;
                        break;
                    }
                }
            }
            if !is_visible {
                is_visible = true;
                for right_col in col + 1..tree_grid[0].len() {
                    if tree_grid[row][right_col] >= height {
                        is_visible = false;
                        break;
                    }
                }
            }
            if is_visible {
                visible_tree_count += 1;
            }
        }
    }

    return visible_tree_count;
}

fn solve_2(tree_grid: &Vec<Vec<u8>>) -> u32 {
    let mut max_scenic_score = 0;

    for row in 0..tree_grid.len() {
        for col in 0..tree_grid[0].len() {
            let height = tree_grid[row][col];
            let mut current_scenic_score = 1;

            let mut current_direction_score = 0;
            for upper_row in (0..row).rev() {
                current_direction_score += 1;
                if tree_grid[upper_row][col] >= height {
                    break;
                }
            }
            current_scenic_score *= current_direction_score;
            current_direction_score = 0;
            for lower_row in row + 1..tree_grid.len() {
                current_direction_score += 1;
                if tree_grid[lower_row][col] >= height {
                    break;
                }
            }
            current_scenic_score *= current_direction_score;
            current_direction_score = 0;
            for left_col in (0..col).rev() {
                current_direction_score += 1;
                if tree_grid[row][left_col] >= height {
                    break;
                }
            }
            current_scenic_score *= current_direction_score;
            current_direction_score = 0;
            for right_col in col + 1..tree_grid[0].len() {
                current_direction_score += 1;
                if tree_grid[row][right_col] >= height {
                    break;
                }
            }
            current_scenic_score *= current_direction_score;
            max_scenic_score = max(max_scenic_score, current_scenic_score);
        }
    }

    return max_scenic_score;
}

fn main() {
    let tree_grid = get_data("input.txt");
    println!("Task 1: {}", solve_1(&tree_grid));
    println!("Task 2: {}", solve_2(&tree_grid));
}

#[cfg(test)]
mod tests {
    use crate::{get_data, solve_1, solve_2};

    #[test]
    fn test_1() {
        let tree_grid = get_data("test.txt");
        assert_eq!(solve_1(&tree_grid), 21);
    }

    #[test]
    fn test_2() {
        let tree_grid = get_data("test.txt");
        assert_eq!(solve_2(&tree_grid), 8);
    }
}
