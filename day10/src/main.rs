use std::fs::read_to_string;

fn count_and_say(init_seq: &[u8]) -> Vec<u8> {
    assert!(!init_seq.is_empty());

    let mut result = Vec::new();
    let mut current_number = init_seq[0];
    let mut current_runlength = 1;

    for i in &init_seq[1..] {
        if current_number == *i {
            current_runlength += 1;
        } else {
            result.push(current_runlength);
            result.push(current_number);
            current_runlength = 1;
            current_number = *i;
        }
    }
    result.push(current_runlength);
    result.push(current_number);
    result
}

fn part1() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str = read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let mut sequence = input_str.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();

    for _ in 0..40 {
        sequence = count_and_say(&sequence);
    }

    println!("Final length of the sequence {}", sequence.len());
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
