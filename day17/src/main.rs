use std::cmp::max;
use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::Instant;

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

fn parse(lines: &Vec<String>) -> String {
    return lines[0].clone();
}

fn get_data(input_file: &str) -> String {
    let lines = read_file(input_file);
    return parse(&lines);
}

fn get_shape(round: u64) -> Vec<(u32, u64)> {
    match round % 5 {
        0 => Vec::from([(0, 0), (1, 0), (2, 0), (3, 0)]), // Flat line shape
        1 => Vec::from([(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]), // Plus shape
        2 => Vec::from([(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]), // Backwards L shape
        3 => Vec::from([(0, 0), (0, 1), (0, 2), (0, 3)]), // Vertical line shape
        4 => Vec::from([(0, 0), (0, 1), (1, 1), (1, 0)]), // Square shape
        _ => panic!("Not a valid shape")
    }
}

fn run_simulation(jet_pattern: &str, rounds: u64) -> u64 {
    let right_wall = 7;
    let jet_pattern_length = jet_pattern.len();
    let mut jet_index: usize = 0;

    let mut height: u64 = 0;

    let mut stationary_rocks = HashSet::new();
    for i in 0..right_wall {
        stationary_rocks.insert((i, 0));
    }

    let mut first_selected_jet_index = 0;
    let mut first_selected_round = 0;
    let mut first_selected_height = 0;
    let mut added_height = 0;
    let mut round = 0;

    while round < rounds {
        let mut rock = get_shape(round);

        for rock_part in rock.iter_mut() {
            rock_part.0 += 2;
            rock_part.1 += 4 + height;
        }

        let mut is_falling = true;

        while is_falling {
            let jet_direction = jet_pattern.chars().nth(jet_index).unwrap();
            if jet_index < jet_pattern_length - 1 {
                jet_index += 1;
            } else {
                jet_index = 0;
            }

            let mut can_move_sideways = true;
            if jet_direction == '>' {
                for rock_part in rock.iter() {
                    if (rock_part.0 == right_wall - 1) || stationary_rocks.contains(&(rock_part.0 + 1, rock_part.1)) {
                        can_move_sideways = false;
                    }
                }

                if can_move_sideways {
                    for rock_part in rock.iter_mut() {
                        rock_part.0 += 1;
                    }
                }
            } else if jet_direction == '<' {
                for rock_part in rock.iter() {
                    if (rock_part.0 == 0) || stationary_rocks.contains(&(rock_part.0 - 1, rock_part.1)) {
                        can_move_sideways = false;
                    }
                }
                if can_move_sideways {
                    for rock_part in rock.iter_mut() {
                        rock_part.0 -= 1;
                    }
                }
            }

            for rock_part in rock.iter() {
                if stationary_rocks.contains(&(rock_part.0, rock_part.1 - 1)) {
                    is_falling = false;
                }
            }
            if is_falling {
                for rock_part in rock.iter_mut() {
                    rock_part.1 -= 1;
                }
            } else {
                for rock_part in rock.iter() {
                    height = max(height, rock_part.1);
                    stationary_rocks.insert((rock_part.0, rock_part.1));
                }

                if first_selected_jet_index == 0 && jet_index != 0 && round > 1000 {
                    first_selected_jet_index = jet_index;
                    first_selected_round = round;
                    first_selected_height = height;
                } else if added_height == 0 && first_selected_jet_index != 0 && jet_index == first_selected_jet_index {
                    // The pattern repeats. Use the round difference and height difference of this
                    // pattern to calculate the height of all the remaining repetitions.

                    let round_difference = round - first_selected_round;
                    let height_difference = height - first_selected_height;
                    let remaining_rounds = (rounds - round) % round_difference;
                    let multiplication_factor = (rounds - round - remaining_rounds) / round_difference;

                    round = rounds - remaining_rounds;
                    added_height = height_difference * multiplication_factor;
                }

                round += 1;
            }
        }
    }
    return height + added_height;
}

fn solve_1(jet_pattern: &str) -> u64 {
    return run_simulation(jet_pattern, 2022);
}

fn solve_2(jet_pattern: &str) -> u64 {
    return run_simulation(jet_pattern, 1000000000000);
}

fn main() {
    let jet_pattern = get_data("input.txt");

    println!("---Task 1---");
    let now = Instant::now();
    println!("Solution: {}", solve_1(&jet_pattern));
    println!("Time: {} ms", now.elapsed().as_millis());

    println!("\n---Task 2---");
    let now = Instant::now();
    println!("Solution: {}", solve_2(&jet_pattern));
    println!("Time: {} ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use crate::{get_data, solve_1, solve_2};

    #[test]
    fn test_1() {
        let jet_pattern = get_data("test.txt");
        assert_eq!(solve_1(&jet_pattern), 3068);
    }

    #[test]
    fn test_2() {
        let jet_pattern = get_data("test.txt");
        assert_eq!(solve_2(&jet_pattern), 1514285714288);
    }
}

