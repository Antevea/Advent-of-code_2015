use std::fs::read_to_string;

fn sum_of_all_numbers(json_str: &String) -> i64 {
    let mut str_iter = json_str.chars().into_iter();
    let mut sum = 0i64;

    while let Some(c) = str_iter.next_back() {
        let mut num_str = String::new();
        if c.is_digit(10) {
            num_str.insert(0, c);
            while let Some(next_c) = str_iter.next_back() {
                if next_c.is_ascii_digit() {
                    num_str.insert(0, next_c);
                } else if next_c == '-' {
                    num_str.insert(0, '-');
                } else {
                    break;
                }
            }
        }
        if !num_str.is_empty() {
            sum += num_str.parse::<i64>().unwrap();
        }
    }

    sum
}

fn part1() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str = read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    println!("Sum of all numbers in json file: {}", sum_of_all_numbers(&input_str));
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
