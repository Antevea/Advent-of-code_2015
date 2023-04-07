use std::fs::read_to_string;

fn part1() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str = read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let input_str_split: Vec<&str> = input_str
        .trim_end()
        .split('\n')
        .collect();

    let mut paper_order: u32 = 0;

    for box_sides in input_str_split.into_iter() {
        let sides: Vec<u32> = box_sides
            .split('x')
            .map(|elem| elem.parse::<u32>().unwrap())
            .collect();

        let first_side = sides[0] * sides[1];
        let second_side = sides[1] * sides[2];
        let third_side = sides[2] * sides[0];
        let smallest_side = [first_side, second_side, third_side].iter().min().unwrap().to_owned();
        paper_order += (first_side * 2) + (second_side * 2) + (third_side * 2) + smallest_side;
    }

    println!("{}", paper_order);
}

fn part2() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str = read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let input_str_split: Vec<&str> = input_str
        .trim_end()
        .split('\n')
        .collect();

    let mut ribon_order: u32 = 0;

    for box_sides in input_str_split.into_iter() {
        let mut sides: Vec<u32> = box_sides
            .split('x')
            .map(|elem| elem.parse::<u32>().unwrap())
            .collect();

        sides.sort();

        let ribon = sides[0] + sides[0] + sides[1] + sides[1];
        let bow = sides[0] * sides[1] * sides[2];
        ribon_order += ribon + bow;
    }
    println!("{ribon_order}");
}

fn main() {
    part1();
    part2();
}
