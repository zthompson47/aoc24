use std::collections::HashMap;

fn main() {
    let mut blank_line = false;
    let mut device =
        include_str!("test0.txt")
            .lines()
            .fold(Device::default(), |mut device, line| {
                if line.is_empty() {
                    blank_line = true;
                } else if !blank_line {
                    let (wire, value) = line.split_once(": ").unwrap();
                    device.wires.insert(wire, Wire::from(value));
                } else {
                    let (gate, wire) = line.split_once(" -> ").unwrap();
                    let gate = gate.split_ascii_whitespace().collect::<Vec<_>>();
                    device.gates.push(Gate {
                        left: gate[0],
                        right: gate[2],
                        left_value: None,
                        right_value: None,
                        to_wire: wire,
                        operation: Operation::from(gate[1]),
                    });
                    device.wires.insert(wire, Wire::default());
                }
                device
            });

    // Index input destinations of wires.
    for i in 0..device.gates.len() {
        let gate = &device.gates[i];
        device
            .wires
            .entry(gate.left)
            .and_modify(|x| x.to_gates.push(i));
        device
            .wires
            .entry(gate.right)
            .and_modify(|x| x.to_gates.push(i));
    }

    println!("{device:?}");

    //while !device.is_done() {

    //}
}

#[derive(Default, Debug)]
struct Device<'a> {
    gates: Vec<Gate<'a>>,
    wires: HashMap<&'a str, Wire>,
}

impl Device<'_> {
    fn is_done(&self) -> bool {
        self.wires
            .iter()
            .filter(|(name, wire)| name.starts_with('z') && wire.value.is_none())
            .collect::<Vec<_>>()
            .is_empty()
    }
}

#[derive(Default, Debug)]
struct Wire {
    value: Option<bool>,
    to_gates: Vec<usize>,
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
        }
    }
}

#[derive(Debug)]
struct Gate<'a> {
    left: &'a str,
    right: &'a str,
    left_value: Option<bool>,
    right_value: Option<bool>,
    to_wire: &'a str,
    operation: Operation,
}

#[derive(Debug)]
enum Operation {
    And,
    Or,
    Xor,
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
