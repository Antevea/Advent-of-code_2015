use std::fs::read_to_string;
use std::collections::{HashSet, HashMap};

struct CityDistances {
    dist: HashMap<(String, String), u64>,
    cities: HashSet<String>,
}

impl CityDistances {
    fn new() -> Self {
        Self {
            dist: HashMap::new(),
            cities: HashSet::new(),
        }
    }

    fn insert_city_dist(&mut self, start: String, dest: String, dist: u64) {
        self.dist.insert((start, dest), dist);
    }
}

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

fn solution() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str = read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));
    let mut city_dist = CityDistances::new();

    for line in input_str.lines() {
        let (cities, distance_str) = line.split_once(" = ").unwrap();
        let (start, dest) = cities.split_once(" to ").unwrap();
        let distance_int = distance_str.parse::<u64>().unwrap();
        city_dist.insert_city_dist(start.to_string(), dest.to_string(), distance_int);
        city_dist.insert_city_dist(dest.to_string(), start.to_string(), distance_int);
        city_dist.cities.insert(start.to_string());
        city_dist.cities.insert(dest.to_string());
    }

    let mut city_list = city_dist
        .cities
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut permut = Permutations { list: Vec::new() };
    let city_list_len = city_list.len();
    permut.generate(&mut city_list, city_list_len);

    let mut shortest = u64::MAX;
    let mut longest = 0_u64;
    for p in permut.list.iter() {
        let mut dist_sum = 0;
        for pair in p.windows(2) {
            dist_sum += city_dist.dist.get(&(pair[0].to_string(), pair[1].to_string())).unwrap();
        }

        shortest = shortest.min(dist_sum);
        longest = longest.max(dist_sum);
    }

    println!("Shortest distance: {}", shortest);
    println!("Longest distance: {}", longest);
}

fn main() {
    solution();
}
