fn main() {
    let inputs = std::fs::read_to_string("./inputs/day2.txt").unwrap();

    let inputs = inputs.split("\n").map(|v| {
        let s = v.trim().split(" ").take(2).collect::<Vec<&str>>();
        (s[0], s[1])
    });

    let total_score: i32 = inputs
        .clone()
        .map(|s| {
            let shape_score = match s.1.trim() {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => unreachable!(),
            };

            let outcome = match s {
                ("A", "X") => 3,
                ("A", "Y") => 6,
                ("A", "Z") => 0,

                ("B", "X") => 0,
                ("B", "Y") => 3,
                ("B", "Z") => 6,

                ("C", "X") => 6,
                ("C", "Y") => 0,
                ("C", "Z") => 3,

                _ => unreachable!(),
            };

            shape_score + outcome
        })
        .sum();

    let total_score_2: i32 = inputs
        .map(|s| {
            let shape_score = match s {
                ("A", "X") => 3,
                ("A", "Y") => 1,
                ("A", "Z") => 2,

                ("B", "X") => 1,
                ("B", "Y") => 2,
                ("B", "Z") => 3,

                ("C", "X") => 2,
                ("C", "Y") => 3,
                ("C", "Z") => 1,

                _ => unreachable!(),
            };

            let outcome = match s.1 {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => unreachable!(),
            };

            shape_score + outcome
        })
        .sum();

    println!("Part 1: {}", total_score);
    println!("Part 2: {}", total_score_2);
}
