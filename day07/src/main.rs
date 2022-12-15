use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use crate::ContentType::{DIRECTORY, FILE};

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

#[derive(PartialEq)]
#[derive(Debug)]
enum ContentType {
    FILE,
    DIRECTORY,
}

#[derive(Debug)]
struct DirectoryContent {
    content_type: ContentType,
    path: String,
    size: u32,
}

fn create_directory_map(lines: &Vec<String>) -> HashMap<String, Vec<DirectoryContent>> {
    let mut directory_map: HashMap<String, Vec<DirectoryContent>> = HashMap::new();
    directory_map.insert(
        "root".to_string(),
        vec!(DirectoryContent {
            content_type: DIRECTORY,
            path: String::from("|/"),
            size: 0,
        }),
    );

    let mut current_directory_path = "".to_string();

    for line in lines {
        let words = line.split_whitespace().collect::<Vec<&str>>();

        let first = words[0];
        match first {
            "$" => {
                let command = words[1];
                if command == "cd" {
                    let new_directory_name = words[2];
                    if new_directory_name == ".." {
                        let previous_directory_path = current_directory_path.to_owned();
                        let size: u32 = directory_map.get(&*current_directory_path)
                            .unwrap().iter().map(|content| content.size).sum();

                        current_directory_path = current_directory_path[0..current_directory_path.rfind('|').unwrap()].parse().unwrap();

                        directory_map.entry(current_directory_path.to_owned())
                            .and_modify(|vec: &mut Vec<DirectoryContent>| {
                                let element = vec.iter().position(|elem| elem.path == previous_directory_path).unwrap();
                                vec[element].size = size;
                            });
                    } else {
                        current_directory_path.push_str(&*("|".to_owned() + new_directory_name));
                        directory_map.insert(current_directory_path.to_owned(), Vec::new());
                    }
                }
            }
            "dir" => {
                let dir_name = words[1];
                let content = DirectoryContent {
                    content_type: DIRECTORY,
                    path: current_directory_path.to_owned() + &*"|".to_owned() + dir_name,
                    size: 0,
                };
                directory_map.entry(current_directory_path.to_owned())
                    .and_modify(|vec: &mut Vec<DirectoryContent>| vec.push(content));
            }
            _ => {
                let file_size: u32 = words[0].parse().unwrap();
                let filename = words[1];
                let content = DirectoryContent {
                    content_type: FILE,
                    path: current_directory_path.to_owned() + &*"|".to_owned() + filename,
                    size: file_size,
                };
                directory_map.entry(current_directory_path.to_owned())
                    .and_modify(|vec: &mut Vec<DirectoryContent>| vec.push(content));
            }
        }
    }

    // Navigate back to the "/" directory
    loop {
        let previous_directory_path = current_directory_path.to_owned();
        let size: u32 = directory_map.get(&*current_directory_path)
            .unwrap().iter().map(|content| content.size).sum();

        current_directory_path = current_directory_path[0..current_directory_path.rfind('|').unwrap()].parse().unwrap();

        directory_map.entry(current_directory_path.to_owned())
            .and_modify(|vec: &mut Vec<DirectoryContent>| {
                let element = vec.iter().position(|elem| elem.path == previous_directory_path).unwrap();
                vec[element].size = size;
            });
        if current_directory_path == "|/" {
            break;
        }
    }

    // Add the size of the "/" folder to the "root" element
    let size: u32 = directory_map.get("|/")
        .unwrap().iter().map(|content| content.size).sum();

    directory_map.entry("root".to_owned())
        .and_modify(|vec: &mut Vec<DirectoryContent>| {
            let element = vec.iter().position(|elem| elem.path == "|/").unwrap();
            vec[element].size = size;
        });

    return directory_map;
}

fn get_data(input_file: &str) -> HashMap<String, Vec<DirectoryContent>> {
    let lines = read_file(input_file);
    return create_directory_map(&lines);
}

fn solve_1(directory_map: &HashMap<String, Vec<DirectoryContent>>) -> u32 {
    let mut total_size: u32 = 0;
    let mut stack = vec!("root");
    while !stack.is_empty() {
        let path = stack.pop().unwrap();
        let contents = directory_map.get(path).unwrap();

        for content in contents {
            if content.content_type == DIRECTORY {
                if content.size <= 10_0000 {
                    total_size += content.size;
                }
                stack.push(&*content.path);
            }
        }
    }
    return total_size;
}

fn solve_2(directory_map: &HashMap<String, Vec<DirectoryContent>>) -> u32 {
    let maximum_space_allowed = 40_000_000;
    let current_space = directory_map.get("root").unwrap()[0].size;

    let minimum_directory_size = current_space - maximum_space_allowed;

    let mut smallest_valid_directory_size = current_space;
    let mut stack = vec!("root");
    while !stack.is_empty() {
        let path = stack.pop().unwrap();
        let contents = directory_map.get(path).unwrap();

        for content in contents {
            if content.content_type == DIRECTORY {
                if content.size >= minimum_directory_size && content.size <= smallest_valid_directory_size {
                    smallest_valid_directory_size = content.size;
                }
                stack.push(&*content.path);
            }
        }
    }
    return smallest_valid_directory_size;
}

fn main() {
    let directory_map = get_data("input.txt");
    println!("Task 1: {}", solve_1(&directory_map));
    println!("Task 2: {}", solve_2(&directory_map));
}

#[cfg(test)]
mod tests {
    use crate::{get_data, solve_1, solve_2};

    #[test]
    fn test_1() {
        let directory_map = get_data("test.txt");
        assert_eq!(solve_1(&directory_map), 95437);
    }

    #[test]
    fn test_2() {
        let directory_map = get_data("test.txt");
        assert_eq!(solve_2(&directory_map), 24933642);
    }
}
