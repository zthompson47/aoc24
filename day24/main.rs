#![allow(dead_code, unused_mut)]
use std::collections::{HashMap, HashSet};

fn main() {
    let mut device = Device::new();
    //println!("gates: {}", device.gates.len());
    //println!("wires: {}", device.wires.len());
    //println!("x wires: {}", device.wires.keys().filter(|x| x.starts_with("x")).count());
    //println!("y wires: {}", device.wires.keys().filter(|x| x.starts_with("y")).count());
    //println!("z wires: {}", device.wires.keys().filter(|x| x.starts_with("z")).count());
    //device.run();
    //println!("Part 1: {}", device.z());

    fn unique_trace(trace: Vec<String>) -> Vec<String> {
        let mut result = HashSet::new();
        for f in trace {
            result.insert(f);
        }
        let mut result = result.into_iter().collect::<Vec<_>>();
        result.sort();
        result
    }

    let wire = "x00";
    let forwardtrace = unique_trace(device.forwardtrace(wire));
    println!("forwardtrace {wire}: {forwardtrace:?}");

    let wire = "x44";
    let forwardtrace = unique_trace(device.forwardtrace(wire));
    println!("forwardtrace {wire}: {forwardtrace:?}");
    let wire = "y44";
    let forwardtrace = unique_trace(device.forwardtrace(wire));
    println!("forwardtrace {wire}: {forwardtrace:?}");

    // ( a    b    c     d    e    f
    // (tqq, ckb, ksv) (z06, z20, z39)
    let known_pairs = [
        [vec!["tqq", "z06"], vec!["ckb", "z20"], vec!["ksv", "z39"]].to_vec(),
        [vec!["tqq", "z06"], vec!["ckb", "z39"], vec!["ksv", "z20"]].to_vec(),
        [vec!["tqq", "z20"], vec!["ckb", "z06"], vec!["ksv", "z39"]].to_vec(),
        [vec!["tqq", "z20"], vec!["ckb", "z39"], vec!["ksv", "z06"]].to_vec(),
        [vec!["tqq", "z39"], vec!["ckb", "z06"], vec!["ksv", "z20"]].to_vec(),
        [vec!["tqq", "z39"], vec!["ckb", "z20"], vec!["ksv", "z06"]].to_vec(),
    ];
    let possible_wires = device
        .wires
        .keys()
        .filter(|x| {
            !x.starts_with("z")
                && !x.starts_with("x")
                && !x.starts_with("y")
                && !["tqq".to_string(), "ckb".to_string(), "ksv".to_string()].contains(x)
        })
        .map(|x| x.as_str())
        .collect::<Vec<_>>();
    //println!("possible wires: {possible_wires:?}");
    let other_pairs = all_groups(possible_wires, 2);
    //println!("other_pairs: {:?}", other_pairs);

    // cull loops

    println!("total: {}", other_pairs.len() * known_pairs.len());
    let mut i = 0;
    for other in other_pairs {
        #[allow(clippy::explicit_counter_loop)]
        for known in &known_pairs {
            i += 1;
            println!("{i}");
            let mut to_test = known.clone();
            to_test.push(other.clone());
            let mut test_device = device.clone();
            let mut gates = device.gates_bak.clone();
            for pair in to_test.iter() {
                let idx0 = gates
                    .iter()
                    .enumerate()
                    .find(|(_, x)| x.to_wire == pair[0])
                    .unwrap()
                    .0;
                let idx1 = gates
                    .iter()
                    .enumerate()
                    .find(|(_, x)| x.to_wire == pair[1])
                    .unwrap()
                    .0;
                //println!(
                //    "before {} {}",
                //    test_device.gates[idx0].to_wire, test_device.gates[idx1].to_wire
                //);
                //println!("idx0 idx1 {idx0} {idx1}");
                gates[idx0].to_wire = pair[1].to_string();
                gates[idx1].to_wire = pair[0].to_string();
                //println!(
                //    "after {} {}",
                //    test_device.gates[idx0].to_wire, test_device.gates[idx1].to_wire
                //);
            }
            //println!("{:?}", device.gates);
            //println!("{} {}", test_device.gates[70].to_wire, test_device.gates[71].to_wire);
            //println!("__________________________________________________");
            if test_device.test(Some(gates)) {
                println!("YES gates: {:?}", to_test.clone());
            }
        }
    }

    /*
    let mut odd_ducks = vec!["tqq", "ckb", "ksv", "z20", "z39", "z45", "z00", "z06"];
    odd_ducks.sort();
    println!("Part 2: {}", odd_ducks.join(","));
    let pairs = all_groups(odd_ducks, 2);
    for pair in pairs {
        println!("{pair:?}");
    }
    //let four_pairs = all_groups(pairs, 4);
    //println!("odd duck four pairs: {four_pairs:#?}");
    */

    /*
    if !device.test() {
        println!("FAIL");
    }

    let mut backtrace = device.backtrace("z10".to_string());
    backtrace.sort();
    let mut set = HashSet::new();
    for b in &backtrace {
        set.insert(b);
    }
    println!("{} {backtrace:?}", backtrace.len());
    println!("____________________");
    println!("{} {set:?}", set.len());

    let set: Vec<String> = vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
    ];
    println!("{:?}", all_groups(set, 3));

    let wire_groups = all_groups(
        device.wires.keys().take(15).map(|x| (*x).clone()).collect(),
        2,
    );
    println!("{}", wire_groups.len());
    let four_groups = all_groups(wire_groups, 4);
    println!("{}", four_groups.len());
    //println!("{four_groups:#?}");
    */
}

