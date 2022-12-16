use parse_display::{Display, FromStr};
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Display, FromStr, Debug, Clone, Hash, PartialEq, Eq)]
#[display("x={x}, y={y}")]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Default)]
struct Bounds {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

impl Bounds {
    pub fn check_y(&mut self, y: i32) {
        if y > self.max_y {
            self.max_y = y;
        }
        if y < self.min_y {
            self.min_y = y;
        }
    }
    pub fn check_x(&mut self, x: i32) {
        if x > self.max_x {
            self.max_x = x;
        }
        if x < self.min_x {
            self.min_x = x;
        }
    }
}

fn manhatten_distance(point1: &Point, point2: &Point) -> i32 {
    (point1.x - point2.x).abs() + (point1.y - point2.y).abs()
}

fn parse_data(path: &str) -> (Bounds, HashMap<Point, i32>, Vec<Point>) {
    let input = std::fs::read_to_string(path).unwrap();
    let mut bounds = Bounds {
        min_x: i32::MAX,
        min_y: i32::MAX,
        max_x: 0,
        max_y: 0,
    };

    let mut points: HashMap<Point, i32> = HashMap::new();
    let mut beacons: Vec<Point> = vec![];

    input.lines().for_each(|line| {
        let (sensor, beacon) = line
            .strip_prefix("Sensor at ")
            .unwrap()
            .split_once(": closest beacon is at ")
            .unwrap();

        let beacon: Point = beacon.parse().unwrap();
        let sensor: Point = sensor.parse().unwrap();

        let distance = manhatten_distance(&sensor, &beacon);

        bounds.check_x(sensor.x + distance);
        bounds.check_x(beacon.x + distance);
        bounds.check_y(sensor.y + distance);
        bounds.check_y(beacon.y + distance);

        bounds.check_x(sensor.x - distance);
        bounds.check_x(beacon.x - distance);
        bounds.check_y(sensor.y - distance);
        bounds.check_y(beacon.y - distance);

        if !beacons.contains(&beacon) {
            beacons.push(beacon);
        }

        points.insert(sensor, distance);
    });

    return (bounds, points, beacons);
}

fn inside_sensor_range(pos: &Point, sensors: &HashMap<Point, i32>) -> bool {
    let mut is_inside = false;
    for (sensor, range) in sensors.iter() {
        if manhatten_distance(&sensor, &pos) <= *range {
            is_inside = true;
            break;
        }
    }
    is_inside
}

fn main() {
    let (bounds, sensors, beacons) = parse_data("./inputs/day15.txt");

    let unusable_positions = (0..(bounds.max_x - bounds.min_x))
        .into_par_iter()
        .fold(
            || 0,
            |count, i| {
                let current_pos = Point {
                    x: bounds.min_x + i,
                    y: 2_000_000,
                };

                if !beacons.contains(&current_pos) {
                    if inside_sensor_range(&current_pos, &sensors) {
                        return count + 1;
                    }
                }
                return count;
            },
        )
        .sum::<i32>();

    println!("Part 1: {}", unusable_positions);

    let frequency = sensors
        .par_iter()
        .find_map_first(|(sensor, range)| {
            let min_i = if -*range - 1 < 0 { 0 } else { -*range - 1 };
            let max_i = if *range + 2 > 4_000_000 {
                4_000_000
            } else {
                *range + 2
            };

            for i in min_i..max_i {
                let manhatten_x = range - i.abs() + 1;

                let left = Point {
                    x: sensor.x - manhatten_x,
                    y: sensor.y + i,
                };
                let right = Point {
                    x: sensor.x + manhatten_x,
                    y: sensor.y + i,
                };

                if left.x >= 0 && left.x <= 4_000_000 && left.y >= 0 && left.y <= 4_000_000 {
                    if !inside_sensor_range(&left, &sensors) {
                        return Some(left.x as i64 * 4_000_000 + left.y as i64);
                    }
                }
                if right.x >= 0 && right.x <= 4_000_000 && right.y >= 0 && right.y <= 4_000_000 {
                    if !inside_sensor_range(&right, &sensors) {
                        return Some(right.x as i64 * 4_000_000 + right.y as i64);
                    }
                }
            }
            None
        })
        .unwrap();

    println!("Part 2: {}", frequency);
}
