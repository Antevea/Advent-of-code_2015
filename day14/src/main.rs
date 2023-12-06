use std::fs::read_to_string;

fn part1() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str =
        read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let mut distances: Vec<u64> = Vec::new();

    for line in input_str.lines() {
        let tokens_vec = line.split(' ').collect::<Vec<&str>>();
        let race_time = 2503u64;
        let dist_remain;

        let speed = tokens_vec[3].parse::<u64>().unwrap();
        let fly_time = tokens_vec[6].parse::<u64>().unwrap();
        let fly_dist = fly_time * speed;
        let rest_time = tokens_vec[13].parse::<u64>().unwrap();

        let dist_base = fly_dist * (race_time / (fly_time + rest_time));
        let time_remaining = race_time % (fly_time + rest_time);
        if time_remaining > fly_time {
            dist_remain = speed * fly_time;
        } else {
            dist_remain = speed * time_remaining;
        }

        distances.push(dist_base + dist_remain);
    }

    let max_distance = distances.iter().max().unwrap();
    println!("Part1: {}", max_distance);
}

struct ReeinderRacer {
    speed: u64,
    fly_time: u64,
    rest_time: u64,
    distance: u64,
    points: u64,
}

impl ReeinderRacer {
    fn new(speed: u64, fly_time: u64, rest_time: u64) -> Self {
        Self {
            speed,
            fly_time,
            rest_time,
            distance: 0u64,
            points: 0u64,
        }
    }
}

fn award_da_leaders(reeinders: &mut Vec<ReeinderRacer>) {
    let max_distance = reeinders.iter().max_by_key(|r| r.distance).unwrap().distance;
    for reeinder in reeinders.iter_mut() {
        if reeinder.distance == max_distance {
            reeinder.points += 1;
        }
    }
}

fn part2() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str =
        read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let mut reeinders: Vec<ReeinderRacer> = Vec::new();
    let race_time = 2503;

    for line in input_str.lines() {
        let tokens_vec = line.split(' ').collect::<Vec<&str>>();

        let reeinder = ReeinderRacer::new(
            tokens_vec[3].parse::<u64>().unwrap(),
            tokens_vec[6].parse::<u64>().unwrap(),
            tokens_vec[13].parse::<u64>().unwrap(),
        );

        reeinders.push(reeinder);
    }

    for s in 0..race_time {
        for reeinder in reeinders.iter_mut() {
            let reeinder_phase = s % (reeinder.fly_time + reeinder.rest_time);
            if reeinder_phase < reeinder.fly_time {
                reeinder.distance += reeinder.speed;
            }
        }
        award_da_leaders(&mut reeinders);
    }

    let max_points = reeinders.iter()
        .max_by_key(|r| r.points)
        .unwrap()
        .points;

    println!("Part2: {}", max_points);
}

fn main() {
    part1();
    part2();
}
