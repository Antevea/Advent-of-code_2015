use std::fs::read_to_string;
use serde_json::Value;

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

fn sum_redless(val: &Value) -> Option<i64> {
    match val {
        Value::Null | Value::Bool(_) => Some(0),
        Value::String(s) => {
            if s == "red" {
                return None;
            } else {
                return Some(0);
            }
        },
        Value::Number(num) => Some(num.as_i64().unwrap()),
        Value::Array(arr) => Some(arr
                                  .iter()
                                  .map(|ent| sum_redless(ent).unwrap_or(0))
                                  .sum()),
        Value::Object(obj) => {
            let (vec_somes, vec_nones): (Vec<_>, Vec<_>) = obj
                .values()
                .map(|ent| sum_redless(ent))
                .partition(Option::is_some);

            if vec_nones.is_empty() {
                Some(vec_somes.iter().map(|ent| ent.unwrap()).sum())
            } else {
                return Some(0);
            }
        },
    }
}

fn part1() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str = read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    println!("Sum of all numbers in json file: {}", sum_of_all_numbers(&input_str));
}

fn part2() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str = read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let json: Value = serde_json::from_str(input_str.as_str()).expect(&format!("Error: Can't parse json file {}", filepath));
    println!("{}", sum_redless(&json).expect(&format!("Error: Can't count readless numbers in file {}", filepath)));
}

fn main() {
    part1();
    part2();
}
