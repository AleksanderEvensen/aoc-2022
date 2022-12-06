fn main() {
    let inputs = std::fs::read_to_string("./inputs/day6.txt").unwrap();

    let sequence = inputs
        .chars()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    let marker_start = sequence
        .iter()
        .enumerate()
        .find(|(i, _)| {
            has_unique_chars(
                &sequence
                    .iter()
                    .skip(i.clone())
                    .take(4)
                    .cloned()
                    .collect::<Vec<String>>()
                    .join(""),
            )
        })
        .unwrap();
    let msg_marker = sequence
        .iter()
        .enumerate()
        .find(|(i, _)| {
            has_unique_chars(
                &sequence
                    .iter()
                    .skip(i.clone())
                    .take(14)
                    .cloned()
                    .collect::<Vec<String>>()
                    .join(""),
            )
        })
        .unwrap();

    println!("Part 1: {:?}", marker_start.0 + 4);
    println!("Part 2: {:?}", msg_marker.0 + 14);
}

fn has_unique_chars(s: &String) -> bool {
    s.chars()
        .fold(vec![], |mut vec: Vec<char>, v| {
            if !vec.contains(&v) {
                vec.push(v);
            }
            vec
        })
        .len()
        == s.len()
}
