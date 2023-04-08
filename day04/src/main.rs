use std::fs::read_to_string;

fn part1() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str = read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let mut i: u64 = 0;
    let mut _digest: md5::Digest = md5::Digest::from(md5::Context::new());
    while i < u64::MAX {
        let secret_key = input_str.trim_end().to_string().clone() + i.to_string().as_str();
        i += 1;
        _digest = md5::compute(secret_key.as_bytes());
        let digest_str = format!("{:x}", _digest);
        if digest_str.starts_with("00000") {
            println!("{}", secret_key.clone());
            return;
        } else {
            continue;
        }
    }
    println!("Unable to find value with u64 MAX {}", u64::MAX);
}

fn main() {
    part1();
    // part2();
}
