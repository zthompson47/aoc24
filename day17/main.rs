use std::{ops::BitXor, str::Lines};

fn main() {
    let mut computer = Computer::from(include_str!("input.txt"));
    //computer.a = 528;
    //computer.a = 37222273957364;
    // 0 => 1
    // 1 => 0
    // 2 => 1
    // 3 => 3
    // 4 => 5
    // 5 => 4
    // 6 => 7
    // 7 => 6
    //println!("{computer:?}");
    println!("Part 1: {}", computer.run());

    let mut a = 0;
    for level in 1..=16 {
        //println!("level:{level}");
        a *= 8;
        loop {
            //println!("  a:{a}");
            computer.reset(a);
            computer.run();
            //println!("  out:{:?}", computer.out);
            if computer.code[16 - level..] == computer.out {
                //println!("  FOUND a:{a}");
                break;
            }
            a += 1;
        }
    }
    println!("Part 2: {a}");

    //let mut i = 35184372088832;

    /*
    let mut i = 37222273957360;
    let mut computer = Computer::from(include_str!("input.txt"));
    loop {
        //for i in 277320..=2218560 {
        //for i in 4328..=2218560 {
        computer.reset(i);
        //computer.a = i;
        let _ = computer.run();

        //if computer.code == computer.out {
        // [0] => 1
        // [3, 0] => 8
        // [3, 3, 0] => 67
        // [0, 3, 3, 0] => 541
        // [5, 0, 3, 3, 0] => 4329
        // [5, 5, 0, 3, 3, 0] => 34665
        // [4, 5, 5, 0, 3, 3, 0] => 277327
        // [4, 4, 5, 5, 0, 3, 3, 0] => 2218620
        // [3, 4, 4, 5, 5, 0, 3, 3, 0] => 17748963
        // [1, 3, 4, 4, 5, 5, 0, 3, 3, 0] => 141991706
        // [5, 1, 3, 4, 4, 5, 5, 0, 3, 3, 0] => 1135933648
        // [7, 5, 1, 3, 4, 4, 5, 5, 0, 3, 3, 0] => 9087469184
        // [2, 7, 5, 1, 3, 4, 4, 5, 5, 0, 3, 3, 0] => 72699753822
        // [1, 2, 7, 5, 1, 3, 4, 4, 5, 5, 0, 3, 3, 0] => 581598030578
        // [4, 1, 2, 7, 5, 1, 3, 4, 4, 5, 5, 0, 3, 3, 0] => 4652784244670
        // [2, 4, 1, 2, 7, 5, 1, 3, 4, 4, 5, 5, 0, 3, 3, 0] => 37222273957364
        let end = &computer.out[computer.out.len() - 5..];
        println!("--> {end:?}");
        if computer.out[computer.out.len() - 16..] == [2, 4, 1, 2, 7, 5, 1, 3, 4, 4, 5, 5, 0, 3, 3, 0] {
            println!("!!!!!!!!  {i}");
            break;
        }

        //println!("{:?} {:?}", computer.code, computer.out);
        //println!("{i}");
        //println!("{}", computer.b);
        //if i % 100000 == 0 {
        //    println!("{i}");
        //}
        i += 1;
    }
    //println!("{i}");
    */

        /*
    let mut computer = Computer::from(include_str!("input.txt"));
    let mut a = 1;
    for i in 1..16 {
        loop {
            computer.reset(a);
            computer.run();
            if computer.out[..] == computer.code[computer.out.len() - i]
        }
    }
    */
}


#[derive(Debug)]
struct Computer {
    a: u64,
    b: u64,
    c: u64,
    code: Vec<u64>,
    out: Vec<u64>,
    ptr: u64,
}

impl Computer {
    fn run(&mut self) -> String {
        //println!("{:?}", self.code);
        //println!(
        //    "_,_ a:{:>8} b:{:>6} c:{:>6} ptr:{:>2} out:{:?}",
        //    self.a, self.b, self.c, self.ptr, self.out
        //);
        while self.ptr < self.code.len() as u64 - 1 {
            let opcode = self.code[self.ptr as usize];
            let operand = self.code[self.ptr as usize + 1];
            //if self.ptr == 0 {
            //    println!("_______________");
            //}
            self.execute(opcode, operand);
            //println!(
            //    "{opcode},{operand} a:{:>8} b:{:>6} c:{:>6} ptr:{:>2} out:{:?}",
            //    self.a, self.b, self.c, self.ptr, self.out
            //);
            if opcode != 3 {
                self.ptr += 2;
            }
        }

        self.out
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn execute(&mut self, opcode: u64, operand: u64) {
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

    fn combo(&self, operand: u64) -> u64 {
        match operand {
            i @ 0..=3 => i,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }

    fn reset(&mut self, i: u64) {
        self.a = i;
        self.b = 0;
        self.c = 0;
        self.ptr = 0;
        self.out.clear();
    }

    // 0
    fn adv(&mut self, operand: u64) {
        self.a /= 2_u64.pow(self.combo(operand) as u32);
    }

    // 1
    fn bxl(&mut self, operand: u64) {
        self.b = self.b.bitxor(operand);
    }

    // 2
    fn bst(&mut self, operand: u64) {
        self.b = self.combo(operand) % 8;
    }

    // 3
    fn jnz(&mut self, operand: u64) {
        if self.a != 0 {
            self.ptr = operand;
        } else {
            self.ptr += 2;
        }
    }

    // 4
    fn bxc(&mut self, _operand: u64) {
        self.b = self.b.bitxor(self.c);
    }

    // 5
    fn out(&mut self, operand: u64) {
        self.out.push(self.combo(operand) % 8);
    }

    // 6
    fn bdv(&mut self, operand: u64) {
        self.b = self.a / 2_u64.pow(self.combo(operand) as u32);
    }

    // 7
    fn cdv(&mut self, operand: u64) {
        self.c = self.a / 2_u64.pow(self.combo(operand) as u32);
    }
}

impl From<&str> for Computer {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        fn parse_register(lines: &mut Lines) -> u64 {
            lines
                .next()
                .unwrap()
                .split_once(": ")
                .unwrap()
                .1
                .parse::<u64>()
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
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<_>>()
            },
            out: Vec::new(),
            ptr: 0,
        }
    }
}
