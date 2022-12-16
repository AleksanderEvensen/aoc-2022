use parse_display::{Display, FromStr};
use std::collections::HashMap;

#[derive(Display, FromStr, Debug, Clone, PartialEq, Eq, Hash)]
#[display("{x},{y}")]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Bounds {
    min_x: i32,
    max_x: i32,
    max_y: i32,
}

fn parse_data(path: &str) -> (Bounds, HashMap<Point, bool>) {
    let input: Vec<Vec<Point>> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.split(" -> ").map(|p| p.parse().unwrap()).collect())
        .collect();
    let mut bounds = Bounds {
        min_x: i32::MAX,
        max_x: 0,
        max_y: 0,
    };

    let mut wall_map: HashMap<Point, bool> = HashMap::new();

    for line in input {
        let mut line_iter = line.iter().peekable();

        while let Some(start) = line_iter.next() {
            let Some(end) = line_iter.peek() else {
				break;
			};

            wall_map.insert(start.clone(), true);
            wall_map.insert((*end).clone(), true);

            let dx = end.x - start.x;
            let dy = end.y - start.y;

            if start.x < bounds.min_x {
                bounds.min_x = start.x;
            }
            if start.x > bounds.max_x {
                bounds.max_x = start.x;
            }
            if start.y + 1 > bounds.max_y {
                bounds.max_y = start.y + 1;
            }
            if end.x < bounds.min_x {
                bounds.min_x = end.x;
            }
            if end.x > bounds.max_x {
                bounds.max_x = end.x;
            }
            if end.y + 1 > bounds.max_y {
                bounds.max_y = end.y + 1;
            }

            if dx != 0 {
                let dir = if dx < 0 { -1 } else { 1 };
                let dx = dx.abs();

                for x_offset in 0..dx {
                    wall_map.insert(
                        Point {
                            x: start.x + x_offset * dir,
                            y: start.y,
                        },
                        true,
                    );
                }
            } else if dy != 0 {
                let dir = if dy < 0 { -1 } else { 1 };
                let dy = dy.abs();
                for y_offset in 0..dy {
                    wall_map.insert(
                        Point {
                            x: start.x,
                            y: start.y + y_offset * dir,
                        },
                        true,
                    );
                }
            }
        }
    }

    (bounds, wall_map)
}

fn main() {
    let (bounds, walls) = parse_data("./inputs/day14.txt");

    let mut walls = walls;
    println!("{:?}", bounds);

    let mut still_sand: Vec<Point> = vec![];

    let mut current_sand = Point { x: 500, y: 0 };

    let mut part_1 = 0;

    'spawn_sand: loop {
        'move_sand: loop {
            if current_sand.y > bounds.max_y && part_1 == 0 {
                part_1 = still_sand.len();
            }
            let left_pos = Point {
                x: current_sand.x - 1,
                y: current_sand.y + 1,
            };
            let down_pos = Point {
                x: current_sand.x,
                y: current_sand.y + 1,
            };
            let right_pos = Point {
                x: current_sand.x + 1,
                y: current_sand.y + 1,
            };

            let can_go_left = !walls.contains_key(&left_pos);
            let can_go_right = !walls.contains_key(&right_pos);
            let can_go_down = !walls.contains_key(&down_pos);

            if current_sand.y < bounds.max_y {
                if can_go_down {
                    current_sand = down_pos;
                } else if can_go_left {
                    current_sand = left_pos;
                } else if can_go_right {
                    current_sand = right_pos;
                } else {
                    break 'move_sand;
                }
            } else {
                break 'move_sand;
            }
        }

        if current_sand == (Point { x: 500, y: 0 }) {
            still_sand.push(current_sand);
            break 'spawn_sand;
        }
        walls.insert(current_sand.clone(), true);
        still_sand.push(current_sand);
        current_sand = Point { x: 500, y: 0 };
    }

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", still_sand.len());
}
