use std::collections::HashMap;
use std::fs::read_to_string;

fn handle_encabulator_case(param: &str,
                           value: u32,
                           reference_tape: &HashMap<&str, u32>)
                           -> bool {
    match param {
        "cats" | "trees" => {
            if value > reference_tape[param] {
                true
            } else {
                false
            }
        },
        "pomeranians" | "goldfish" => {
            if value < reference_tape[param] {
                true
            } else {
                false
            }
        },
        _ => {
            if value == reference_tape[param] {
                true
            } else {
                false
            }
        },
    }
}

fn solution() {
    let filepath = "puzzle";
    let input_str =
        read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let reference_tape: HashMap<&str, u32> = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);

    for line in input_str.lines() {
        let cleanline = line.replace(":", "").replace(",", "");
        let tokens = cleanline.split(' ').skip(1).collect::<Vec<&str>>();

        let param1 = tokens[1];
        let val1 = tokens[2].parse::<u32>().unwrap();
        let param2 = tokens[3];
        let val2 = tokens[4].parse::<u32>().unwrap();
        let param3 = tokens[5];
        let val3 = tokens[6].parse::<u32>().unwrap();

        if val1 == reference_tape[param1]
            && val2 == reference_tape[param2]
            && val3 == reference_tape[param3]
        {
            println!("Part 1: {}", tokens[0]);
        }

        let first = handle_encabulator_case(param1, val1, &reference_tape);
        let second = handle_encabulator_case(param2, val2, &reference_tape);
        let third = handle_encabulator_case(param3, val3, &reference_tape);

        if first == true && second == true && third == true {
            println!("Part 2: {}", tokens[0]);
        }
    }
}

fn main() {
    solution();
}
