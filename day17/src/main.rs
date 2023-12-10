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
        .map(|(ind, c)| c * ((x >> ind) & 1))
        .sum::<usize>() == 150
    ).count();

    println!("Part 1: {}", num_can_fit);
}

fn part2() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str =
        read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let mut containers: Vec<usize> = Vec::new();
    for line in input_str.lines() {
        containers.push(line.trim().parse::<usize>().unwrap());
    }

    let max_num = 1_usize << containers.len();
    let num_containers = (0..max_num).filter_map(|x|
        if containers.iter()
                .enumerate()
                .map(|(ind, c)| c * ((x >> ind) & 1))
                .sum::<usize>() == 150
        {
            Some(x.count_ones() as usize)
        }
        else {
            None
        }
    ).collect::<Vec<usize>>();

    let min_num_containers = num_containers.iter().min().unwrap();
    println!("Part 2: {}", num_containers.iter().filter(|&n| n == min_num_containers).count());
}

fn main() {
    part1();
    part2();
}
