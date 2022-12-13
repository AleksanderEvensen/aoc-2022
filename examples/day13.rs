use std::cmp::Ordering;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
enum PairValue {
    Number(i32),
    List(Vec<PairValue>),
}

impl PartialOrd for PairValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PairValue {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            PairValue::Number(x) => match other {
                PairValue::Number(y) => x.cmp(y),
                PairValue::List(y) => {
                    if y.len() == 0 {
                        return Ordering::Greater;
                    }
                    let cmp = PairValue::Number(*x).cmp(&y[0]);
                    if cmp == Ordering::Equal && y.len() > 1 {
                        return Ordering::Less;
                    }
                    return cmp;
                }
            },
            PairValue::List(x) => match other {
                PairValue::Number(y) => {
                    if x.len() == 0 {
                        return Ordering::Less;
                    }
                    let cmp = x[0].cmp(&PairValue::Number(*y));
                    if cmp == Ordering::Equal && x.len() > 1 {
                        return Ordering::Greater;
                    }
                    return cmp;
                }
                PairValue::List(y) => {
                    for i in 0..x.len() {
                        let Some(x_val) = x.get(i) else {
							return std::cmp::Ordering::Less
						};

                        let Some(y_val) = y.get(i) else {
							return std::cmp::Ordering::Greater
						};

                        if x_val < y_val {
                            return Ordering::Less;
                        }

                        if x_val > y_val {
                            return Ordering::Greater;
                        }
                    }

                    if x.len() == y.len() {
                        return std::cmp::Ordering::Equal;
                    }
                    return std::cmp::Ordering::Less;
                }
            },
        }
    }
}

fn parse_data(path: &str) -> Vec<Vec<PairValue>> {
    std::fs::read_to_string(path)
        .unwrap()
        .split("\n\r\n")
        .map(|pair| {
            let (left, right) = pair.split_once("\n").unwrap();
            let _left: Vec<PairValue> = serde_json::from_str(left).unwrap();
            let _right: Vec<PairValue> = serde_json::from_str(right).unwrap();

            vec![PairValue::List(_left), PairValue::List(_right)]
        })
        .collect()
}

fn main() {
    let mut pairs = parse_data("./inputs/day13.txt");

    let index_sum: i32 = pairs
        .iter()
        .enumerate()
        .filter(|(_, pair)| pair[0] < pair[1])
        .map(|(i, _)| i as i32 + 1)
        .sum();

    println!("Part 1: {}", index_sum);

    // Add in the divider packets
    pairs.push(vec![
        serde_json::from_str("[[6]]").unwrap(),
        serde_json::from_str("[[2]]").unwrap(),
    ]);

    let decoder_key = pairs
        .iter()
        .flatten()
        .sorted_by(|a, b| a.cmp(b))
        .enumerate()
        .fold(1, |v, (i, pair)| {
            if *pair == serde_json::from_str::<PairValue>("[[6]]").unwrap() {
                return v * (i as i32 + 1);
            }
            if *pair == serde_json::from_str::<PairValue>("[[2]]").unwrap() {
                return v * (i as i32 + 1);
            }
            v
        });

    println!("Part 2: {}", decoder_key);
}
