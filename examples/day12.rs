use std::collections::{HashMap, VecDeque};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn manhatten_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl From<(i32, i32)> for Point {
    fn from(point: (i32, i32)) -> Self {
        Self {
            x: point.0,
            y: point.1,
        }
    }
}

fn main() {
    let mut start = Point::from((0, 0));
    let mut goal = Point::from((0, 0));
    let height_map: Vec<Vec<i32>> = std::fs::read_to_string("./inputs/day12.txt")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    if char == 'S' {
                        start = (x as i32, y as i32).into();
                    } else if char == 'E' {
                        goal = (x as i32, y as i32).into();
                    }
                    char as i32 - 97
                })
                .collect()
        })
        .collect();

    let data = find_path(&height_map, &start, &goal).unwrap();

    println!("Part 1: {}", data.len() - 1);

    // Part 2

    let best_start = vec![0; height_map.len()]
        .iter()
        .enumerate()
        .map(|(i, _)| i as i32)
        .collect::<Vec<i32>>()
        .par_iter()
        .map(|v| {
            if let Some(data) = find_path(&height_map, &Point { x: 0, y: *v }, &goal) {
                return data.len() as i32 - 1;
            }
            return i32::MAX;
        })
        .min()
        .unwrap();

    println!("Part 2: {}", best_start);
}

fn find_path(height_map: &Vec<Vec<i32>>, start: &Point, goal: &Point) -> Option<Vec<Point>> {
    let mut open_set: VecDeque<Point> = vec![].into();

    open_set.push_back(start.clone());

    let mut came_from: HashMap<Point, Point> = HashMap::new();

    let mut g_scores = HashMap::new();
    g_scores.insert(start.clone(), 0);

    let mut f_scores = HashMap::new();
    f_scores.insert(start.clone(), goal.manhatten_distance(&start));

    while !open_set.is_empty() {
        let (current_index, current) = open_set
            .iter()
            .enumerate()
            .min_by_key(|(_, n)| f_scores.get(n).unwrap())
            .unwrap();
        let current = current.clone();

        if current == goal.clone() {
            let mut current = current.clone();
            let mut total_path = vec![current.clone()];
            while came_from.contains_key(&current) {
                current = came_from[&current].clone();
                total_path.push(current.clone());
            }

            total_path.reverse();
            return Some(total_path);
        }

        open_set.remove(current_index);

        let current_height = if current == start.clone() {
            0
        } else {
            height_map[current.y as usize][current.x as usize]
        };

        let neighbors: Vec<Point> = vec![
            Point {
                x: current.x + 1,
                y: current.y,
            },
            Point {
                x: current.x - 1,
                y: current.y,
            },
            Point {
                x: current.x,
                y: current.y + 1,
            },
            Point {
                x: current.x,
                y: current.y - 1,
            },
        ]
        .iter()
        .filter(|v| {
            if v.y >= height_map.len() as i32 || v.y < 0 {
                return false;
            }

            if v.x >= height_map[v.y as usize].len() as i32 || v.x < 0 {
                return false;
            }

            let mut height = height_map[v.y as usize][v.x as usize];

            if v.x == goal.x && v.y == goal.y {
                height = 25
            }

            if height - current_height > 1 {
                return false;
            }

            return true;
        })
        .cloned()
        .collect();

        // println!("{:?}", neighbors.len());

        neighbors.iter().for_each(|neighbor| {
            if !g_scores.contains_key(neighbor) {
                g_scores.insert(neighbor.clone(), i32::MAX);
            }

            let tentative_gscore = g_scores[&current] + start.clone().manhatten_distance(&current);
            if tentative_gscore < g_scores[neighbor] {
                came_from.insert(neighbor.clone(), current.clone());
                g_scores.insert(neighbor.clone(), tentative_gscore);
                f_scores.insert(
                    neighbor.clone(),
                    tentative_gscore + goal.manhatten_distance(&current),
                );

                if !open_set.contains(&neighbor) {
                    open_set.push_back(neighbor.clone());
                }
            }
        });
    }
    None
}
