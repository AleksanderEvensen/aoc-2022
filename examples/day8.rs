fn main() {
    let inputs = std::fs::read_to_string("./inputs/day8.txt").unwrap();
    let inputs: Vec<Vec<i32>> = inputs
        .lines()
        .map(|l| {
            l.chars()
                .map(|char| char.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let width = inputs[0].len();
    let height = inputs.len();

    let mut visible = 0;

    let mut best_spot = 0;

    for i in 0..height {
        for (j, tree_height) in inputs[i].iter().enumerate() {
            // if is edge
            if i == 0 || i == width - 1 || j == 0 || j == height - 1 {
                visible += 1;
                continue;
            }

            let before = inputs[i].iter().take(j).max().unwrap();
            let after = inputs[i].iter().skip(j + 1).take(width - j).max().unwrap();

            let over = inputs
                .iter()
                .take(i)
                .map(|v| v.get(j).unwrap())
                .max()
                .unwrap();
            let under = inputs
                .iter()
                .skip(i + 1)
                .take(height - i)
                .map(|v| v.get(j).unwrap())
                .max()
                .unwrap();

            let is_visible = before < tree_height
                || after < tree_height
                || over < tree_height
                || under < tree_height;

            if is_visible {
                visible += 1;
            }

            let before = inputs[i]
                .iter()
                .rev()
                .skip(width - j)
                .take_while(|v| *v < tree_height)
                .collect::<Vec<&i32>>();

            let after = inputs[i]
                .iter()
                .skip(j + 1)
                .take_while(|v| *v < tree_height)
                .collect::<Vec<&i32>>();

            let over = inputs
                .iter()
                .rev()
                .skip(height - i)
                .map(|v| v.get(j).unwrap())
                .take_while(|v| *v < tree_height)
                .collect::<Vec<&i32>>();
            let under = inputs
                .iter()
                .skip(i + 1)
                .map(|v| v.get(j).unwrap())
                .take_while(|v| *v < tree_height)
                .collect::<Vec<&i32>>();

            let under = if under.len() == height - i - 1 {
                under.len()
            } else {
                under.len() + 1
            };
            let over = if over.len() == i {
                over.len()
            } else {
                over.len() + 1
            };
            let before = if before.len() == j {
                before.len()
            } else {
                before.len() + 1
            };
            let after = if after.len() == width - j - 1 {
                after.len()
            } else {
                after.len() + 1
            };

            let score = before * after * under * over;

            if i == 3 && j == 2 {
                println!("{}", score);
                println!("{:?}", (over, before, under, after));
                println!("");
            }

            if score > best_spot {
                best_spot = score;
            }
        }
    }

    println!("Part 1: {}", visible);
    println!("Part 2: {}", best_spot);
}
