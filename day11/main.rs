use std::collections::HashMap;

fn main() {
    let stones: Vec<u128> = include_str!("input.txt")
        .split_ascii_whitespace()
        .map(|x| x.parse::<u128>().unwrap())
        .collect();
    //let mut cache = Cache::default();

    let part1 = run(&stones, 25).len();
    println!("Part 1: {}", part1);

    let part2 = run3(&stones, 75);
    println!("Part 2: {}", part2);

    /*
    let part2 = run2(&stones, 37, &mut cache);

    let mut num_count = cache.num_count.iter().collect::<Vec<_>>();
    num_count.sort_by(|l, r| r.1.cmp(l.1));
    println!("num_count: {:?}", num_count);

    let mut after = cache.after.iter().collect::<Vec<_>>();
    after.sort_by(|l, r| r.1.cmp(l.1));
    println!("after: {:?}", after);

    println!("Part 2: {}", part2);
    */
}

#[derive(Default, Debug)]
struct Cache {
    num_count: HashMap<u128, usize>,
    after: HashMap<u128, usize>,
}

#[allow(unused)]
fn run(stones: &[u128], blinks: usize) -> Vec<u128> {
    let mut stones = stones.to_vec();
    for i in 0..blinks {
        //println!("{i} {:?}", stones.len());
        let mut r = Vec::new();
        for stone in stones {
            r.append(&mut split_stone(stone));
        }
        //println!("{r:?}");
        stones = r;
    }
    stones
}

fn run3(stones: &[u128], blinks: usize) -> usize {
    let mut stones = fold_stones(stones);
    for i in 0..blinks {
        println!("{i} {:?}", stones.len());
        let new_stones: HashMap<u128, usize> =
            stones.iter().fold(HashMap::new(), |mut acc, (k, v)| {
                for stone in split_stone(*k) {
                    acc.entry(stone)
                        .and_modify(|count| *count += v)
                        .or_insert(*v);
                }
                acc
            });
        //println!("{new_stones:?}");
        stones = new_stones;
    }
    stones.values().sum()
}

fn fold_stones(stones: &[u128]) -> HashMap<u128, usize> {
    stones.iter().fold(HashMap::new(), |mut acc, stone| {
        acc.insert(*stone, 1);
        acc
    })
}

fn run2(stones: &[u128], blinks: usize, cache: &mut Cache) -> usize {
    //println!("blink: {blinks}");
    //println!("stones: {stones:?}");
    let mut result = 0;

    for stone in stones {
        //print!("{stone} ");
        //if *stone == 0 {
        //    print!(".");
        //}
        //if blinks == 40 {
        //    println!("....{stone:?}");
        //}
        if blinks == 25 {
            let _ = *cache
                .num_count
                .entry(*stone)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
        if blinks == 0 {
            let _ = *cache
                .after
                .entry(*stone)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }

        if blinks == 0 {
            result += 1;
        } else {
            let sub_stones = split_stone(*stone);
            let x = run2(&sub_stones, blinks - 1, cache);
            result += x;
        }
    }

    result
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
