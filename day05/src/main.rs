use std::fs::read_to_string; 

fn is_nice(word: &str) -> bool {
    let mut vowels_cnt: u32 = 0;
    let mut doubles_cnt: u32 = 0;
    let nice_vowels: &str = "aeiou";

    let mut word_cpy = word.chars();
    word_cpy.next();

    for c in word.chars() {
        if nice_vowels.contains(c) {
            vowels_cnt += 1;
        }

        match word_cpy.next() {
            Some(next) => {
                if next == c {
                    doubles_cnt += 1;
                }
            },
            None => break,
        }
    }

    let stopwords = word.contains("ab") ||
        word.contains("cd") ||
        word.contains("pq") ||
        word.contains("xy");

    let niceword = vowels_cnt >= 3 && doubles_cnt >= 1 && stopwords == false;
    niceword
}

fn part1() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str = read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let words_vec: Vec<&str> = input_str.trim_end().split('\n').collect();
    let mut nicewords_cnt: u32 = 0;

    for word in words_vec.into_iter() {
        if is_nice(word) {
            nicewords_cnt += 1;
        }
    }

    println!("{}", nicewords_cnt);
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
