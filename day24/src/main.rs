use std::fs::read_to_string;
use itertools::Itertools;

fn get_quantum_entanglement(gifts_vec: Vec<i64>, division: i64) -> Option<i64> {
    let mut brek = false;
    let mut qe = i64::MAX;
    let target = gifts_vec.iter().sum::<i64>() / division;

    for n in 1..gifts_vec.len() {
        for combo in gifts_vec.clone().into_iter().combinations(n) {
            let sum = combo.iter().sum::<i64>();
            if sum == target {
                qe = qe.min(combo.iter().product());
                brek = true;
            }
        }
        if brek == true {
            return Some(qe);
        }
    }

    None
}

fn solutions() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str = read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let mut gifts: Vec<i64> = Vec::new();
    for line in input_str.lines() {
        gifts.push(line
                   .trim()
                   .parse::<i64>()
                   .unwrap()
                   )
    }

    println!("Part 1: {}", get_quantum_entanglement(gifts.clone(), 3).unwrap());
    println!("Part 2: {}", get_quantum_entanglement(gifts, 4).unwrap());
}

fn main() {
    solutions();
}
