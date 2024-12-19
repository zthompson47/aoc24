use std::{ops::BitXor, str::Lines};

fn main() {
    let computer = Computer::from(include_str!("input.txt"));
    //println!("{computer:?}");
    println!("Part 1: {}", computer.run());
}

#[derive(Debug)]
struct Computer {
    a: u32,
    b: u32,
    c: u32,
    code: Vec<u32>,
    out: Vec<u32>,
    ptr: u32,
}

impl Computer {
    fn run(mut self) -> String {
        while self.ptr < self.code.len() as u32 - 1 {
            let opcode = self.code[self.ptr as usize];
            let operand = self.code[self.ptr as usize + 1];
            self.execute(opcode, operand);
            if opcode != 3 {
                self.ptr += 2;
            }
            //println!("opcode:{opcode} operand:{operand} {self:?}");
        }

        self.out
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn execute(&mut self, opcode: u32, operand: u32) {
        match opcode {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => unreachable!(),
        };
    }

    fn combo(&self, operand: u32) -> u32 {
        match operand {
            i @ 0..=3 => i,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }

    fn adv(&mut self, operand: u32) {
        self.a /= 2_u32.pow(self.combo(operand));
    }

    fn bxl(&mut self, operand: u32) {
        self.b = self.b.bitxor(operand);
    }

    fn bst(&mut self, operand: u32) {
        self.b = self.combo(operand) % 8;
    }

    fn jnz(&mut self, operand: u32) {
        if self.a != 0 {
            self.ptr = operand;
        } else {
            self.ptr += 2;
        }
    }

    fn bxc(&mut self, _operand: u32) {
        self.b = self.b.bitxor(self.c);
    }

    fn out(&mut self, operand: u32) {
        self.out.push(self.combo(operand) % 8);
    }

    fn bdv(&mut self, operand: u32) {
        self.b = self.a / 2_u32.pow(self.combo(operand));
    }

    fn cdv(&mut self, operand: u32) {
        self.c = self.a / 2_u32.pow(self.combo(operand));
    }
}

impl From<&str> for Computer {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        fn parse_register(lines: &mut Lines) -> u32 {
            lines
                .next()
                .unwrap()
                .split_once(": ")
                .unwrap()
                .1
                .parse::<u32>()
                .unwrap()
        }
        Computer {
            a: parse_register(&mut lines),
            b: parse_register(&mut lines),
            c: parse_register(&mut lines),
            code: {
                lines.next();
                lines
                    .next()
                    .unwrap()
                    .split_once(": ")
                    .unwrap()
                    .1
                    .split(',')
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            },
            out: Vec::new(),
            ptr: 0,
        }
    }
}