fn all_groups<T>(set: Vec<T>, count: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    //println!("___________________________");
    //println!("count: {count} set: {set:?}");

    /*
    if count == 0 {
        let end = vec![vec![set[0].clone()]];
        println!("count == 0, return {end:?}");
        return end;
    }
    */

    let mut result = Vec::new();
    if count == 1 {
        return set.into_iter().map(|x| vec![x]).collect();
    }
    for i in 0..set.len() - count + 1 {
        //println!("for {i} to {}", set.len() - count);
        for mut subset in all_groups(set[i + 1..].to_vec(), count - 1) {
            let mut first = vec![set[i].clone()];
            first.append(&mut subset);
            result.push(first);
        }
    }
    result
}

#[derive(Default, Debug, Clone)]
struct Device {
    gates: Vec<Gate>,
    wires: HashMap<String, Wire>,
    gates_bak: Vec<Gate>,
}

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone)]
struct Wire {
    value: Option<bool>,
    to_gates: Vec<usize>,
    completed: bool,
}

#[derive(Debug, Clone)]
struct Gate {
    left: String,
    right: String,
    left_value: Option<bool>,
    right_value: Option<bool>,
    to_wire: String,
    operation: Operation,
    completed: bool,
}

#[derive(Debug, Clone)]
enum Operation {
    And,
    Or,
    Xor,
}

impl Device {
    fn backtrace(&self, wire: String) -> Vec<String> {
        if wire.starts_with("x") || wire.starts_with("y") {
            return vec![];
        }

        let mut result = vec![wire.clone()];

        let gate = self.gates.iter().find(|x| x.to_wire == wire).unwrap();

        let (left, right) = (gate.left.clone(), gate.right.clone());

        result.append(&mut self.backtrace(left));
        result.append(&mut self.backtrace(right));

        result
    }

    fn forwardtrace(&self, wire: &str) -> Vec<String> {
        let mut result = vec![wire.to_string()];
        let gate_wires = self
            .gates
            .iter()
            .filter(|x| x.left == wire || x.right == wire)
            .map(|x| x.to_wire.clone())
            .collect::<Vec<_>>();
        for wire in gate_wires {
            result.append(&mut self.forwardtrace(&wire));
        }
        result
    }

