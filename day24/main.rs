use std::collections::HashMap;

fn main() {
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

    // Index input destinations of wires.
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

    //println!("{device:?}");

    let mut i = 0;
    #[allow(clippy::never_loop)]
    while !device.is_done() {
        println!("{:?}", device);
        let wires_todo = device
            .wires
            .iter()
            .filter(|(name, wire)| !wire.completed && wire.value.is_some())
            .map(|(name, _)| name.clone())
            .collect::<Vec<_>>();
        for name in wires_todo {
            println!("wire: {name}");
            let wire = device.wires.get_mut(&name).unwrap();
            for gate_idx in wire.to_gates.clone() {
                device.gates[gate_idx].set_operand(name.clone(), wire.value.unwrap());
                wire.completed = true;
            }
        }
        println!("{:?}", device);

        let gates_todo = device
            .gates
            .iter()
            .enumerate()
            .filter(|(_, gate)| {
                gate.left_value.is_some() && gate.right_value.is_some() && !gate.completed
            })
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        for gate_idx in gates_todo {
            println!("gate: {gate_idx}");
            let gate = &mut device.gates[gate_idx];
            let result = gate
                .operation
                .perform(gate.left_value.unwrap(), gate.right_value.unwrap());
            device.wires.get_mut(&gate.to_wire).unwrap().value = Some(result);
            gate.completed = true;
        }
        println!("{:?}", device);

        println!("___________________________________");
        if i == 1 {
            //break;
        }
        i += 1;
    }

    let mut z = device
        .wires
        .iter()
        .filter(|(name, _)| name.starts_with('z'))
        .collect::<Vec<_>>();
    z.sort_by(|(a, _), (b, _)| a.cmp(b));
    let part1: u64 = z
        .iter()
        .map(|(_, wire)| wire.value.unwrap())
        .enumerate()
        .map(|(i, value)| 2u64.pow(i as u32) * if value { 1 } else { 0 })
        .sum();
    println!("Part 1: {part1}");
}

#[derive(Default, Debug)]
struct Device {
    gates: Vec<Gate>,
    wires: HashMap<String, Wire>,
}

#[derive(Default, Debug, Hash, PartialEq, Eq)]
struct Wire {
    value: Option<bool>,
    to_gates: Vec<usize>,
    completed: bool,
}

#[derive(Debug)]
struct Gate {
    left: String,
    right: String,
    left_value: Option<bool>,
    right_value: Option<bool>,
    to_wire: String,
    operation: Operation,
    completed: bool,
}

impl Device {
    fn is_done(&self) -> bool {
        self.wires
            .iter()
            .filter(|(name, wire)| {
                //println!("wtf {name} {wire:?}");
                name.starts_with('z') && wire.value.is_none()
            })
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
        //if let (Some(left), Some(right)) = (self.left_value, self.right_value) {
        //    return Some(self.operation.perform(left, right));
        //}
        //None
    }
}

#[derive(Debug)]
enum Operation {
    And,
    Or,
    Xor,
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
let mut completed_wires: HashSet<&str> = HashSet::new();
while !device.is_done() {
    let mut inner_completed_wires = completed_wires.clone();
    for name in inner_completed_wires.iter().filter(|x| {
        device.wires.get(**x).unwrap().value.is_some() && !inner_completed_wires.contains(**x)
    }) {
        let wire = device.wires.get(name).unwrap();
        for gate in wire.to_gates.iter() {
            if let Some(result) = device.gates[*gate].set_value(name, wire.value.unwrap()) {
                device.wires.get_mut(name).unwrap().value = Some(result);
                completed_wires.insert(name);
            }
        }
    }
}
*/
