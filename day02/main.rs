fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

#[derive(Clone, Copy)]
enum Direction {
    Increase,
    Decrease,
}

impl Direction {
    fn is_valid(self, left: i32, right: i32) -> bool {
        let delta = left - right;
        match self {
            Direction::Increase => {
                if delta > 0 {
                    return false;
                }
            }
            Direction::Decrease => {
                if delta < 0 {
                    return false;
                }
            }
        }
        if delta.abs() < 1 || delta.abs() > 3 {
            return false;
        }
        true
    }
}

fn part1() -> u32 {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|i| i.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .fold(
            0,
            |acc, line| {
                if is_line_valid(&line) {
                    acc + 1
                } else {
                    acc
                }
            },
        )
}

fn part2() -> u32 {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|i| i.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .fold(0, |acc, line| {
            if is_line_valid(&line) {
                acc + 1
            } else {
                for i in 0..line.len() {
                    let mut tmp = line.clone();
                    tmp.remove(i);
                    if is_line_valid(&tmp) {
                        return acc + 1;
                    }
                }
                acc
            }
        })
}

fn is_line_valid(line: &[i32]) -> bool {
    let delta = line[0] - line[1];
    if delta == 0 {
        return false;
    }
    let direction = if delta > 0 {
        Direction::Decrease
    } else {
        Direction::Increase
    };
    for i in 0..line.len() - 1 {
        if !direction.is_valid(line[i], line[i + 1]) {
            return false;
        }
    }

    true
}
