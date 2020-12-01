use regex::Regex;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub enum Wire {
    Constant(u16),
    Connection(String),
}

impl Wire {
    fn new(source: &str) -> Self {
        if let Ok(num) = source.parse::<u16>() {
            Wire::Constant(num)
        } else {
            Wire::Connection(source.to_owned())
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    Constant { input: u16 },
    PassThrough { input: Wire },
    Not { input: Wire },
    And { input_1: Wire, input_2: Wire },
    Or { input_1: Wire, input_2: Wire },
    LShift { input: Wire, shift: u16 },
    RShift { input: Wire, shift: u16 },
}

#[derive(Debug, Default, Clone)]
pub struct Circuit {
    wires: HashMap<Wire, Operation>,
}

lazy_static! {
    static ref AND_CHECK: Regex = Regex::new(r"^(\w+) AND (\w+) -> (\w+)$").unwrap();
    static ref OR_CHECK: Regex = Regex::new(r"^(\w+) OR (\w+) -> (\w+)$").unwrap();
    static ref NOT_CHECK: Regex = Regex::new(r"^NOT (\w+) -> (\w+)$").unwrap();
    static ref CONST_CHECK: Regex = Regex::new(r"^(\d+) -> (\w+)$").unwrap();
    static ref LSHIFT_CHECK: Regex = Regex::new(r"^(\w+) LSHIFT (\d+) -> (\w+)$").unwrap();
    static ref RSHIFT_CHECK: Regex = Regex::new(r"^(\w+) RSHIFT (\d+) -> (\w+)$").unwrap();
    static ref PASS_THROUGH_CHECK: Regex = Regex::new(r"^(\w+) -> (\w+)$").unwrap();
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Circuit {
    let wires = input
        .lines()
        .map(|l| l.trim())
        .fold(HashMap::new(), |mut acc, line| {
            if let Some(cap) = AND_CHECK.captures(line) {
                let op = Operation::And {
                    input_1: Wire::new(cap.get(1).unwrap().as_str()),
                    input_2: Wire::new(cap.get(2).unwrap().as_str()),
                };
                let wire = Wire::new(cap.get(3).unwrap().as_str());
                acc.insert(wire, op);
            } else if let Some(cap) = OR_CHECK.captures(line) {
                let op = Operation::Or {
                    input_1: Wire::new(cap.get(1).unwrap().as_str()),
                    input_2: Wire::new(cap.get(2).unwrap().as_str()),
                };
                let wire = Wire::new(cap.get(3).unwrap().as_str());
                acc.insert(wire, op);
            } else if let Some(cap) = NOT_CHECK.captures(line) {
                let op = Operation::Not {
                    input: Wire::new(cap.get(1).unwrap().as_str()),
                };
                let wire = Wire::new(cap.get(2).unwrap().as_str());
                acc.insert(wire, op);
            } else if let Some(cap) = CONST_CHECK.captures(line) {
                let op = Operation::Constant {
                    input: cap.get(1).unwrap().as_str().parse().unwrap(),
                };
                let wire = Wire::new(cap.get(2).unwrap().as_str());
                acc.insert(wire, op);
            } else if let Some(cap) = LSHIFT_CHECK.captures(line) {
                let op = Operation::LShift {
                    input: Wire::new(cap.get(1).unwrap().as_str()),
                    shift: cap.get(2).unwrap().as_str().parse().unwrap(),
                };
                let wire = Wire::new(cap.get(3).unwrap().as_str());
                acc.insert(wire, op);
            } else if let Some(cap) = RSHIFT_CHECK.captures(line) {
                let op = Operation::RShift {
                    input: Wire::new(cap.get(1).unwrap().as_str()),
                    shift: cap.get(2).unwrap().as_str().parse().unwrap(),
                };
                let wire = Wire::new(cap.get(3).unwrap().as_str());
                acc.insert(wire, op);
            } else if let Some(cap) = PASS_THROUGH_CHECK.captures(line) {
                let op = Operation::PassThrough {
                    input: Wire::new(cap.get(1).unwrap().as_str()),
                };
                let wire = Wire::new(cap.get(2).unwrap().as_str());
                acc.insert(wire, op);
            };
            acc
        });
    Circuit { wires }
}

fn find_wire_value(circuit: Arc<RwLock<Circuit>>, wire: &Wire) -> u16 {
    if let Wire::Constant(num) = wire {
        return *num;
    }
    let wire_to_check = if let Ok(c) = circuit.read() {
        c.wires.get(wire).unwrap().clone()
    } else {
        panic!("Unable to find wire: {:?}", wire);
    };
    let result = match wire_to_check {
        Operation::Constant { input } => input,
        Operation::PassThrough { input } => find_wire_value(circuit.clone(), &input),
        Operation::Not { input } => !find_wire_value(circuit.clone(), &input),
        Operation::And { input_1, input_2 } => {
            find_wire_value(circuit.clone(), &input_1) & find_wire_value(circuit.clone(), &input_2)
        }
        Operation::Or { input_1, input_2 } => {
            find_wire_value(circuit.clone(), &input_1) | find_wire_value(circuit.clone(), &input_2)
        }
        Operation::LShift { input, shift } => find_wire_value(circuit.clone(), &input) << shift,
        Operation::RShift { input, shift } => find_wire_value(circuit.clone(), &input) >> shift,
    };

    if let Ok(mut c) = circuit.write() {
        c.wires
            .insert(wire.clone(), Operation::Constant { input: result });
    }
    result
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Circuit) -> i32 {
    let sharing_circuit = Arc::new(RwLock::new(input.clone()));
    find_wire_value(sharing_circuit.clone(), &Wire::new("a")) as i32
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Circuit) -> i32 {
    let part1_solution = solve_part1(input);
    let sharing_circuit = Arc::new(RwLock::new(input.clone()));
    if let Ok(mut c) = sharing_circuit.write() {
        c.wires.insert(
            Wire::new("b"),
            Operation::Constant {
                input: part1_solution as u16,
            },
        );
    }

    find_wire_value(sharing_circuit.clone(), &Wire::new("a")) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let sample_data = "123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i";

        let test_cases = vec![
            ("d", 72),
            ("e", 507),
            ("f", 492),
            ("g", 114),
            ("h", 65412),
            ("i", 65079),
            ("x", 123),
            ("y", 456),
        ];

        let parsed_data = Arc::new(RwLock::new(input_generator(sample_data)));

        println!("Parsed Data: {:?}", parsed_data);

        for (wire, result) in test_cases {
            assert_eq!(
                result,
                find_wire_value(parsed_data.clone(), &Wire::new(wire)),
                "Comparing wire {} to {}",
                wire,
                result
            )
        }
    }
}
