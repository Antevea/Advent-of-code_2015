use std::fs::read_to_string;

fn part1() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str = read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let presents_target = input_str.trim().parse::<u64>().unwrap();
    let mut houses = vec![0u64; 1000000];

    'outer : for elf in 1..1000000 {
        for i in (elf..1000000).step_by(elf) {
            houses[i] += elf as u64 * 10;
            if houses[i] >= presents_target {
                println!("Part 1: {}", i);
                break 'outer;
            }
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
