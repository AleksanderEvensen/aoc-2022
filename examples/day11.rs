use std::collections::VecDeque;

use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug, Clone)]
enum Operation {
    #[display("Operation: new = old + {0}")]
    Add(i32),

    #[display("Operation: new = old - {0}")]
    Sub(i32),

    #[display("Operation: new = old / {0}")]
    Div(i32),

    #[display("Operation: new = old * {0}")]
    Mult(i32),

    #[display("Operation: new = old * old")]
    Expo,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<i32>,
    operation: Operation,
    test_against: i32,
    throw_to: (usize, usize),
    inspection_count: i32,
}

fn main() {
    let mut monkeys: Vec<Monkey> = std::fs::read_to_string("./inputs/day11.txt")
        .unwrap()
        .split("\n\r\n")
        .map(|v| parse_monkey(v))
        .collect();
    let mut monkeys_2 = monkeys.clone();

    let base: i32 = monkeys_2.iter().map(|v| v.test_against).product();

    for _ in 1..=20 {
        for i in 0..monkeys.len() {
            if monkeys[i].items.len() == 0 {
                continue;
            }
            let targets = monkeys[i].throw_to;

            while monkeys[i].items.len() > 0 {
                do_monkey_business(&mut monkeys, i, targets.0, targets.1, 1, 3_f32);
            }
        }
    }
    for _ in 1..=10_000 {
        for i in 0..monkeys_2.len() {
            if monkeys_2[i].items.len() == 0 {
                continue;
            }
            let targets = monkeys_2[i].throw_to;

            while monkeys_2[i].items.len() > 0 {
                do_monkey_business(&mut monkeys_2, i, targets.0, targets.1, base, 1_f32);
            }
        }
    }

    println!("Monkeys:\n{:#?}", monkeys);
    let total_monkey_business = monkeys
        .iter()
        .sorted_by(|a, b| b.inspection_count.cmp(&a.inspection_count))
        .take(2)
        .fold(1, |v, mon| v * mon.inspection_count);
    let total_monkey_business_2: i64 = monkeys_2
        .iter()
        .sorted_by(|a, b| b.inspection_count.cmp(&a.inspection_count))
        .take(2)
        .fold(1, |v, mon| v * mon.inspection_count as i64);

    println!("Part 1: {}", total_monkey_business);
    println!("Part 2: {}", total_monkey_business_2);
}

fn do_monkey_business(
    monkeys: &mut Vec<Monkey>,
    i: usize,
    target_1: usize,
    target_2: usize,
    base: i32,
    div: f32,
) {
    monkeys[i].inspection_count += 1;
    let item = monkeys[i].items.pop_front().unwrap();

    let new_value: i64 = (match monkeys[i].operation {
        Operation::Add(x) => (item + x) as f64,
        Operation::Sub(x) => (item - x) as f64,
        Operation::Div(x) => (item as f64 / x as f64).floor() as f64,
        Operation::Mult(x) => (item as i64 * x as i64) as f64,
        Operation::Expo => (item as i64 * item as i64) as f64,
    } / div as f64)
        .floor() as i64;
    let new_value = (new_value % base as i64) as i32;

    if new_value % monkeys[i].test_against == 0 {
        monkeys[target_1].items.push_back(new_value);
    } else {
        monkeys[target_2].items.push_back(new_value);
    }
}

fn parse_monkey(string: &str) -> Monkey {
    let mut monkey = Monkey {
        items: VecDeque::new(),
        operation: Operation::Add(0),
        test_against: 0,
        throw_to: (0, 0),
        inspection_count: 0,
    };

    string.lines().skip(1).for_each(|line| {
        if let Some(items) = line.trim().strip_prefix("Starting items: ") {
            monkey.items = items.split(", ").map(|v| v.parse().unwrap()).collect();
        }
        if line.trim().starts_with("Operation: ") {
            monkey.operation = line.trim().parse().unwrap();
        }

        if let Some(divisible_by) = line.trim().strip_prefix("Test: divisible by ") {
            monkey.test_against = divisible_by.parse().unwrap();
        }

        if let Some(target_monkey) = line.trim().strip_prefix("If true: throw to monkey ") {
            monkey.throw_to.0 = target_monkey.parse().unwrap();
        }
        if let Some(target_monkey) = line.trim().strip_prefix("If false: throw to monkey ") {
            monkey.throw_to.1 = target_monkey.parse().unwrap();
        }
    });

    monkey
}
