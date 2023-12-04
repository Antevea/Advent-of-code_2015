use std::fs::read_to_string;
use std::collections::{HashMap, HashSet};

struct Permutations<T> {
    list: Vec<Vec<T>>,
}

impl<T: Clone> Permutations<T> {
    // Heap permutation algorithm with recursion for vector of elements
    fn generate(&mut self, vec: &mut Vec<T>, size: usize) {
        if size == 1 {
            self.list.push(vec.to_owned());
        } else {
            for i in 0..size {
                self.generate(vec, size - 1);
                if size % 2 == 0 {
                    vec.swap(i, size - 1);
                } else {
                    vec.swap(0, size - 1);
                }
            }
        }
    }
}

struct TableHappiness {
    data: HashMap<(String, String), i64>,
    people: HashSet<String>,
}

impl TableHappiness {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
            people: HashSet::new(),
        }
    }

    fn parse(&mut self, input_str: String) {
        for line in input_str.lines() {
            let ary = line.split(' ').collect::<Vec<&str>>();
            let first = ary[0].to_string();
            let sign = if ary[2] == "gain" { 1 } else { -1 };
            let hap = ary[3].parse::<i64>().unwrap();
            let mut second = ary[10].to_string();

            // remove full stop at end
            second.pop();

            self.people.insert(first.clone());
            self.people.insert(second.clone());
            self.data.insert((first, second), sign * hap);
        }
    }

    fn find_happiest(&self) -> i64 {
        let mut all = self
            .people
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let last_person = all.pop().unwrap();
        let all_len = all.len();

        let mut max_happiness = 0;
        let mut permut = Permutations { list: Vec::new() };
        permut.generate(&mut all, all_len);
        for entry in permut.list.iter() {
            let mut entry = entry.clone();
            entry.push(last_person.clone());
            max_happiness = self.happiness(entry).max(max_happiness);
        }
        max_happiness
    }

    fn happiness(&self, v: Vec<String>) -> i64 {
        let mut result = 0;
        let wrap = v.len() - 1;

        let first = (v[0].clone(), v[1].clone());
        let last = (v[0].clone(), v[wrap].clone());

        // first
        result += self.data.get(&first).unwrap_or(&0) + self.data.get(&last).unwrap_or(&0);

        // middle
        for entry in 1..wrap {
            let one = (v[entry].clone(), v[entry - 1].clone());
            let two = (v[entry].clone(), v[entry + 1].clone());

            result += self.data.get(&one).unwrap_or(&0) + self.data.get(&two).unwrap_or(&0);
        }

        let first = (v[wrap].clone(), v[wrap - 1].clone());
        let last = (v[wrap].clone(), v[0].clone());

        // last
        result += self.data.get(&first).unwrap_or(&0) + self.data.get(&last).unwrap_or(&0);

        result
    }
}

fn solutions() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str = read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let mut table = TableHappiness::new();

    table.parse(input_str);
    println!("First part: {}", table.find_happiest());
    table.people.insert("Myself".to_string());
    println!("Second part: {}", table.find_happiest());
}

fn main() {
    solutions();
}
