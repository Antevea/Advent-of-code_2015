use std::fs::read_to_string;

fn is_pass_meets_requirements(pass_vec: &Vec<char>) -> bool {
    assert!(!pass_vec.is_empty());
    let mut flag_increasing_chars = false;
    let mut count_pair_chars = 0_u16;

    // Check if pass has increacing letters
    let alphabet = "abcdefghijklmnopqrstuvwxyz".to_string();
    let mut win_iter = pass_vec.windows(3);
    while let Some(win_three_slice) = win_iter.next() {
        let win_three_str = win_three_slice.iter().collect::<String>();
        if alphabet.contains(win_three_str.as_str()) {
            flag_increasing_chars = true;
        }
    }

    // Check if pass has pairs of letters
    let mut pairs_iter = pass_vec.windows(2);
    while let Some(pair) = pairs_iter.next() {
        if pair[0] == pair[1] {
            count_pair_chars += 1;
            pairs_iter.next();
        }
    }

    // Check if pass has banned char
    let banned_chars = "iol".to_string();
    let flag_ban_chars = pass_vec.iter().any(|c| banned_chars.contains(*c));

    flag_increasing_chars && !flag_ban_chars && count_pair_chars >= 2
}

fn iterate_password(init_vec: &Vec<char>) -> Vec<char> {
    assert!(!init_vec.is_empty());

    let c_vec = init_vec.iter().rev().map(|c| *c as u8).collect::<Vec<u8>>();
    let mut c_vec_iter = c_vec.iter();
    let mut accum_vec = Vec::new();

    if c_vec[0] != 'z' as u8 {
        accum_vec.push((*c_vec_iter.next().unwrap() + 1) as char);
        while let Some(next_c) = c_vec_iter.next() {
            accum_vec.push(*next_c as char);
        }
    } else {
        loop {
            if let Some(next_c) = c_vec_iter.next() {
                if *next_c == 'z' as u8 {
                    accum_vec.push('a');
                    continue;
                } else {
                    accum_vec.push((*next_c + 1) as char);
                    break;
                }
            } else {
                break;
            }
        }

        while let Some(next_c) = c_vec_iter.next() {
            accum_vec.push(*next_c as char);
        }
    }

    let next_pass_vec = accum_vec.iter().rev().map(|c| *c).collect::<Vec<char>>();
    next_pass_vec
}

fn solution() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str = read_to_string(filepath)
        .expect(&format!("Error: File {} not found!", filepath))
        .trim_end()
        .to_string();

    let mut pass_vec = input_str.chars().collect::<Vec<char>>();

    while is_pass_meets_requirements(&pass_vec) != true {
        pass_vec = iterate_password(&pass_vec);
    }
    // Answer for part 1
    println!("Santa password: {}", pass_vec.iter().collect::<String>());

    pass_vec = iterate_password(&pass_vec);
    while is_pass_meets_requirements(&pass_vec) != true {
        pass_vec = iterate_password(&pass_vec);
    }
    // Answer for part 2
    println!("Next santa password: {}", pass_vec.iter().collect::<String>());
}

fn main() {
    solution();
}
