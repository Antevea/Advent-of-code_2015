use std::fs::read_to_string;

fn part1() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str = read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let mut floor: i32 = 0;
    floor += input_str.matches('(').count() as i32;
    floor -= input_str.matches(')').count() as i32;
    println!("{floor}");
}

fn part2() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str = read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let mut floor: i32 = 0;
    let mut moves: i32 = 0;
    let mut updown = input_str.chars();
    while floor >= 0 {
        if updown.next().unwrap() == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
        moves += 1;
    }
    println!("{moves}");
}

fn main() {
    part1();
    part2();
}
