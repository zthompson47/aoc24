fn main() {
    println!("Part 1: {}", part1());
}

fn part1() -> usize {
    let stones: Vec<u128> = include_str!("input.txt")
        .split_ascii_whitespace()
        .map(|x| x.parse::<u128>().unwrap())
        .collect();

    //println!("{stones:?}");
    let result = run(&stones, 25);
    //println!("{result:?}");
    result.len()
}

fn run(stones: &[u128], blinks: usize) -> Vec<u128> {
    let mut stones = stones.to_vec();
    for i in 0..blinks {
        let mut r = Vec::new();
        for stone in stones {
            if stone == 0 {
                r.push(1);
            } else if stone.to_string().len() % 2 == 0 {
                let mut left = stone.to_string();
                let right = left.split_off(left.len() / 2);
                r.push(left.parse::<u128>().unwrap());
                r.push(right.parse::<u128>().unwrap());
            } else {
                r.push(stone * 2024);
            }
        }
        stones = r;
        //println!("{i} {stones:?}");
        //println!("{i}");
    }
    stones
}
