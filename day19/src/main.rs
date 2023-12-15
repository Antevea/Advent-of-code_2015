use std::fs::read_to_string;
use std::collections::{HashMap, HashSet};

fn part1() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str = read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let mut combinations: HashMap<String, String> = HashMap::new();
    let mut molecule = String::new();
    let mut crafted_molecules: HashSet<String> = HashSet::new();

    for line in input_str.lines() {
        if let Some((left, right)) = line.split_once(" => ") {
            combinations.insert(right.to_string(), left.to_string());
        } else {
            molecule = line.to_string();
        }
    }

    for (key, val) in &combinations {
        for mol in molecule.match_indices(val) {
            let (left, right) = molecule.split_at(mol.0);
            let right = right.to_string().split_off(val.len());
            crafted_molecules.insert(format!("{left}{key}{right}"));
        }
    }

    println!("Part 1: {}", crafted_molecules.len());
}

fn part2() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str = read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let mut combinations: HashMap<String, String> = HashMap::new();
    let mut molecule = String::new();
    let mut count = 0;

    for line in input_str.lines() {
        if let Some((left, right)) = line.split_once(" => ") {
            combinations.insert(right.to_string(), left.to_string());
        } else {
            molecule = line.to_string();
        }
    }

    loop {
        let mut brek = true;
        for (key, val) in &combinations {
            if let Some(pos) = molecule.find(key) {
                let (left, right) = molecule.split_at(pos);
                let right = right.to_string().split_off(key.len());
                molecule = format!("{left}{val}{right}");
                brek = false;
                count += 1;
            }
        }
        if brek {
            break;
        }
    }

    println!("Part 2: {}", count);
}

fn main() {
    part1();
    part2();
}
