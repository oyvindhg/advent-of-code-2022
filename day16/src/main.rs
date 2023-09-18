use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::Instant;

#[derive(Debug)]
struct Valve {
    flow_rate: u32,
    neighbors: Vec<String>,
}

#[derive(Debug)]
struct PipeNetwork {
    valves: HashMap<String, Valve>,
}

#[derive(Debug)]
struct PipeNetworkAction {
    next_valve: String,
    additional_pressure_release: u32,
}

#[derive(Debug)]
struct PipeNetworkState {
    open_valves: Vec<String>,
    current_valves: Vec<String>,
    pressure_release: u32,
    minutes_left: u32,
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

fn parse(lines: &Vec<String>) -> PipeNetwork {
    let mut valves: HashMap<String, Valve> = HashMap::new();

    for line in lines {
        let valve_name = (&line[6..8]).to_owned();

        let equal_sign_index = line.find('=').unwrap();
        let colon_sign_index = line.find(';').unwrap();
        let flow_rate = line[equal_sign_index + 1..colon_sign_index].parse::<u32>().unwrap();

        let neighbors = line.split(", ")
            .map(|neighbor| neighbor[neighbor.len() - 2..neighbor.len()].to_owned())
            .collect::<Vec<String>>();

        let valve = Valve { flow_rate, neighbors };
        valves.insert(valve_name, valve);
    }
    return PipeNetwork { valves };
}

fn get_data(input_file: &str) -> PipeNetwork {
    let lines = read_file(input_file);
    return parse(&lines);
}

fn get_action_candidates(pipe_network: &PipeNetwork, current_valve: &str, minutes_left: u32, can_open_valve: bool) -> Vec<PipeNetworkAction> {
    let mut action_candidates: Vec<PipeNetworkAction> = Vec::new();

    if can_open_valve {
        let flow_rate = pipe_network.valves.get(current_valve).unwrap().flow_rate;
        let additional_pressure_release = flow_rate * minutes_left;

        if additional_pressure_release > 0 {
            let open_valve = PipeNetworkAction {
                next_valve: current_valve.to_owned(),
                additional_pressure_release,
            };
            action_candidates.push(open_valve);
        }
    }

    for neighbor in &pipe_network.valves.get(current_valve).unwrap().neighbors {
        let move_valve = PipeNetworkAction {
            next_valve: neighbor.to_owned(),
            additional_pressure_release: 0,
        };
        action_candidates.push(move_valve);
    }

    return action_candidates;
}

fn solve_1(pipe_network: &PipeNetwork) -> u32 {
    // The optimal_visited map stores the best pressure release for each state so far
    // The state is given as "OPEN_VALVES_COUNT[CURRENT_VALVES]", for example "3[FF]"
    let mut optimal_visited: HashMap<String, u32> = HashMap::new();
    optimal_visited.insert("[AA]".to_string(), 0);

    let mut highest_release: u32 = 0;

    let initial_state = PipeNetworkState {
        open_valves: Vec::new(),
        current_valves: Vec::from(["AA".to_string()]),
        pressure_release: 0,
        minutes_left: 30,
    };

    let mut visit_queue: VecDeque<PipeNetworkState> = VecDeque::from([initial_state]);

    while !visit_queue.is_empty() {
        let current_state = visit_queue.pop_front().unwrap();
        let valve = current_state.current_valves[0].to_owned();
        let updated_minutes_left = current_state.minutes_left - 1;

        if updated_minutes_left == 0 {
            break;
        }

        let can_open_valve = !current_state.open_valves.contains(&valve);
        let action_candidates = get_action_candidates(pipe_network, &valve, updated_minutes_left, can_open_valve);

        for action in action_candidates {
            let updated_pressure_release = current_state.pressure_release + action.additional_pressure_release;
            let mut updated_open_valves = current_state.open_valves.clone();

            if action.additional_pressure_release > 0 {
                updated_open_valves.push(valve.to_owned());
                updated_open_valves.sort();
            }

            let state_string = updated_open_valves.len().to_string() + "[" + &action.next_valve + "]";

            if !optimal_visited.contains_key(&state_string) || *optimal_visited.get(&state_string).unwrap() < updated_pressure_release {
                optimal_visited.insert(state_string, updated_pressure_release);

                let next_state = PipeNetworkState {
                    open_valves: updated_open_valves,
                    current_valves: Vec::from([action.next_valve.to_owned()]),
                    pressure_release: updated_pressure_release,
                    minutes_left: updated_minutes_left,
                };
                visit_queue.push_back(next_state);

                if updated_pressure_release > highest_release {
                    highest_release = updated_pressure_release;
                }
            }
        }
    }

    return highest_release;
}

fn solve_2(pipe_network: &PipeNetwork) -> u32 {
    // The optimal_visited map stores the best pressure release for each state so far
    // The state is given as "OPEN_VALVES_COUNT[CURRENT_VALVES]", for example "3[AA,FF]"
    let mut optimal_visited: HashMap<String, u32> = HashMap::new();
    optimal_visited.insert("[AA,AA]".to_string(), 0);

    let mut highest_release: u32 = 0;

    let initial_state = PipeNetworkState {
        open_valves: Vec::new(),
        current_valves: Vec::from(["AA".to_string(), "AA".to_string()]),
        pressure_release: 0,
        minutes_left: 26,
    };

    let mut non_zero_valve_count = 0;
    for valve in &pipe_network.valves {
        if valve.1.flow_rate > 0 {
            non_zero_valve_count += 1;
        }
    }

    let mut visit_queue: VecDeque<PipeNetworkState> = VecDeque::from([initial_state]);

    while !visit_queue.is_empty() {
        let current_state = visit_queue.pop_front().unwrap();
        let first_valve = current_state.current_valves[0].to_owned();
        let second_valve = current_state.current_valves[1].to_owned();
        let updated_minutes_left = current_state.minutes_left - 1;

        if updated_minutes_left == 0 {
            break;
        }

        let first_can_open_valve = !current_state.open_valves.contains(&first_valve);
        let second_can_open_valve = !current_state.open_valves.contains(&second_valve);
        let first_action_candidates = get_action_candidates(pipe_network, &first_valve, updated_minutes_left, first_can_open_valve);
        let second_action_candidates = get_action_candidates(pipe_network, &second_valve, updated_minutes_left, second_can_open_valve);

        for first_action in first_action_candidates {
            for second_action in &second_action_candidates {

                let updated_pressure_release = current_state.pressure_release + first_action.additional_pressure_release + second_action.additional_pressure_release;
                let mut updated_open_valves = current_state.open_valves.clone();

                if first_action.next_valve == second_action.next_valve && first_action.additional_pressure_release > 0 && second_action.additional_pressure_release > 0 {
                    continue; // They are trying to open the same valve, so skip this combination of actions
                }

                if first_action.additional_pressure_release > 0 {
                    updated_open_valves.push(first_action.next_valve.to_owned());
                    updated_open_valves.sort();
                }
                if second_action.additional_pressure_release > 0 {
                    updated_open_valves.push(second_action.next_valve.to_owned());
                    updated_open_valves.sort();
                }

                let mut action_string = String::new();
                if first_action.next_valve < second_action.next_valve {
                    action_string.push_str(&*first_action.next_valve);
                    action_string.push_str(",");
                    action_string.push_str(&*second_action.next_valve);
                } else {
                    action_string.push_str(&*second_action.next_valve);
                    action_string.push_str(",");
                    action_string.push_str(&*first_action.next_valve);
                }

                let state_string = updated_open_valves.len().to_string() + "[" + &*action_string + "]";

                if !optimal_visited.contains_key(&state_string) || *optimal_visited.get(&state_string).unwrap() < updated_pressure_release {
                    optimal_visited.insert(state_string, updated_pressure_release);

                    if updated_pressure_release > highest_release {
                        highest_release = updated_pressure_release;
                    }

                    if non_zero_valve_count > current_state.open_valves.len(){
                        let next_state = PipeNetworkState {
                            open_valves: updated_open_valves,
                            current_valves: Vec::from([first_action.next_valve.to_owned(), second_action.next_valve.to_owned()]),
                            pressure_release: updated_pressure_release,
                            minutes_left: updated_minutes_left,
                        };
                        visit_queue.push_back(next_state);
                    }
                }
            }
        }
    }

    return highest_release;
}

fn main() {
    let data = get_data("input.txt");

    println!("---Task 1---");
    let now = Instant::now();
    println!("Solution: {}", solve_1(&data));
    println!("Time: {} ms", now.elapsed().as_millis());

    println!("\n---Task 2---");
    let now = Instant::now();
    println!("Solution: {}", solve_2(&data));
    println!("Time: {} ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use crate::{get_data, solve_1, solve_2};

    #[test]
    fn test_1() {
        let data = get_data("test.txt");
        assert_eq!(solve_1(&data), 1651);
    }

    #[test]
    fn test_2() {
        let data = get_data("test.txt");
        assert_eq!(solve_2(&data), 1707);
    }
}
