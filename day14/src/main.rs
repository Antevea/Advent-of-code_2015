use std::fs::read_to_string;

fn part1() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str =
        read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let mut distances: Vec<u64> = Vec::new();

    for line in input_str.lines() {
        let input_vec = line.split(' ').collect::<Vec<&str>>();
        let time = 2503u64;
        let dist_remain;

        let speed = input_vec[3].parse::<u64>().unwrap();
        let fly_time = input_vec[6].parse::<u64>().unwrap();
        let fly_dist = fly_time * speed;
        let rest_time = input_vec[13].parse::<u64>().unwrap();

        let dist_base = fly_dist * (time / (fly_time + rest_time));
        let time_remaining = time % (fly_time + rest_time);
        if time_remaining > fly_time {
            dist_remain = speed * fly_time;
        } else {
            dist_remain = speed * time_remaining;
        }

        distances.push(dist_base + dist_remain);
    }

    println!("{}", distances.iter().max().unwrap());
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
