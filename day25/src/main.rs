fn num_at_rc(row: u64, col: u64) -> u64 {
    1 + (col + row - 1) * (col + row) / 2 - row
}

fn power_mod(start: u64, mut base: u64, mut exp: u64, modul: u64) -> u64 {
    let mut result = start % modul;
    base %= modul;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modul;
        }

        exp >>= 1;
        base = (base * base) % modul;
    }

    result
}

fn solution() {
    let row = 2978;
    let col = 3083;

    let ans = power_mod(
        20151125,
        252533,
        num_at_rc(row, col) - 1,
        33554393,
    );

    println!("Answer: {}", ans);
}

fn main() {
    solution();
}
