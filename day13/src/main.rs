use std::cmp::Ordering;
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

fn split_line(line: &String) -> Vec<String> {
    // Create a new string with comma separation also for '[' and ']'
    let mut comma_separated_line = String::new();
    for char in line.chars() {
        if char == ']' {
            comma_separated_line.push(',');
        }
        comma_separated_line.push(char);
        if char == '[' {
            comma_separated_line.push(',');
        }
    }

    // Split the string on commas and filter out empty elements
    return comma_separated_line.split(',')
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>();
}

fn is_right_order(first_raw: &String, second_raw: &String) -> bool {
    let first = split_line(first_raw);
    let second = split_line(second_raw);

    // The current index that is being processed the vector
    let mut first_index = 0;
    let mut second_index = 0;

    // The current actual list depth (as seen in the raw data)
    let mut first_depth = 0;
    let mut second_depth = 0;

    // If numbers are compared to lists, the number is wrapped in a list. The extended depth
    // measures the depth of this list
    let mut first_extended_depth = 0;
    let mut second_extended_depth = 0;

    // A flag that notifies if a number needs to be compared. If this happens, we need to stop
    // at the number and wait and potentially compare it to another number
    let mut first_is_number = false;
    let mut second_is_number = false;

    loop {
        // If we are not waiting to compare a number and the extended depth is not 0, we need to
        // reduce the extended depth by reading a "]" instead of the next word
        let first_word = if first_extended_depth == 0 || first_is_number {
            &first[first_index]
        } else {
            "]"
        };
        let second_word = if second_extended_depth == 0 || second_is_number {
            &second[second_index]
        } else {
            "]"
        };

        // The only possible words are "[", "]" and numbers
        if first_word != "[" && first_word != "]" {
            first_is_number = true;
        }
        if second_word != "[" && second_word != "]" {
            second_is_number = true;
        }

        if first_word == "[" {
            first_depth += 1;
            first_index += 1;

            // If the second word is waiting at a number, we need to extend its depth (add "[")
            if second_is_number {
                second_extended_depth += 1;
            }
        } else if first_word == "]" {
            if first_extended_depth > 0 {
                first_extended_depth -= 1;
            } else {
                first_depth -= 1;
                first_index += 1;
            }
        }

        if second_word == "[" {
            second_depth += 1;
            second_index += 1;

            // If the first word is waiting at a number, we need to extend its depth (add "[")
            if first_is_number {
                first_extended_depth += 1;
            }
        } else if second_word == "]" {
            if second_extended_depth > 0 {
                second_extended_depth -= 1;
            } else {
                second_depth -= 1;
                second_index += 1;
            }
        }

        if first_depth + first_extended_depth < second_depth + second_extended_depth {
            return true;
        } else if first_depth + first_extended_depth > second_depth + second_extended_depth {
            return false;
        }

        if first_is_number && second_is_number {
            let first_number: i32 = first_word.parse().unwrap();
            let second_number: i32 = second_word.parse().unwrap();
            if first_number < second_number {
                return true;
            } else if first_number > second_number {
                return false;
            } else {
                if first_extended_depth == 0 {
                    first_index += 1;
                }
                if second_extended_depth == 0 {
                    second_index += 1;
                }
            }
            first_is_number = false;
            second_is_number = false;
        }
    }
}

#[derive(Debug)]
#[derive(Eq)]
struct Packet {
    line: String
}

// To implement Ord, this also needs to be implemented
impl PartialEq<Self> for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line
    }
}

// To implement Ord, this also needs to be implemented
impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Define the order of packets so that they can be sorted
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        if is_right_order(&self.line, &other.line){
            return Ordering::Greater;
        }
        return Ordering::Less;
    }
}

fn solve_1(lines: &Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    let mut index = 0;
    while index < lines.len() {
        let first = &lines[index];
        let second = &lines[index + 1];

        if is_right_order(first, second) {
            sum += index as u32 / 3 + 1;
        }
        index += 3;
    }
    return sum;
}

fn solve_2(lines: &Vec<String>) -> u32 {

    let mut packets: Vec<Packet> = Vec::new();

    for line in lines{
        if !line.is_empty() {
            packets.push(Packet{line: line.to_owned()})
        }
    }

    packets.push(Packet{line: "[[2]]".parse().unwrap() });
    packets.push(Packet{line: "[[6]]".parse().unwrap() });

    packets.sort_by(|first, second| second.cmp(first));

    let first_position = packets.iter()
        .position(|packet| packet.line == "[[2]]").unwrap() as u32;

    let second_position = packets.iter()
        .position(|packet| packet.line == "[[6]]").unwrap() as u32;

    return (first_position + 1) * (second_position + 1);
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
        assert_eq!(solve_2(&lines), 140);
    }
}
