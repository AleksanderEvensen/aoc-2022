use std::collections::VecDeque;

fn main() {
    let inputs = std::fs::read_to_string("./inputs/day5.txt").unwrap();

    let inputs = inputs.split("\n\r\n").collect::<Vec<&str>>();

    let mut crates = {
        let inp = inputs[0]
            .lines()
            .take(inputs[0].lines().collect::<Vec<&str>>().len() - 1)
            .collect::<Vec<&str>>();

        let mut out: Vec<VecDeque<String>> = vec![];
        for i in 0..9 {
            let mut _crate = VecDeque::from(vec![]);

            inp.iter().for_each(|line| {
                let char = line.chars().collect::<Vec<char>>()[i * 4 + 1].to_string();
                if char != " " {
                    _crate.push_front(char);
                }
            });

            out.push(_crate)
        }
        out
    };

    let mut crates_2 = crates.clone();

    let instructions = inputs[1].lines();

    for instruction in instructions {
        let mut iter = instruction.split_whitespace();
        iter.next();
        let amount = iter.next().unwrap().parse::<usize>().unwrap();
        iter.next();
        let from = iter.next().unwrap().parse::<usize>().unwrap();
        iter.next();
        let to = iter.next().unwrap().parse::<usize>().unwrap();

        for _ in 0..amount {
            let value = crates[from - 1].pop_back().unwrap().clone();
            crates[to - 1].push_back(value);
        }
    }

    let instructions = inputs[1].lines();

    for instruction in instructions {
        let mut iter = instruction.split_whitespace();
        iter.next();
        let amount = iter.next().unwrap().parse::<usize>().unwrap();
        iter.next();
        let from = iter.next().unwrap().parse::<usize>().unwrap();
        iter.next();
        let to = iter.next().unwrap().parse::<usize>().unwrap();

        let mut moving_crates = VecDeque::from(vec![]);

        for _ in 0..amount {
            moving_crates.push_front(crates_2[from - 1].pop_back().unwrap().clone());
        }

        crates_2[to - 1].append(&mut moving_crates);
    }

    let top_crates = crates
        .iter()
        .map(|v| {
            if let Some(v) = v.back() {
                v.clone()
            } else {
                "".to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("");
    let top_crates_2 = crates_2
        .iter()
        .map(|v| {
            if let Some(v) = v.back() {
                v.clone()
            } else {
                "".to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("");
    println!("Part 1 {}", top_crates);
    println!("Part 2 {}", top_crates_2);
}