    fn test(&mut self, gates: Option<Vec<Gate>>) -> bool {
        let mut pass = true;

        //for (x, y) in [(0, 0), (0, 1), (1, 0), (1, 1), (2, 1), (1, 2)] {
        //    self.reset(x, y);
        //    self.run();
        //    println!("{} + {} = {}", x, y, self.z());
        //}

        for i in 0..=44 {
            let x = 2u64.pow(i);
            //let x = 0;
            let y = 2u64.pow(i);
            //let y = 0;
            self.reset(x, y);
            if let Some(gates) = &gates {
                self.gates = gates.clone();
            }
            //println!("before run");
            if self.run() {
                //println!("after run");
                let z = self.z();
                //println!("[z] {:0>1$b}", z, 45);
                if x + y == z {
                    //println!("YUP: {} + {} = {}", x, y, z);
                } else {
                    pass = false;
                    //println!("__!ERR: {} + {} = {}", x, y, z);
                }
            } else {
                pass = false;
            }
        }

        /*
        for i in 0..=44 {
            let mut x = 0;
            let mut y = 0;
            for j in 0..=i {
                x += 2u64.pow(j);
                y += 2u64.pow(j);
            }
            self.reset(x, y);
            self.run();
            let z = self.z();
            println!("[z] {:0>1$b}", z, 45);
            if x + y == z {
                println!("YUP: {} + {} = {}", x, y, z);
            } else {
                pass = false;
                println!("ERR: {} + {} = {}", x, y, z);
            }
        }
        */

        pass
    }

    fn reset(&mut self, x: u64, y: u64) {
        self.gates = self.gates_bak.clone();

        for wire in self.wires.values_mut() {
            wire.value = None;
            wire.completed = false;
        }

        let number_len = self.wires.keys().filter(|x| x.starts_with('x')).count();

        //println!("---------->> {x} {y}");
        let x = format!("{:0>1$b}", x, number_len);
        let y = format!("{:0>1$b}", y, number_len);
        //println!("[x] {x}");
        //println!("[y] {y}");
        //println!("---------->> len:{} chars_count:{} {x} {y}", number_len, x.chars().count());

        for (i, digit) in x.chars().enumerate() {
            let key = format!("x{:0>2}", number_len - 1 - i);
            let wire = self.wires.get_mut(&key).unwrap();
            wire.value = Some(digit == '1');
        }
        for (i, digit) in y.chars().enumerate() {
            let key = format!("y{:0>2}", number_len - 1 - i);
            let wire = self.wires.get_mut(&key).unwrap();
            wire.value = Some(digit == '1');
        }
    }

    fn run(&mut self) -> bool {
        //let mut i = 0;
        let mut prior_wires_todo: Option<Vec<String>> = None;
        while !self.is_done() {
            let wires_todo = self
                .wires
                .iter()
                .filter(|(_, wire)| !wire.completed && wire.value.is_some())
                .map(|(name, _)| name.clone())
                .collect::<Vec<_>>();
            if let Some(prior) = &prior_wires_todo {
                //println!("MORE {} {}", wires_todo.len(), (*prior).len());
                if wires_todo == *prior {
                    return false;
                }
            }
            prior_wires_todo = Some(wires_todo.clone());
            for name in wires_todo {
                let wire = self.wires.get_mut(&name).unwrap();
                for gate_idx in wire.to_gates.clone() {
                    self.gates[gate_idx].set_operand(name.clone(), wire.value.unwrap());
                    wire.completed = true;
                }
            }
            let gates_todo = self
                .gates
                .iter()
                .enumerate()
                .filter(|(_, gate)| {
                    gate.left_value.is_some() && gate.right_value.is_some() && !gate.completed
                })
                .map(|(i, _)| i)
                .collect::<Vec<_>>();
            for gate_idx in gates_todo {
                let gate = &mut self.gates[gate_idx];
                let result = gate
                    .operation
                    .perform(gate.left_value.unwrap(), gate.right_value.unwrap());
                self.wires.get_mut(&gate.to_wire).unwrap().value = Some(result);
                gate.completed = true;
            }
            //i += 1;
        }
        //println!("iterations: {i}");
        true
    }

    fn z(&self) -> u64 {
        let mut z = self
            .wires
            .iter()
            .filter(|(name, _)| name.starts_with('z'))
            .collect::<Vec<_>>();
        z.sort_by(|(a, _), (b, _)| a.cmp(b));
        z.iter()
            .map(|(_, wire)| wire.value.unwrap())
            .enumerate()
            .map(|(i, value)| 2u64.pow(i as u32) * if value { 1 } else { 0 })
            .sum()
    }

