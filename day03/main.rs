fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> i32 {
    calculate(include_str!("input.txt"))
}

fn part2() -> i32 {
    let mut input = include_str!("input.txt");
    let mut sum = 0;
    while !input.is_empty() {
        if let Some((l, r)) = input.split_once("don't()") {
            sum += calculate(l);
            if let Some((_l, r)) = r.split_once("do()") {
                input = r;
            } else {
                input = "";
            }
        } else {
            sum += calculate(input);
        }
    }
    sum
}

fn calculate(input: &str) -> i32 {
    input
        .split("mul(")
        .skip(1)
        .map(|chunk| {
            if let Some((chunk, _)) = chunk.split_once(')') {
                if let Some((l, r)) = chunk.split_once(',') {
                    if let (Ok(l), Ok(r)) = (l.parse::<i32>(), r.parse::<i32>()) {
                        if (-999..=999).contains(&l) && (-999..=999).contains(&r) {
                            return l * r;
                        }
                    }
                }
            }
            0
        })
        .sum()
}
