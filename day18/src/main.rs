use std::fs::read_to_string;

const ROW_NUM: usize = 100;
const COL_NUM: usize = 100;

fn grid_step(grid: &[[u16; ROW_NUM]; COL_NUM]) -> [[u16; ROW_NUM]; COL_NUM] {
    let mut next_step_grid = grid.clone().to_owned();

    for i in 0..ROW_NUM {
        for j in 0..COL_NUM {
            let neigbours_num: u16;
            if i == 0
            {
                if j == 0 {
                    neigbours_num = grid[i+1][j+1] + grid[i+1][j] + grid[i][j+1];
                } else if j == (COL_NUM-1) {
                    neigbours_num = grid[i+1][j-1] + grid[i][j-1] + grid[i+1][j];
                } else {
                    neigbours_num = grid[i][j-1] + grid[i][j+1] + grid[i+1][j+1] + grid[i+1][j] + grid[i+1][j-1];
                }
            } else if i == (ROW_NUM-1)
            {
                if j == 0 {
                    neigbours_num = grid[i-1][j+1] + grid[i-1][j] + grid[i][j+1];
                } else if j == (COL_NUM-1) {
                    neigbours_num = grid[i-1][j-1] + grid[i][j-1] + grid[i-1][j];
                } else {
                    neigbours_num = grid[i][j-1] + grid[i][j+1] + grid[i-1][j-1] + grid[i-1][j] + grid[i-1][j+1];
                }
            } else if j == 0 
            {
                neigbours_num = grid[i-1][j] + grid[i-1][j+1] + grid[i][j+1] + grid[i+1][j+1] + grid[i+1][j];
            } else if j == (COL_NUM-1)
            {
                neigbours_num = grid[i-1][j] + grid[i-1][j-1] + grid[i][j-1] + grid[i+1][j-1] + grid[i+1][j];
            } else {
                neigbours_num = grid[i-1][j-1] + grid[i-1][j] + grid[i-1][j+1]
                              + grid[i][j-1]                  + grid[i][j+1]
                              + grid[i+1][j-1] + grid[i+1][j] + grid[i+1][j+1];
            }

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
        grid = grid_step(&grid);
    }

    for row in grid.iter() {
        for col in row.iter() {
            active_lights += col.to_owned() as u32;
        }
    }

    println!("Part 1: {}", active_lights);
}

/* fn part2() {
    let filepath = "sample";
    // let filepath = "puzzle";
} */

fn main() {
    part1();
    // part2();
}
