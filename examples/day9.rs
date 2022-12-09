use parse_display::{Display, FromStr};

fn distance(pos1: &(i32, i32), pos2: &(i32, i32)) -> f64 {
    let dx = pos1.0 - pos2.0;
    let dy = pos1.1 - pos2.1;
    return ((dx * dx + dy * dy) as f64).sqrt();
}

#[derive(Display, FromStr, Debug)]
enum Direction {
    #[display("U")]
    Up,
    #[display("D")]
    Down,
    #[display("L")]
    Left,
    #[display("R")]
    Right,
}

#[derive(Display, FromStr, Debug)]
#[display("{direction} {amount}")]
struct Movement {
    direction: Direction,
    amount: i32,
}

fn move_rope(
    movement: &Movement,
    head: &mut (i32, i32),
    tails: &mut Vec<(i32, i32)>,
    tail_positions: &mut Vec<(i32, i32)>,
) {
    for _ in 0..movement.amount {
        match movement.direction {
            Direction::Up => head.1 += 1,
            Direction::Down => head.1 -= 1,
            Direction::Left => head.0 -= 1,
            Direction::Right => head.0 += 1,
        }

        let mut prev = head.clone();
        tails.iter_mut().for_each(|tail| {
            let dist = distance(&prev, &tail);

            if dist > std::f64::consts::SQRT_2 {
                tail.0 += if prev.0 > tail.0 {
                    1
                } else if prev.0 < tail.0 {
                    -1
                } else {
                    0
                };
                tail.1 += if prev.1 > tail.1 {
                    1
                } else if prev.1 < tail.1 {
                    -1
                } else {
                    0
                };
            }
            prev = tail.clone();
        });

        let last_tail = tails.last().unwrap().clone();
        if !tail_positions.contains(&last_tail) {
            tail_positions.push(last_tail);
        }
    }
}

fn main() {
    let movements: Vec<Movement> = std::fs::read_to_string("./inputs/day9.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    let mut tail_positions: Vec<(i32, i32)> = vec![];
    let mut head = (0, 0);
    let mut tail = vec![(0, 0)];

    for movement in movements.iter() {
        move_rope(&movement, &mut head, &mut tail, &mut tail_positions);
    }
    println!("Part 1: {}", tail_positions.len());

    tail_positions = vec![];
    head = (0, 0);
    tail = vec![
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];

    for movement in movements.iter() {
        move_rope(&movement, &mut head, &mut tail, &mut tail_positions)
    }

    println!("Part 2: {}", tail_positions.len());
}
