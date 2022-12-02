fn main() {
    let inputs = std::fs::read_to_string("./inputs/day1.txt").unwrap();

    let mut elves: Vec<i32> = inputs
        .split("\n\r\n")
        .map(|elf| {
            elf.trim()
                .split("\r\n")
                .map(|line| line.parse::<i32>().unwrap())
                .sum()
        })
        .collect();

    elves.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {}", elves.first().unwrap(),);
    println!("Part 2: {}", elves.iter().take(3).sum::<i32>());
}
