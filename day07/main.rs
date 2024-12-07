fn main() {
    println!("Part 1: {}", part1());
}

fn part1() -> usize {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            let (target, operands) = line.split_once(':').unwrap();
            let target = target.parse::<u128>().unwrap();
            let operands = operands
                .split_ascii_whitespace()
                .map(|x| x.parse::<u128>().unwrap())
                .collect::<Vec<_>>();
            for operators in Operators::new(operands.len() - 1) {
                let result = operators.iter().zip(operands[1..].iter()).fold(
                    operands[0],
                    |acc, (operator, operand)| match operator {
                        Operator::Add => acc + operand,
                        Operator::Multiply => acc * operand,
                    },
                );
                if result == target {
                    return target as usize;
                }
            }

            0
        })
        .sum()
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

struct Operators {
    len: usize,
    count: usize,
}

impl Operators {
    fn new(len: usize) -> Self {
        Operators {
            len,
            count: 0,
        }
    }
}

impl Iterator for Operators {
    type Item = Vec<Operator>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count > 2usize.pow(self.len as u32) {
            return None;
        }

        let binary = format!("{:01$b}", self.count, self.len);
        self.count += 1;

        Some(
            binary
                .chars()
                .map(|c| {
                    if c == '0' {
                        Operator::Add
                    } else {
                        Operator::Multiply
                    }
                })
                .collect::<Vec<_>>(),
        )
    }
}
