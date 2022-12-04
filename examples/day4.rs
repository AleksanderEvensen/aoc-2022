fn main() {
    let inputs = std::fs::read_to_string("./inputs/day4.txt").unwrap();

    let ranges = inputs
        .lines()
        .map(|line| {
            line.split(',')
                .map(|range| {
                    let s = range
                        .split('-')
                        .map(|v| v.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    (s[0], s[1])
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .collect::<Vec<Vec<(i32, i32)>>>();

    let fully_contain = ranges
        .iter()
        .filter(|section| {
            (section[0].0 <= section[1].0 && section[0].1 >= section[1].1)
                || (section[1].0 <= section[0].0 && section[1].1 >= section[0].1)
        })
        .collect::<Vec<&Vec<(i32, i32)>>>();

    let any_overlap = ranges
        .iter()
        .filter(|section| {
            (section[0].0..=section[0].1).contains(&section[1].0)
                || (section[0].0..=section[0].1).contains(&section[1].1)
                || (section[1].0..=section[1].1).contains(&section[0].0)
                || (section[1].0..=section[1].1).contains(&section[0].1)
        })
        .collect::<Vec<&Vec<(i32, i32)>>>();

    println!("Part 1: {}", fully_contain.len());
    println!("Part 2: {}", any_overlap.len());
    // dbg!(fully_contain);
}
