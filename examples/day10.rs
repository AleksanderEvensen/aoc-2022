use std::ops::RangeInclusive;

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug)]
enum OpCode {
    #[display("noop")]
    NoOp,
    #[display("addx {0}")]
    Addx(i32),
}

fn main() {
    let opcodes = std::fs::read_to_string("./inputs/day10.txt")
        .unwrap()
        .lines()
        .map(|v| v.parse::<OpCode>().unwrap())
        .map(|opcode| match opcode {
            OpCode::NoOp => vec![OpCode::NoOp],
            // This nice hack tho
            OpCode::Addx(x) => vec![OpCode::NoOp, OpCode::Addx(x)],
        })
        .flatten()
        .collect::<Vec<OpCode>>();

    let mut register: i32 = 1;
    let mut sum: i32 = 0;
    let mut screen = vec![vec![" ".to_string(); 40]; 6];
    let mut brush: RangeInclusive<i32> = 0..=2;

    for (cycle, opcode) in opcodes.iter().enumerate() {
        let pos = (
            cycle - (cycle as f32 / 40_f32).floor() as usize * 40,
            (cycle as f32 / 40_f32).floor() as usize,
        );
        let cycle = cycle as i32 + 1;

        if brush.contains(&(pos.0 as i32)) {
            screen[pos.1][pos.0] = "#".to_string();
        }

        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
            sum += cycle * register;
        }

        match opcode {
            OpCode::NoOp => {}
            OpCode::Addx(x) => register += x,
        };

        brush = (register - 1)..=(register + 1);
    }

    println!("Part 1: {}", sum);
    println!(
        "Part 2:\n{}",
        screen
            .iter()
            .map(|line| line
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(""))
            .collect::<Vec<String>>()
            .join("\n")
    );
}
