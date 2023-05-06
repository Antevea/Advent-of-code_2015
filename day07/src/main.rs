use std::fs::read_to_string;
use std::collections::HashMap;
use std::cell::RefCell;

#[derive(Debug)]
enum VarOptions {
    Var(String),
    Num(u16),
}

impl From<String> for VarOptions {
    fn from(s: String) -> Self {
        if let Ok(num) = s.parse::<u16>() {
            VarOptions::Num(num)
        } else {
            VarOptions::Var(s)
        }
    }
}

impl From<&String> for VarOptions {
    fn from(s: &String) -> Self {
        if let Ok(num) = s.parse::<u16>() {
            VarOptions::Num(num)
        } else {
            VarOptions::Var(s.clone())
        }
    }
}

#[derive(Debug)]
enum Command {
    NUM(u16),
    VAR(String),
    AND(VarOptions, VarOptions),
    OR(String, String),
    LSHIFT(String, u16),
    RSHIFT(String, u16),
    NOT(String)
}

struct CircutBoard {
    commands: HashMap<String, Command>,
    wires: RefCell<HashMap<String, u16>>,
    part1_wire_a: u16,
}

impl CircutBoard {
    fn new() -> Self {
        // let filepath = "sample";
        let filepath = "puzzle";
        let input_str = read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));
        let instr_vec: Vec<&str> = input_str.lines().collect::<Vec<&str>>();
        let mut commands: HashMap<String, Command> = HashMap::new();

        for line in instr_vec {
            let (l, r) = line.split_once(" -> ").unwrap();
            let r = r.to_string();
            let comm = l.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();

            let command = if comm.len() == 1 && comm[0].chars().next().unwrap().is_numeric() {
                Command::NUM(comm[0].parse::<u16>().unwrap())
            } else if comm.len() == 1 {
                Command::VAR(comm[0].clone())
            } else if comm[0] == "NOT" {
                Command::NOT(comm[1].to_string())
            } else {
                    match comm[1].as_str() {
                    "RSHIFT" => Command::RSHIFT(comm[0].clone(), comm[2].parse::<u16>().unwrap()),
                    "LSHIFT" => Command::LSHIFT(comm[0].clone(), comm[2].parse::<u16>().unwrap()),
                    "AND" => Command::AND(comm[0].clone().into(), comm[2].clone().into()),
                    "OR" => Command::OR(comm[0].clone().into(), comm[2].clone().into()),
                    _ => panic!("ERROR: Corrupted input data"),
                }
            };
            commands.insert(r, command);
        }
        Self {
            commands,
            wires: RefCell::new(HashMap::new()),
            part1_wire_a: 0,
        }
    }

    fn get_value(&self, var: &VarOptions) -> u16 {
        let var = match var {
            VarOptions::Var(s) => s,
            VarOptions::Num(n) => return *n,
        };

        if let Some(value) = self.wires.borrow().get(var) {
            return *value;
        }

        let command = self.commands.get(var).unwrap();
        let value = match command {
            Command::NUM(num) => *num,
            Command::VAR(name) => self.get_value(&name.into()),
            Command::AND(left, right) => {
                let left = left.clone();
                let right = right.clone();

                let left = self.get_value(&left);
                let right = self.get_value(&right);
                left & right
            }
            Command::OR(left, right) => {
                let left = left.clone();
                let right = right.clone();

                let left = self.get_value(&left.into());
                let right = self.get_value(&right.into());
                left | right
            }
            Command::LSHIFT(left, offset) => {
                let offset = *offset;
                let left = self.get_value(&left.clone().into());
                left << offset
            }
            Command::RSHIFT(right, offset) => {
                let offset = *offset;
                let right = self.get_value(&right.clone().into());
                right >> offset
            }
            Command::NOT(name) => {
                let value = self.get_value(&name.clone().into());
                !value
            }
        };

        self.wires.borrow_mut().insert(var.clone(), value);
        value
    }

    fn set_b(&self, value: u16) {
        self.wires.borrow_mut().clear();
        self.wires.borrow_mut().insert("b".to_string().into(), value);
    }
}

fn part1_and_part2() {
    let mut cb = CircutBoard::new();
    let part1_answer = cb.get_value(&VarOptions::Var("a".to_string()));
    cb.part1_wire_a = part1_answer;
    println!("Part 1: Wire \'a\' has signal: {}", part1_answer);

    cb.set_b(part1_answer);
    let part2_answer = cb.get_value(&VarOptions::Var("a".to_string()));
    println!("Part 2: Wire \'a\' has signal: {}", part2_answer);
}

fn main() {
    part1_and_part2();
}
