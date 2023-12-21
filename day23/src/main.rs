use std::fs::read_to_string;

fn solution(reg_a_init_val: u64) -> u64 {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str = read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let mut reg_a = reg_a_init_val;
    let mut reg_b = 0u64;
    let mut instr_ind = 0usize;
    let instr_vec = input_str.lines().map(|l| l.to_string()).collect::<Vec<String>>();
    let instr_size = instr_vec.len();

    while instr_ind < instr_size {
        let instr_line = instr_vec[instr_ind].clone();
        let instruction = instr_line.split_once(' ').unwrap().0;
        match instruction {
            "hlf" => {
                let reg = instr_line.split_once(' ').unwrap().1;
                match reg {
                    "a" => reg_a = reg_a / 2,
                    "b" => reg_b = reg_b / 2,
                    _ => println!("Error: unknown register `{}` on line {}", reg, instr_ind),
                }

                instr_ind += 1;
            },
            "tpl" => {
                let reg = instr_line.split_once(' ').unwrap().1;
                match reg {
                    "a" => reg_a = reg_a * 3,
                    "b" => reg_b = reg_b * 3,
                    _ => println!("Error: unknown register `{}` on line {}", reg, instr_ind),
                }

                instr_ind += 1;
            },
            "inc" => {
                let reg = instr_line.split_once(' ').unwrap().1;
                match reg {
                    "a" => reg_a += 1,
                    "b" => reg_b += 1,
                    _ => println!("Error: unknown register `{}` on line {}", reg, instr_ind),
                }

                instr_ind += 1;
            },
            "jmp" => {
                let offset_str = instr_line.split_once(' ')
                    .unwrap()
                    .1
                    .to_string();
                let offset_size = offset_str.chars()
                    .skip(1)
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();

                if offset_str.chars().take(1).all(|s| s == '+')  {
                    instr_ind += offset_size;
                } else {
                    instr_ind -= offset_size;
                }
            },
            "jie" => {
                let reg = instr_line.split(' ').nth(1).unwrap();
                match reg {
                    "a," => {
                        if reg_a % 2 != 0 {
                            instr_ind += 1;
                            continue;
                        }
                    },
                    "b," => {
                        if reg_b % 2 != 0 {
                            instr_ind += 1;
                            continue;
                        }
                    },
                    _ => println!("Error: unknown register `{}` on line {}", reg, instr_ind),
                }

                let offset_str = instr_line.split(' ').last().unwrap();
                let offset_size = offset_str.chars()
                    .skip(1)
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();

                if offset_str.chars().take(1).all(|s| s == '+') {
                    instr_ind += offset_size;
                } else {
                    instr_ind -= offset_size;
                }
            },
            "jio" => {
                let reg = instr_line.split(' ').nth(1).unwrap();
                match reg {
                    "a," => {
                        if reg_a != 1 {
                            instr_ind += 1;
                            continue;
                        }
                    },
                    "b," => {
                        if reg_b != 1 {
                            instr_ind += 1;
                            continue;
                        }
                    },
                    _ => println!("Error: unknown register `{}` on line {}", reg, instr_ind),
                }

                let offset_str = instr_line.split(' ').last().unwrap();
                let offset_size = offset_str.chars()
                    .skip(1)
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();

                if offset_str.chars().take(1).all(|s| s == '+') {
                    instr_ind += offset_size;
                } else {
                    instr_ind -= offset_size;
                }
            },
            _ => {},
        }
    }
    reg_b
}

fn main() {
    let part1 = solution(0);
    let part2 = solution(1);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
