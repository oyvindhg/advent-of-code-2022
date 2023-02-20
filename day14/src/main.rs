use std::cmp::{max, min};
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

#[derive(Debug)]
struct Cave {
    rocks: HashSet<(u32, u32)>,
    lowest_rock: u32,
}

fn parse(lines: &Vec<String>) -> Cave {
    let mut rocks: HashSet<(u32, u32)> = HashSet::new();
    let mut lowest_rock: u32 = 0;

    for line in lines {
        let corners = line.split(" -> ").map(|line| {
            let mut positions = line.split(',');
            let x: u32 = positions.next().unwrap().parse().unwrap();
            let y: u32 = positions.next().unwrap().parse().unwrap();
            (x, y)
        }).collect::<Vec<(u32, u32)>>();

        for current_corner in 1..corners.len() {
            let x1 = corners[current_corner - 1].0;
            let x2 = corners[current_corner].0;
            let y1 = corners[current_corner - 1].1;
            let y2 = corners[current_corner].1;

            for x in min(x1, x2)..max(x1, x2) + 1 {
                for y in min(y1, y2)..max(y1, y2) + 1 {
                    rocks.insert((x, y));
                }
            }

            lowest_rock = max(lowest_rock, max(y1, y2))
        }
    }

    return Cave {rocks, lowest_rock};
}

fn get_data(input_file: &str) -> Cave {
    let lines = read_file(input_file);
    return parse(&lines);
}

fn find_next_sand_position(occupied: &HashSet<(u32, u32)>, lowest_rock: u32, include_floor: bool) -> Option<(u32, u32)> {
    let mut sand_pos: (u32, u32) = (500, 0);
    let floor = lowest_rock + 2;

    if occupied.contains(&(sand_pos.0, sand_pos.1)) {
        return None;
    }

    while sand_pos.1 < floor {
        if !occupied.contains(&(sand_pos.0, sand_pos.1 + 1)) {
        } else if !occupied.contains(&(sand_pos.0 - 1, sand_pos.1 + 1)) {
            sand_pos.0 -= 1
        } else if !occupied.contains(&(sand_pos.0 + 1, sand_pos.1 + 1)) {
            sand_pos.0 += 1;
        } else {
            return Some(sand_pos);
        }
        sand_pos.1 += 1
    }

    return if include_floor == false {
        None
    } else {
        Some((sand_pos.0, floor - 1))
    }
}

fn fill_sand(cave: &Cave, include_floor: bool) -> u32 {
    let mut occupied: HashSet<(u32, u32)> = cave.rocks.iter().copied().collect();
    let lowest_rock = cave.lowest_rock;

    let mut sand_count = 0;
    loop {
        let sand_position = find_next_sand_position(&occupied, lowest_rock, include_floor);
        match sand_position {
            Some(position) => {
                sand_count += 1;
                occupied.insert(position);
            },
            None => return sand_count
        }
    }
}

fn solve_1(cave: &Cave) -> u32 {
    return fill_sand(cave, false)
}

fn solve_2(cave: &Cave) -> u32 {
    return fill_sand(cave, true)
}

fn main() {
    let cave = get_data("input.txt");
    println!("Task 1: {}", solve_1(&cave));
    println!("Task 2: {}", solve_2(&cave));
}

#[cfg(test)]
mod tests {
    use crate::{get_data, solve_1, solve_2};

    #[test]
    fn test_1() {
        let cave = get_data("test.txt");
        assert_eq!(solve_1(&cave), 24);
    }

    #[test]
    fn test_2() {
        let cave = get_data("test.txt");
        assert_eq!(solve_2(&cave), 93);
    }
}
