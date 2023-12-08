fn solution() {
    let mut max_score = 0i64;
    let mut max_score_calories = 0i64;

    for i1 in 0..100 {
        for i2 in 0..(100 - i1) {
            for i3 in 0..(100 - i1 - i2) {
                let i4 = 100 - i1 - i2 - i3;

                if i4 < 0 {
                    continue;
                }

                let cap = ((4 * i1) - i3).max(0);
                let dur = (5 * i2 - 2 * i1).max(0);
                let fla = (5 * i3 - i2 - 2 * i4).max(0);
                let tex = (2 * i4).max(0);

                let score = cap * dur * fla * tex;
                max_score = max_score.max(score);

                let calories = 5 * i1 + 8 * i2 + 6 * i3 + i4;
                if calories == 500 {
                    max_score_calories = max_score_calories.max(score);
                }
            }
        }
    }

    println!("Part 1: {}", max_score);
    println!("Part 2: {}", max_score_calories);
}

fn main() {
    solution();
}
