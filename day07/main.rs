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
            for operators in Operators::new(operands.len() - 1) {
                let result = operators.iter().zip(operands[1..].iter()).fold(
                    operands[0],
                    |acc, (operator, operand)| match operator {
                        Operator::Add => acc + operand,
                        Operator::Multiply => acc * operand,
                        _ => unreachable!(),
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

fn part2() -> usize {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            println!("___line: {line}");
            let (target, operands) = line.split_once(':').unwrap();
            let target = target.parse::<u128>().unwrap();
            let operands = operands
                .split_ascii_whitespace()
                .map(|x| x.parse::<u128>().unwrap())
                .collect::<Vec<_>>();

            //for operators in Operators::new(operands.len() - 1) {
            for operators in Operators2::new(
                operands.len() - 1,
                //vec![Operator::Add, Operator::Multiply, Operator::Concatenate],
                vec![Operator::Add, Operator::Multiply, Operator::Concatenate],
            ) {
                //println!("operators: {operators:?}");
                let result = operators.iter().zip(operands[1..].iter()).fold(
                    operands[0],
                    |acc, (operator, operand)| match operator {
                        Operator::Add => acc + operand,
                        Operator::Multiply => acc * operand,
                        Operator::Concatenate => {
                            let mut number = acc.to_string();
                            number.push_str(operand.to_string().as_str());
                            number.parse::<u128>().unwrap()
                        }
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

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

struct Operators {
    len: usize,
    count: usize,
}

impl Operators {
    fn new(len: usize) -> Self {
        Operators { len, count: 0 }
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

struct Operators2 {
    len: usize,
    count: usize,
    operators: Vec<Operator>,
}

impl Operators2 {
    fn new(len: usize, operators: Vec<Operator>) -> Self {
        Operators2 {
            len,
            count: 0,
            operators,
        }
    }
}

impl Iterator for Operators2 {
    type Item = Vec<Operator>;

    fn next(&mut self) -> Option<Self::Item> {
        /*println!(
            "coutn: {}, lenpow: {}",
            self.count,
            self.operators.len().pow(self.len as u32)
        );*/
        if self.count > self.operators.len().pow(self.len as u32) - 1 {
            return None;
        }

        let mut result = vec![self.operators[0]; self.len];
        //println!("result: {result:?}");
        let mut count = self.count;
        let mut i = 0;
        loop {
            let remainder = count % self.operators.len();
            //println!("remainder: {remainder}");
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

/*
impl Iterator for Operators2 {
    type Item = Vec<Operator>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count > self.operators.len().pow(self.len as u32) {
            return None;
        }

        let mut number = String::new();
        let mut count = self.count;
        //println!("Count: {count}");
        loop {
            let remainder = count % self.operators.len();
            //println!("-----{remainder}--");
            number.push_str(&remainder.to_string());
            //println!("---------number: {number} len: {}", self.len);

            count /= self.operators.len();
            //println!("__Count: {count}");
            if count == 0 {
                break;
            }
        }
        number = format!("{:01$}", number.parse::<usize>().unwrap(), self.len);
        //println!("** '{number}'");
        self.count += 1;

        Some(
            number
                .chars()
                .map(|c| {
                    //println!("!!!!   {c}   !!!!!!!!");
                    self.operators[c.to_string().parse::<usize>().unwrap()]
                })
                .collect::<Vec<_>>(),
        )
    }
}
*/