    fn new() -> Self {
        let mut blank_line = false;
        let mut device =
            include_str!("input.txt")
                .lines()
                .fold(Device::default(), |mut device, line| {
                    if line.is_empty() {
                        blank_line = true;
                    } else if !blank_line {
                        let (wire, value) = line.split_once(": ").unwrap();
                        device.wires.insert(wire.to_string(), Wire::from(value));
                    } else {
                        let (gate, wire) = line.split_once(" -> ").unwrap();
                        let gate = gate.split_ascii_whitespace().collect::<Vec<_>>();
                        device.gates.push(Gate {
                            left: gate[0].to_string(),
                            right: gate[2].to_string(),
                            left_value: None,
                            right_value: None,
                            to_wire: wire.to_string(),
                            operation: Operation::from(gate[1]),
                            completed: false,
                        });
                        device.wires.insert(wire.to_string(), Wire::default());
                    }
                    device
                });

        // Index wire inputs.
        for i in 0..device.gates.len() {
            let gate = &device.gates[i];
            device
                .wires
                .entry(gate.left.clone())
                .and_modify(|x| x.to_gates.push(i));
            device
                .wires
                .entry(gate.right.clone())
                .and_modify(|x| x.to_gates.push(i));
        }

        // Save copy of logic gate layout for reset.
        device.gates_bak = device.gates.clone();

        device
    }

    fn is_done(&self) -> bool {
        self.wires
            .iter()
            .filter(|(name, wire)| name.starts_with('z') && wire.value.is_none())
            .collect::<Vec<_>>()
            .is_empty()
    }
}

impl From<&str> for Wire {
    fn from(value: &str) -> Self {
        Wire {
            value: Some(match value {
                "0" => false,
                "1" => true,
                _ => unreachable!(),
            }),
            to_gates: Vec::new(),
            completed: false,
        }
    }
}

impl Gate {
    fn set_operand(&mut self, name: String, value: bool) {
        if self.left == name {
            self.left_value = Some(value);
        }
        if self.right == name {
            self.right_value = Some(value);
        }
    }
}

impl Operation {
    fn perform(&self, left: bool, right: bool) -> bool {
        match self {
            Operation::And => left && right,
            Operation::Or => left || right,
            Operation::Xor => left ^ right,
        }
    }
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Operation::And,
            "OR" => Operation::Or,
            "XOR" => Operation::Xor,
            _ => unreachable!(),
        }
    }
}

/*
struct Choose {
    count: usize,
    set: Vec<String>,
    counter: Vec<usize>,
    first_time: bool,
}

impl Choose {
    fn new(count: usize, set: Vec<String>) -> Self {
        let mut counter = Vec::new();
        for i in 0..count {
            counter.push(i);
        }
        Choose {
            count,
            set,
            counter,
            first_time: true,
        }
    }

    fn end_counter(&self) -> Vec<usize> {
        let mut result = Vec::new();
        for i in 0..self.count {
            result.push(self.set.len() - self.count + i);
        }
        result
    }

    fn update_counter(&mut self) {
        let len = self.set.len();
        let mut counter_clone = self.counter.clone();
        counter_clone.reverse();
        for (i, value) in counter_clone.iter().enumerate() {
            if *value == len - 1 - i {
                continue;
            } else {
                self.counter[i] += 1;
            }
        }
    }
}

impl Iterator for Choose {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_time {
            self.first_time = false;
        } else if self.counter == self.end_counter() && !self.first_time {
            return None;
        } else {
            self.update_counter();
        }
        let mut result = Vec::new();
        for i in &self.counter {
            result.push(self.set[*i].clone());
        }
        Some(result)
    }
}
*/

/*
fn all_groups<'a>(set: &[&'a str], count: usize) -> Vec<Vec<&'a str>> {
    println!("_____________________{count} {set:?}");
    let mut result = Vec::new();
    let len = set.len();
    if count == 0 {
        for element in set {
            result.push(vec![*element]);
        }
    } else {
        for i in 0..len - count {
            println!("for {i} in {len}");
            let current = Vec::from(&set[i..i + count - 1]);
            println!("current {current:?}");
            for next_level in all_groups(&set[count - 1..], count - 1) {
                let mut to_add = current.clone();
                to_add.append(&mut next_level.clone());
                result.push(to_add);
            }
        }
    }
    result
}
*/
