use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u32 {
    // Input two columns of numbers and sort.
    let lines = include_str!("input.txt").lines();
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in lines {
        let mut pair = line.split_whitespace();
        left.push(pair.next().unwrap().parse::<u32>().unwrap());
        right.push(pair.next().unwrap().parse::<u32>().unwrap());
    }
    left.sort();
    right.sort();

    // Calculate total difference between numbers.
    left.iter().zip(right).map(|(x, y)| x.abs_diff(y)).sum()
}

fn part2() -> u32 {
    // Input two columns of numbers.
    let lines = include_str!("input.txt").lines();
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in lines {
        let mut pair = line.split_whitespace();
        left.push(pair.next().unwrap().parse::<u32>().unwrap());
        right.push(pair.next().unwrap().parse::<u32>().unwrap());
    }

    // Count occurences of each number in right list.
    let mut count = HashMap::new();
    for i in right {
        count.entry(i).and_modify(|i| *i += 1).or_insert(1);
    }

    // Calculate similarity score of left list: multiply entry by count of value
    // in right list.
    left.iter()
        .map(|i| {
            if count.contains_key(i) {
                count.get(i).unwrap() * i
            } else {
                0
            }
        })
        .sum()
}
