use std::cmp::max;
use std::collections::VecDeque;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::Instant;

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_for_ore_robot: u32,
    ore_for_clay_robot: u32,
    ore_for_obsidian_robot: u32,
    clay_for_obsidian_robot: u32,
    ore_for_geode_robot: u32,
    obsidian_for_geode_robot: u32,
}

#[derive(Debug)]
struct Resources {
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
    ores: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
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

fn parse(lines: &Vec<String>) -> Vec<Blueprint> {
    return lines.iter().map(|line| {
        let blueprint_id_end = line.find(":").unwrap();
        let blueprint_id = line[10..blueprint_id_end].parse::<u32>().unwrap();

        let mut split_line = line.split('.');

        let ore_line = split_line.next().unwrap();
        let clay_line = split_line.next().unwrap();
        let obsidian_line = split_line.next().unwrap();
        let geode_line = split_line.next().unwrap();

        let ore_for_ore_start = ore_line.find("costs ").unwrap() + 6;
        let ore_for_ore_end = ore_line.rfind(" ore").unwrap();
        let ore_for_ore = ore_line[ore_for_ore_start..ore_for_ore_end].parse::<u32>().unwrap();

        let ore_for_clay_start = clay_line.find("costs ").unwrap() + 6;
        let ore_for_clay_end = clay_line.rfind(" ore").unwrap();
        let ore_for_clay = clay_line[ore_for_clay_start..ore_for_clay_end].parse::<u32>().unwrap();

        let ore_for_obsidian_start = obsidian_line.find("costs ").unwrap() + 6;
        let ore_for_obsidian_end = obsidian_line.rfind(" ore").unwrap();
        let ore_for_obsidian = obsidian_line[ore_for_obsidian_start..ore_for_obsidian_end].parse::<u32>().unwrap();
        let clay_for_obsidian_start = obsidian_line.find("and ").unwrap() + 4;
        let clay_for_obsidian_end = obsidian_line.rfind(" clay").unwrap();
        let clay_for_obsidian = obsidian_line[clay_for_obsidian_start..clay_for_obsidian_end].parse::<u32>().unwrap();

        let ore_for_geode_start = geode_line.find("costs ").unwrap() + 6;
        let ore_for_geode_end = geode_line.rfind(" ore").unwrap();
        let ore_for_geode = geode_line[ore_for_geode_start..ore_for_geode_end].parse::<u32>().unwrap();
        let obsidian_for_geode_start = geode_line.find("and ").unwrap() + 4;
        let obsidian_for_geode_end = geode_line.rfind(" obsidian").unwrap();
        let obsidian_for_geode = geode_line[obsidian_for_geode_start..obsidian_for_geode_end].parse::<u32>().unwrap();

        Blueprint {
            id: blueprint_id,
            ore_for_ore_robot: ore_for_ore,
            ore_for_clay_robot: ore_for_clay,
            ore_for_obsidian_robot: ore_for_obsidian,
            clay_for_obsidian_robot: clay_for_obsidian,
            ore_for_geode_robot: ore_for_geode,
            obsidian_for_geode_robot: obsidian_for_geode,
        }
    }).collect::<Vec<Blueprint>>();
}

fn get_data(input_file: &str) -> Vec<Blueprint> {
    let lines = read_file(input_file);
    return parse(&lines);
}

fn find_max_geodes(blueprint: &Blueprint, max_minutes: u32) -> u32 {
    let mut max_geodes = 0;
    let mut resource_queue: VecDeque<(u32, Resources)> = VecDeque::new();

    resource_queue.push_back(
        (
            0,
            Resources {
                ore_robots: 1,
                clay_robots: 0,
                obsidian_robots: 0,
                geode_robots: 0,
                ores: 0,
                clay: 0,
                obsidian: 0,
                geodes: 0,
            }
        )
    );

    while !resource_queue.is_empty() {
        let (minute, resources) = resource_queue.pop_front().unwrap();

        let remaining_minutes = max_minutes - minute;
        let mut max_possible_geode_robots = resources.geode_robots;
        let mut max_possible_geodes = resources.geodes;

        for _ in 0..remaining_minutes {
            max_possible_geodes += max_possible_geode_robots;
            max_possible_geode_robots += 1;
        }

        if max_possible_geodes < max_geodes {
            continue;
        }

        if minute >= max_minutes {
            return max_geodes;
        }

        let next_minute = minute + 1;

        if resources.ores >= blueprint.ore_for_ore_robot &&
            resources.ores < blueprint.ore_for_ore_robot + resources.ore_robots {
            resource_queue.push_back(
                (
                    next_minute,
                    Resources {
                        ores: resources.ores - blueprint.ore_for_ore_robot + resources.ore_robots,
                        clay: resources.clay + resources.clay_robots,
                        obsidian: resources.obsidian + resources.obsidian_robots,
                        geodes: resources.geodes + resources.geode_robots,
                        ore_robots: resources.ore_robots + 1,
                        ..resources
                    }
                )
            );
        }

        if resources.ores >= blueprint.ore_for_clay_robot &&
            resources.ores < blueprint.ore_for_clay_robot + resources.ore_robots {
            resource_queue.push_back(
                (
                    next_minute,
                    Resources {
                        ores: resources.ores - blueprint.ore_for_clay_robot + resources.ore_robots,
                        clay: resources.clay + resources.clay_robots,
                        obsidian: resources.obsidian + resources.obsidian_robots,
                        geodes: resources.geodes + resources.geode_robots,
                        clay_robots: resources.clay_robots + 1,
                        ..resources
                    }
                )
            );
        }

        if (resources.ores >= blueprint.ore_for_obsidian_robot &&
            resources.ores < blueprint.ore_for_obsidian_robot + resources.ore_robots &&
            resources.clay >= blueprint.clay_for_obsidian_robot) ||
            (resources.ores >= blueprint.ore_for_obsidian_robot &&
                resources.clay >= blueprint.clay_for_obsidian_robot &&
                resources.clay < blueprint.clay_for_obsidian_robot + resources.clay_robots) {
            resource_queue.push_back(
                (
                    next_minute,
                    Resources {
                        ores: resources.ores - blueprint.ore_for_obsidian_robot + resources.ore_robots,
                        clay: resources.clay - blueprint.clay_for_obsidian_robot + resources.clay_robots,
                        obsidian: resources.obsidian + resources.obsidian_robots,
                        geodes: resources.geodes + resources.geode_robots,
                        obsidian_robots: resources.obsidian_robots + 1,
                        ..resources
                    }
                )
            );
        }

        if (resources.ores >= blueprint.ore_for_geode_robot &&
            resources.ores < blueprint.ore_for_geode_robot + resources.ore_robots &&
            resources.obsidian >= blueprint.obsidian_for_geode_robot) ||
            (resources.ores >= blueprint.ore_for_geode_robot &&
                resources.obsidian >= blueprint.obsidian_for_geode_robot &&
                resources.obsidian < blueprint.obsidian_for_geode_robot + resources.obsidian_robots) {
            resource_queue.push_back(
                (
                    next_minute,
                    Resources {
                        ores: resources.ores - blueprint.ore_for_geode_robot + resources.ore_robots,
                        clay: resources.clay + resources.clay_robots,
                        obsidian: resources.obsidian - blueprint.obsidian_for_geode_robot + resources.obsidian_robots,
                        geodes: resources.geodes + resources.geode_robots,
                        geode_robots: resources.geode_robots + 1,
                        ..resources
                    }
                )
            );
        }

        resource_queue.push_back(
            (
                next_minute,
                Resources {
                    ores: resources.ores + resources.ore_robots,
                    clay: resources.clay + resources.clay_robots,
                    obsidian: resources.obsidian + resources.obsidian_robots,
                    geodes: resources.geodes + resources.geode_robots,
                    ..resources
                }
            )
        );

        max_geodes = max(max_geodes, resources.geodes + resources.geode_robots);
    }

    panic!("Simulation did not end!")
}

fn solve_1(blueprints: &Vec<Blueprint>) -> u32 {
    let mut total_quality_level = 0;
    for blueprint in blueprints {
        total_quality_level += blueprint.id * find_max_geodes(blueprint, 24);
    }

    return total_quality_level;
}

fn solve_2(blueprints: &Vec<Blueprint>, open_count: usize) -> u32 {
    let mut multiplied_geode_count = 1;

    for blueprint in blueprints.iter().take(open_count) {
        multiplied_geode_count *= find_max_geodes(blueprint, 32);
    }

    return multiplied_geode_count;
}

fn main() {
    let blueprints = get_data("input.txt");

    println!("---Task 1---");
    let now = Instant::now();
    println!("Solution: {}", solve_1(&blueprints));
    println!("Time: {} ms", now.elapsed().as_millis());

    println!("\n---Task 2---");
    let now = Instant::now();
    println!("Solution: {}", solve_2(&blueprints, 3));
    println!("Time: {} ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use crate::{get_data, solve_1, solve_2};

    #[test]
    fn test_1() {
        let blueprints = get_data("test.txt");
        assert_eq!(solve_1(&blueprints), 33);
    }

    #[test]
    fn test_2() {
        let blueprints = get_data("test.txt");
        assert_eq!(solve_2(&blueprints, 2), 56 * 62);
    }
}
