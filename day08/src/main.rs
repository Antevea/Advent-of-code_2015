use std::fs::read_to_string;

fn decoded_str_size_diff(s: &str) -> u32 {
    let mut escape_count = 0_u32;
    let s_vec = s.chars().collect::<Vec<char>>();
    let mut i = 1;
    while i < s_vec.len() - 1 {
        escape_count += 1;
        if s_vec[i] == '\\' {
            if s_vec[i + 1] == 'x' {
                i += 4;
            } else {
                i += 2;
            }
        } else {
            i += 1;
        }
    }

    s.len() as u32 - escape_count
}

fn part1() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str = read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let mut file_size_diff = 0_u32;

    for line in input_str.lines() {
        file_size_diff += decoded_str_size_diff(line);
    }

    println!("Decoded file size difference: {}", file_size_diff);
}

fn encoded_str_size_diff(s: &str) -> u32 {
    let mut escape_count = 0_u32;
    let s_vec = s.chars().collect::<Vec<char>>();
    let mut i = 0;
    while i < s_vec.len() {
        escape_count += 1;
        if s_vec[i] == '\\' {
            if s_vec[i + 1] == 'x' && (s_vec[i + 2].is_ascii() || s_vec[i + 2].is_numeric()) {
                escape_count += 1;
            } else {
                escape_count += 1;
            }
        } else if s_vec[i] == '\"' {
            escape_count += 1;
        }
        i += 1;
    }
    escape_count += 2;

    escape_count - s.len() as u32
}

fn part2() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str = read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let mut file_size_diff = 0_u32;

    for line in input_str.lines() {
        file_size_diff += encoded_str_size_diff(line);
    }

    println!("Encoded file size difference: {}", file_size_diff);
}

fn main() {
    part1();
    part2();
}
