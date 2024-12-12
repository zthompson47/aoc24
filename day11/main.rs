use std::collections::HashMap;

fn main() {
    let stones: Vec<u128> = include_str!("input.txt")
        .split_ascii_whitespace()
        .map(|x| x.parse::<u128>().unwrap())
        .collect();
    println!("Part 1: {}", run(&stones, 25));
    println!("Part 2: {}", run(&stones, 75));
}

fn run(stones: &[u128], blinks: usize) -> usize {
    let mut stones = stones.iter().fold(HashMap::new(), |mut acc, stone| {
        acc.insert(*stone, 1);
        acc
    });
    for _ in 0..blinks {
        let new_stones: HashMap<u128, usize> =
            stones.iter().fold(HashMap::new(), |mut acc, (k, v)| {
                for stone in split_stone(*k) {
                    acc.entry(stone)
                        .and_modify(|count| *count += v)
                        .or_insert(*v);
                }
                acc
            });
        stones = new_stones;
    }
    stones.values().sum()
}

fn split_stone(stone: u128) -> Vec<u128> {
    let mut result = Vec::new();
    if stone == 0 {
        result.push(1);
    } else if stone.to_string().len() % 2 == 0 {
        let mut left = stone.to_string();
        let right = left.split_off(left.len() / 2);
        result.push(left.parse::<u128>().unwrap());
        result.push(right.parse::<u128>().unwrap());
    } else {
        result.push(stone * 2024);
    }
    result
}
