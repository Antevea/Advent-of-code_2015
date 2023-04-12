use std::fs::read_to_string;

fn part1() {
    enum LightswithFlags {
        LightOn,
        LightOff,
        LightToggle,
    }

    fn get_coordinates_from_str(coord: &str) -> Vec<u16> {
        let coordinates = coord.
            split(',').
            map(|s| s.parse::<u16>().unwrap()).
            collect();
        return coordinates;
    }

    fn parse_instruction(instr: &str) -> (LightswithFlags, Vec<u16>, Vec<u16>) {
        let instruct_vec: Vec<&str> = instr.split(' ').collect::<Vec<&str>>();

        let mut lightflag = LightswithFlags::LightOff;
        let mut start_coordinates_vec: Vec<u16> = Vec::with_capacity(2);
        let mut end_coordinates_vec: Vec<u16> = Vec::with_capacity(2);

        let mut instruct_iter = instruct_vec.into_iter();

        if let Some(lightswitch_instr) = instruct_iter.next() {
            match lightswitch_instr {
                "turn" => {
                    match instruct_iter.next() {
                        Some("on") => {
                            lightflag = LightswithFlags::LightOn;
                        },
                        Some("off") => {
                            lightflag = LightswithFlags::LightOff;
                        },
                        _ => { eprintln!("Error: Wrong light switch instruction {}", lightswitch_instr) }
                    }

                    if let Some(start_coord_str) = instruct_iter.next() {
                        start_coordinates_vec = get_coordinates_from_str(start_coord_str);
                    } else {
                        eprintln!("Error: Unable to recognize start coordinates: {}", instr);
                    }

                    if let Some(end_coord_str) = instruct_iter.last() {
                        end_coordinates_vec = get_coordinates_from_str(end_coord_str);
                    } else {
                        eprintln!("Error: Unable to recognize end coordinates: {}", instr);
                    }
                },
                "toggle" => {
                    lightflag = LightswithFlags::LightToggle;

                    if let Some(start_coord_str) = instruct_iter.next() {
                        start_coordinates_vec = get_coordinates_from_str(start_coord_str);
                    } else {
                        eprintln!("Error: Unable to recognize start coordinates: {}", instr);
                    }

                    if let Some(end_coord_str) = instruct_iter.last() {
                        end_coordinates_vec = get_coordinates_from_str(end_coord_str);
                    } else {
                        eprintln!("Error: Unable to recognize end coordinates: {}", instr);
                    }
                },
                _ => { eprintln!("Error: Wrong light switch instruction {}", lightswitch_instr) },
            }
        } else {
            eprintln!("Error: Unable to recognize instruction: {:?}", instr);
        }

        (lightflag, start_coordinates_vec, end_coordinates_vec)
    }

    fn lightswitch_process(
        light_grid: &mut [[bool; 1000]; 1000],
        lightflag: LightswithFlags,
        start_coord: Vec<u16>,
        end_coord: Vec<u16>)
    {
        let start_x = start_coord.first().unwrap().to_owned() as usize;
        let start_y = start_coord.last().unwrap().to_owned() as usize;
        let end_x = end_coord.first().unwrap().to_owned() as usize;
        let end_y = end_coord.last().unwrap().to_owned() as usize;

        for row in &mut light_grid[start_x..=end_x] {
            for light in &mut row[start_y..=end_y] {
                *light = match lightflag {
                    LightswithFlags::LightOn => true,
                    LightswithFlags::LightOff => false,
                    LightswithFlags::LightToggle => if *light == true {false} else {true},
                }
            }
        }
    }

    fn count_lit_lights(light_grid: [[bool; 1000]; 1000]) -> u64 {
        let mut lit_lights: u64 = 0;

        for row in &light_grid[..] {
            for light in &row[..] {
                if *light == true {
                    lit_lights += 1;
                }
            }
        }

        lit_lights
    }

    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str = read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let instructions_vec: Vec<&str> = input_str.trim_end().split('\n').collect::<Vec<&str>>();
    let mut light_grid = [[false; 1000]; 1000];

    for instruction in instructions_vec {
        let (lightflag, start_coord, end_coord) = parse_instruction(instruction);
        lightswitch_process(&mut light_grid, lightflag, start_coord, end_coord);
    }

    let lit_lights = count_lit_lights(light_grid);
    println!("{}", lit_lights);
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
