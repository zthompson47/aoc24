fn main() {
    let secrets: Vec<u64> = include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    let part1: u64 = secrets
        .into_iter()
        .map(|mut num| {
            for _ in 0..2000 {
                num = step(num);
            }
            num
        })
        .sum();

    println!("Part 1: {part1}");
}

fn step(mut num: u64) -> u64 {
    num = num ^ (num * 64);
    num %= 16777216;
    num = num ^ (num / 32);
    num %= 16777216;
    num = num ^ (num * 2048);
    num %= 16777216;
    num
}
