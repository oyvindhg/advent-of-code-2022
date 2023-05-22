use std::cmp::{max, min};
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

#[derive(Debug)]
struct SensorReport {
    sensor: (i32, i32),
    beacon: (i32, i32),
}

fn parse_position(position_string: &str) -> (i32, i32) {
    let x_start = position_string.find("x=").unwrap() + 2;
    let x_stop = position_string.find(",").unwrap();
    let x: i32 = position_string[x_start..x_stop].parse::<i32>().unwrap();

    let y_start = position_string.find("y=").unwrap() + 2;
    let y: i32 = position_string[y_start..position_string.len()].parse::<i32>().unwrap();

    return (x, y);
}

fn parse(lines: &Vec<String>) -> Vec<SensorReport> {
    let reports = lines.iter().map(|line| {
        let mut split_line = line.split(":");

        let sensor_line = split_line.next().unwrap();
        let beacon_line = split_line.next().unwrap();

        let sensor_position = parse_position(sensor_line);
        let beacon_position = parse_position(beacon_line);

        SensorReport {
            sensor: sensor_position,
            beacon: beacon_position,
        }
    }).collect::<Vec<SensorReport>>();

    return reports;
}

fn get_data(input_file: &str) -> Vec<SensorReport> {
    let lines = read_file(input_file);
    return parse(&lines);
}

#[derive(Debug)]
struct SensorCoverage {
    sensor: (i32, i32),
    distance: i32,
}

fn calculate_distance(first: (i32, i32), second: (i32, i32)) -> i32 {
    return (second.0 - first.0).abs() + (second.1 - first.1).abs();
}

fn solve_1(sensor_reports: &Vec<SensorReport>, y: i32) -> i32 {
    let sensor_coverages = sensor_reports.iter().map(|report| {
        SensorCoverage {
            sensor: report.sensor,
            distance: calculate_distance(report.sensor, report.beacon),
        }
    }).collect::<Vec<SensorCoverage>>();

    let mut lowest_covered_x = sensor_coverages[0].sensor.0;
    let mut highest_covered_x = sensor_coverages[0].sensor.0;

    for sensor_coverage in &sensor_coverages {
        lowest_covered_x = min(
            lowest_covered_x,
            sensor_coverage.sensor.0 - sensor_coverage.distance,
        );

        highest_covered_x = max(
            highest_covered_x,
            sensor_coverage.sensor.0 + sensor_coverage.distance,
        );
    }

    let mut impossible_position_count = 0;

    for x in lowest_covered_x..=highest_covered_x {
        let current_position = (x, y);

        let mut is_beacon = false;
        for sensor_report in sensor_reports {
            // Check if the current position is on top of a beacon - if so we want to break out
            if calculate_distance(current_position, sensor_report.beacon) == 0 {
                is_beacon = true;
                break;
            }
        }

        if !is_beacon {
            for sensor_coverage in &sensor_coverages {
                let sensor_distance = calculate_distance(current_position, sensor_coverage.sensor);

                // Check if the current position is inside the coverage of the sensor - if so, we count
                // it, and also break out to the next position in order to not count it several times
                if sensor_distance <= sensor_coverage.distance {
                    impossible_position_count += 1;
                    break;
                }
            }
        }
    }

    return impossible_position_count;
}

fn solve_2(sensor_reports: &Vec<SensorReport>, x_max: i32, y_max: i32) -> i64 {
    let sensor_coverages = sensor_reports.iter().map(|report| {
        SensorCoverage {
            sensor: report.sensor,
            distance: calculate_distance(report.sensor, report.beacon),
        }
    }).collect::<Vec<SensorCoverage>>();

    for sensor_coverage in &sensor_coverages {
        let sensor_x = sensor_coverage.sensor.0;
        let sensor_y = sensor_coverage.sensor.1;

        // Start one step outside the border of the sensor coverage
        let mut x = sensor_x + sensor_coverage.distance + 1;
        let mut y = sensor_y;

        loop {
            if x < 0 || x > x_max || y < 0 || y > y_max {
                break;
            }

            let mut impossible_position = false;

            for other_sensor_coverage in &sensor_coverages {
                let sensor_distance = calculate_distance((x, y), other_sensor_coverage.sensor);

                // Check if the current position is inside the coverage of the sensor - if so, we
                // mark it as an impossible position for the hidden beacon and break out
                if sensor_distance <= other_sensor_coverage.distance {
                    impossible_position = true;
                    break;
                }
            }
            if !impossible_position {
                return (x as i64) * 4_000_000 + (y as i64);
            }

            if x > sensor_x && y >= sensor_y {
                // Move diagonally up left
                x -= 1;
                y += 1;
            } else if x <= 0 && y > sensor_y {
                // Move diagonally down left
                x -= 1;
                y -= 1;
            } else if x <= 0 {
                // Move diagonally down right
                x += 1;
                y -= 1;
            } else {
                // Move diagonally up right
                x += 1;
                y += 1;
            }

            // Check if we moved around the whole sensor
            if x == sensor_x + sensor_coverage.distance + 1 {
                break;
            }
        }
    }

    panic!("There is no possible position for the distress beacon!")
}

fn main() {
    let sensor_reports = get_data("input.txt");

    println!("---Task 1---");
    let now = Instant::now();
    println!("Solution: {}", solve_1(&sensor_reports, 2_000_000));
    println!("Time: {} ms", now.elapsed().as_millis());

    println!("\n---Task 2---");
    let now = Instant::now();
    println!("Solution: {}", solve_2(&sensor_reports, 4_000_000, 4_000_000));
    println!("Time: {} ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use crate::{get_data, solve_1, solve_2};

    #[test]
    fn test_1() {
        let sensor_reports = get_data("test.txt");
        assert_eq!(solve_1(&sensor_reports, 10), 26);
    }

    #[test]
    fn test_2() {
        let sensor_reports = get_data("test.txt");
        assert_eq!(solve_2(&sensor_reports, 20, 20), 56_000_011);
    }
}
