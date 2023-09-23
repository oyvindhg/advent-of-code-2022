use std::cmp::max;
use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::Instant;

#[derive(Debug)]
struct Boulder {
    cubes: HashSet<(u32, u32, u32)>,
    sides: u32,
    encompassing_cube: (u32, u32, u32),
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

fn parse(lines: &Vec<String>) -> Vec<(u32, u32, u32)> {
    return lines.iter().map(|line| {
        let cube = line.split(',')
            .map(|cube_string| cube_string.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        (cube[0], cube[1], cube[2])
    }).collect::<Vec<(u32, u32, u32)>>();
}

fn get_data(input_file: &str) -> Vec<(u32, u32, u32)> {
    let lines = read_file(input_file);
    return parse(&lines);
}

fn calculate_boulder(cubes: &Vec<(u32, u32, u32)>) -> Boulder {
    let mut visited_cubes: HashSet<(u32, u32, u32)> = HashSet::new();
    let mut sides = 0;

    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    for cube in cubes {
        let mut new_sides = 6;

        if cube.0 > 0 && visited_cubes.contains(&(cube.0 - 1, cube.1, cube.2)) {
            new_sides -= 2;
        }
        if visited_cubes.contains(&(cube.0 + 1, cube.1, cube.2)) {
            new_sides -= 2;
        }
        if cube.1 > 0 && visited_cubes.contains(&(cube.0, cube.1 - 1, cube.2)) {
            new_sides -= 2;
        }
        if visited_cubes.contains(&(cube.0, cube.1 + 1, cube.2)) {
            new_sides -= 2;
        }
        if cube.2 > 0 && visited_cubes.contains(&(cube.0, cube.1, cube.2 - 1)) {
            new_sides -= 2;
        }
        if visited_cubes.contains(&(cube.0, cube.1, cube.2 + 1)) {
            new_sides -= 2;
        }

        visited_cubes.insert(*cube);
        sides += new_sides;

        max_x = max(max_x, cube.0);
        max_y = max(max_y, cube.1);
        max_z = max(max_z, cube.2);
    }

    return Boulder {
        cubes: visited_cubes,
        sides: sides as u32,
        encompassing_cube: (max_x, max_y, max_z),
    };
}

fn is_internal(cube: (u32, u32, u32), cubes: &HashSet<(u32, u32, u32)>, encompassing_cube: (u32, u32, u32)) -> bool {
    let mut stack: Vec<(u32, u32, u32)> = Vec::new();
    stack.push(cube);

    let mut visited: HashSet<(u32, u32, u32)> = HashSet::new();
    visited.insert(cube);

    while !stack.is_empty() {
        let current_cube = stack.pop().unwrap();

        if current_cube.0 == 0 || current_cube.0 == encompassing_cube.0 ||
            current_cube.1 == 0 || current_cube.1 == encompassing_cube.1 ||
            current_cube.2 == 0 || current_cube.2 == encompassing_cube.2 {
            return false;
        }

        let mut neighbor = (current_cube.0 - 1, current_cube.1, current_cube.2);
        if !cubes.contains(&neighbor) && !visited.contains(&neighbor) {
            visited.insert(neighbor);
            stack.push(neighbor);
        }

        neighbor = (current_cube.0 + 1, current_cube.1, current_cube.2);
        if !cubes.contains(&neighbor) && !visited.contains(&neighbor) {
            visited.insert(neighbor);
            stack.push(neighbor);
        }

        neighbor = (current_cube.0, current_cube.1 - 1, current_cube.2);
        if !cubes.contains(&neighbor) && !visited.contains(&neighbor) {
            visited.insert(neighbor);
            stack.push(neighbor);
        }

        neighbor = (current_cube.0, current_cube.1 + 1, current_cube.2);
        if !cubes.contains(&neighbor) && !visited.contains(&neighbor) {
            visited.insert(neighbor);
            stack.push(neighbor);
        }

        neighbor = (current_cube.0, current_cube.1, current_cube.2 - 1);
        if !cubes.contains(&neighbor) && !visited.contains(&neighbor) {
            visited.insert(neighbor);
            stack.push(neighbor);
        }

        neighbor = (current_cube.0, current_cube.1, current_cube.2 + 1);
        if !cubes.contains(&neighbor) && !visited.contains(&neighbor) {
            visited.insert(neighbor);
            stack.push(neighbor);
        }
    }

    return true;
}

fn solve_1(cubes: &Vec<(u32, u32, u32)>) -> u32 {
    return calculate_boulder(cubes).sides;
}

fn solve_2(cubes: &Vec<(u32, u32, u32)>) -> u32 {
    let boulder = calculate_boulder(cubes);

    let mut sides = boulder.sides;

    let max_x = boulder.encompassing_cube.0;
    let max_y = boulder.encompassing_cube.1;
    let max_z = boulder.encompassing_cube.2;

    for x in 1..max_x {
        for y in 1..max_y {
            for z in 1..max_z {
                let current_cube = (x, y, z);

                if !boulder.cubes.contains(&current_cube) {
                    if is_internal(current_cube, &boulder.cubes, boulder.encompassing_cube) {
                        if cubes.contains(&(current_cube.0 - 1, current_cube.1, current_cube.2)) {
                            sides -= 1;
                        }
                        if cubes.contains(&(current_cube.0 + 1, current_cube.1, current_cube.2)) {
                            sides -= 1;
                        }
                        if cubes.contains(&(current_cube.0, current_cube.1 - 1, current_cube.2)) {
                            sides -= 1;
                        }
                        if cubes.contains(&(current_cube.0, current_cube.1 + 1, current_cube.2)) {
                            sides -= 1;
                        }
                        if cubes.contains(&(current_cube.0, current_cube.1, current_cube.2 - 1)) {
                            sides -= 1;
                        }
                        if cubes.contains(&(current_cube.0, current_cube.1, current_cube.2 + 1)) {
                            sides -= 1;
                        }
                    }
                }
            }
        }
    }

    return sides;
}

fn main() {
    let cubes = get_data("input.txt");

    println!("---Task 1---");
    let now = Instant::now();
    println!("Solution: {}", solve_1(&cubes));
    println!("Time: {} ms", now.elapsed().as_millis());

    println!("\n---Task 2---");
    let now = Instant::now();
    println!("Solution: {}", solve_2(&cubes));
    println!("Time: {} ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use crate::{get_data, solve_1, solve_2};

    #[test]
    fn test_1() {
        let cubes = get_data("test.txt");
        assert_eq!(solve_1(&cubes), 64);
    }

    #[test]
    fn test_2() {
        let cubes = get_data("test.txt");
        assert_eq!(solve_2(&cubes), 58);
    }
}
