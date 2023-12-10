use std::fs::read_to_string;

fn part1() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str =
        read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let mut containers: Vec<usize> = Vec::new();
    for line in input_str.lines() {
        containers.push(line.trim().parse::<usize>().unwrap());
    }

    let max_num = 1_usize << containers.len();
    let num_can_fit = (0..max_num).filter(|x|
        containers.iter()
        .enumerate()
        .map(|(idx, c)| c * ((x >> idx) & 1))
        .sum::<usize>() == 150
    ).count();

    println!("Part 1: {}", num_can_fit);
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
