use std::fs::read_to_string; 

fn part1() {
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

fn part2() {
    fn has_twin_doublechars(word: &str) -> bool {
        let mut word_cpy = word.trim().chars();
        let mut prev_char = ' ';

        while let Some(word_char) = word_cpy.next() {
            let pair = format!("{}{}", prev_char, word_char);

            if word_cpy.as_str().contains(&pair) {
                return true;
            }
            prev_char = word_char.clone();
        }

        return false;
    }

    fn has_char_repeat(word: &str) -> bool {
        let mut word_cpy = word.trim().chars();
        while let Some(word_next_char) = word_cpy.next() {
            if let Some(rep) = word_cpy.as_str().chars().nth(1) {
                if word_next_char == rep {
                    return true;
                }
            }
        }
        return false;
    }

    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str = read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let words_vec: Vec<&str> = input_str.trim_end().split('\n').collect();
    let mut nicewords_cnt: u32 = 0;

    for word in words_vec.into_iter() {
        if has_char_repeat(word) && has_twin_doublechars(word) {
            nicewords_cnt += 1;
        }
    }

    println!("{}", nicewords_cnt);
}

fn main() {
    part1();
    part2();
}
