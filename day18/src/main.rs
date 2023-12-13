use std::fs::read_to_string;

const ROW_NUM: usize = 100;
const COL_NUM: usize = 100;

fn grid_step(grid: &[[u16; ROW_NUM]; COL_NUM], is_part_two: bool) -> [[u16; ROW_NUM]; COL_NUM] {
    let mut next_step_grid = grid.clone().to_owned();
    const ROWS: usize = ROW_NUM-1;
    const COLS: usize = COL_NUM-1;

    for i in 0..ROW_NUM {
        for j in 0..COL_NUM {
            let neigbours_num = match (i, j) {
                (0, 0) => {
                    if is_part_two == true {
                        continue;
                    } else {
                        grid[i+1][j+1] + grid[i+1][j] + grid[i][j+1]
                    }
                },
                (0, COLS) => {
                    if is_part_two == true {
                        continue;
                    } else {
                        grid[i+1][j-1] + grid[i][j-1] + grid[i+1][j]
                    }
                },
                (ROWS, 0) => {
                    if is_part_two == true {
                        continue;
                    } else {
                        grid[i-1][j+1] + grid[i-1][j] + grid[i][j+1]
                    }
                },
                (ROWS, COLS) => {
                    if is_part_two == true {
                        continue;
                    } else {
                        grid[i-1][j-1] + grid[i][j-1] + grid[i-1][j]
                    }
                },
                (0, _)      => grid[i][j-1] + grid[i][j+1] + grid[i+1][j+1] + grid[i+1][j] + grid[i+1][j-1],
                (ROWS, _)   => grid[i][j-1] + grid[i][j+1] + grid[i-1][j-1] + grid[i-1][j] + grid[i-1][j+1],
                (_, 0)      => grid[i-1][j] + grid[i-1][j+1] + grid[i][j+1] + grid[i+1][j+1] + grid[i+1][j],
                (_, COLS)   => grid[i-1][j] + grid[i-1][j-1] + grid[i][j-1] + grid[i+1][j-1] + grid[i+1][j],
                _ =>  grid[i-1][j-1] + grid[i-1][j] + grid[i-1][j+1]
                    + grid[i][j-1]                  + grid[i][j+1]
                    + grid[i+1][j-1] + grid[i+1][j] + grid[i+1][j+1],
            };

            if grid[i][j] == 1 {
                if neigbours_num != 2 && neigbours_num != 3 {
                    next_step_grid[i][j] = 0;
                }
            } else {
                if neigbours_num == 3 {
                    next_step_grid[i][j] = 1;
                }
            }
        }
    }

    next_step_grid
}

fn part1() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str =
        read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let mut grid = [[0 as u16; ROW_NUM]; COL_NUM];
    let mut active_lights = 0u32;

    let mut i = 0usize;
    let mut j = 0usize;
    for line in input_str.lines() {
        for c in line.chars() {
            if c == '#' {
                grid[j][i] = 1;
            } else {
                grid[j][i] = 0;
            }
            i += 1;
        }
        i = 0;
        j += 1;
    }

    for _ in 0..100 {
        grid = grid_step(&grid, false);
    }

    for row in grid.iter() {
        for col in row.iter() {
            active_lights += col.to_owned() as u32;
        }
    }

    println!("Part 1: {}", active_lights);
}

fn part2() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str =
        read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let mut grid = [[0 as u16; ROW_NUM]; COL_NUM];
    let mut active_lights = 0u32;

    let mut i = 0usize;
    let mut j = 0usize;
    for line in input_str.lines() {
        for c in line.chars() {
            if c == '#' {
                grid[j][i] = 1;
            } else {
                grid[j][i] = 0;
            }
            i += 1;
        }
        i = 0;
        j += 1;
    }

    for _ in 0..100 {
        grid = grid_step(&grid, true);
    }

    for row in grid.iter() {
        for col in row.iter() {
            active_lights += col.to_owned() as u32;
        }
    }

    println!("Part 2: {}", active_lights);
}

fn main() {
    part1();
    part2();
}
