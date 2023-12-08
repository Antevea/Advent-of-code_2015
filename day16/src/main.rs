use std::collections::HashMap;
use std::fs::read_to_string;

fn part1() {
    let filepath = "puzzle";
    let input_str =
        read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let reference_tape = HashMap::from([
        ("children", "3"),
        ("cats", "7"),
        ("samoyeds", "2"),
        ("pomeranians", "3"),
        ("akitas", "0"),
        ("vizslas", "0"),
        ("goldfish", "5"),
        ("trees", "3"),
        ("cars", "2"),
        ("perfumes", "1"),
    ]);

    for line in input_str.lines() {
        let cleanline = line.replace(":", "").replace(",", "");
        let tokens = cleanline.split(' ').skip(1).collect::<Vec<&str>>();

        let param1 = tokens[1];
        let val1 = tokens[2];
        let param2 = tokens[3];
        let val2 = tokens[4];
        let param3 = tokens[5];
        let val3 = tokens[6];

        if val1 == reference_tape[param1]
            && val2 == reference_tape[param2]
            && val3 == reference_tape[param3]
        {
            println!("Part 1: {}", tokens[0]);
            break;
        }
    }
}

/* fn part2() {
    let filepath = "sample";
    // let filepath = "puzzle";
    let input_str = read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));
} */

fn main() {
    part1();
    // part2();
}
