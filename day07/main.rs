fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
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
            for operators in
                Operators::new(operands.len() - 1, vec![Operator::Add, Operator::Multiply])
            {
                if apply_operators(&operands, &operators) == target {
                    return target as usize;
                }
            }

            0
        })
        .sum()
}

fn part2() -> usize {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            let (target, operands) = line.split_once(':').unwrap();
            let target = target.parse::<u128>().unwrap();
            let operands = operands
                .split_ascii_whitespace()
                .map(|x| x.parse::<u128>().unwrap())
                .collect::<Vec<_>>();

            use rayon::prelude::*;
            let operator_combinations = Operators::new(
                operands.len() - 1,
                vec![Operator::Add, Operator::Multiply, Operator::Concatenate],
            )
            .collect::<Vec<_>>();
            if let Some(operators) = operator_combinations
                .par_iter()
                .find_first(|operators| apply_operators(&operands, operators) == target)
            {
                if apply_operators(&operands.clone(), operators) == target {
                    return target as usize;
                }
            }

            /*
            for operators in Operators::new(
                operands.len() - 1,
                vec![Operator::Add, Operator::Multiply, Operator::Concatenate],
            ) {
                if apply_operators(&operands, &operators) == target {
                    return target as usize;
                }
            }
            */

            0
        })
        .sum()
}

fn apply_operators(operands: &[u128], operators: &[Operator]) -> u128 {
    operators
        .iter()
        .zip(operands[1..].iter())
        .fold(operands[0], |acc, (operator, operand)| match operator {
            Operator::Add => acc + operand,
            Operator::Multiply => acc * operand,
            Operator::Concatenate => {
                let mut number = acc.to_string();
                number.push_str(operand.to_string().as_str());
                number.parse::<u128>().unwrap()
            }
        })
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

struct Operators {
    len: usize,
    count: usize,
    operators: Vec<Operator>,
}

impl Operators {
    fn new(len: usize, operators: Vec<Operator>) -> Self {
        Operators {
            len,
            count: 0,
            operators,
        }
    }
}

impl Iterator for Operators {
    type Item = Vec<Operator>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count > self.operators.len().pow(self.len as u32) - 1 {
            return None;
        }

        // Convert current count into "number" with radix of number of operators
        // and each operator as a digit.
        let mut result = vec![self.operators[0]; self.len];
        let mut count = self.count;
        let mut i = 0;
        loop {
            let remainder = count % self.operators.len();
            result[self.len - 1 - i] = self.operators[remainder];
            count /= self.operators.len();
            i += 1;
            if count == 0 {
                break;
            }
        }
        self.count += 1;

        Some(result)
    }
}
