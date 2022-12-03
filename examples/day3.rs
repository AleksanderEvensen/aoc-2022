fn main() {
    let inputs = std::fs::read_to_string("./inputs/day3.txt").unwrap();

    let inputs = inputs.split("\r\n").collect::<Vec<&str>>();

    let compartments = inputs
        .iter()
        .map(|line| line.split_at(line.len() / 2))
        .collect::<Vec<(&str, &str)>>();

    let sum_eq: u32 = compartments
        .iter()
        .map(|(comp_prim, comp_second)| {
            let matching_char = comp_prim
                .chars()
                .find(|v| comp_second.contains(v.to_string().as_str()))
                .unwrap();
            if (matching_char as u32) < 97 {
                27 + (matching_char as u32) - 65
            } else {
                1 + (matching_char as u32) - 97
            }
        })
        .sum();

    let mut group_sum = 0;
    let mut input_iter = inputs.iter();
    while let Some(sack_1) = input_iter.next() {
        let sack_2 = input_iter.next().unwrap();
        let sack_3 = input_iter.next().unwrap();

        let common = sack_1
            .chars()
            .find(|char| {
                sack_2.contains(char.to_string().as_str())
                    && sack_3.contains(char.to_string().as_str())
            })
            .unwrap();
        if (common as u32) < 97 {
            group_sum += 27 + (common as u32) - 65
        } else {
            group_sum += 1 + (common as u32) - 97
        }
    }

    println!("Part 1: {}", sum_eq);
    println!("Part 2: {}", group_sum);
}
