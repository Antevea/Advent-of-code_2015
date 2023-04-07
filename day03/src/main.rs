use std::fs::read_to_string;

struct Coord {
    x: i32,
    y: i32,
    path_log: Vec<(i32, i32)>,
}

impl Coord {
    fn make_move(&mut self, dx: i32, dy: i32) {
        self.path_log.push((self.x, self.y));
        self.x += dx;
        self.y += dy;
    }
}

fn part1() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str = read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let mut crd = Coord {
        x: 0,
        y: 0,
        path_log: Vec::new(),
    };
    let dir_vec: Vec<char> = input_str.
        trim_end().
        chars().
        into_iter().
        collect();

    for dir in dir_vec {
        match dir {
            '^' => {
                crd.make_move(0, 1);
            },
            'v' => {
                crd.make_move(0, -1);
            },
            '<' => {
                crd.make_move(-1, 0);
            },
            '>' => {
                crd.make_move(1, 0);
            },
            _ => {
                eprintln!("Error: Could not determine move direction character {dir}");
            }
        };
    }

    crd.make_move(0, 0);

    crd.path_log.sort();
    crd.path_log.dedup();

    println!("{:?}", crd.path_log.len());
}

fn part2() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str = read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let mut santa_coord = Coord {
        x: 0,
        y: 0,
        path_log: Vec::new(),
    };
    let mut robot_coord = Coord {
        x: 0,
        y: 0,
        path_log: Vec::new(),
    };
    let dir_vec: Vec<char> = input_str.
        trim_end().
        chars().
        into_iter().
        collect();

    let mut dir_iter = dir_vec.into_iter();
    while let Some(dir) = dir_iter.next() {
        match dir {
            '^' => {
                santa_coord.make_move(0, 1);
            },
            'v' => {
                santa_coord.make_move(0, -1);
            },
            '<' => {
                santa_coord.make_move(-1, 0);
            },
            '>' => {
                santa_coord.make_move(1, 0);
            },
            _ => {
                eprintln!("Error: Could not determine move direction character {dir}");
                break;
            }
        }
        if let Some(dir_temp) = dir_iter.next() {
            match dir_temp {
                '^' => {
                    robot_coord.make_move(0, 1);
                },
                'v' => {
                    robot_coord.make_move(0, -1);
                },
                '<' => {
                    robot_coord.make_move(-1, 0);
                },
                '>' => {
                    robot_coord.make_move(1, 0);
                },
                _ => {
                    eprintln!("Error: Could not determine move direction character {dir}");
                    break;
                }
            }
        }
    }
    santa_coord.make_move(0, 0);
    robot_coord.make_move(0, 0);

    santa_coord.path_log.extend(robot_coord.path_log);
    santa_coord.path_log.sort();
    santa_coord.path_log.dedup();

    println!("{:?}", santa_coord.path_log.len());
}

fn main() {
    part1();
    part2();
}

