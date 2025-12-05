fn main() {
    let mut g: Vec<Vec<i32>> = std::fs::read_to_string("04/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.bytes().map(|b| (b == b'@') as i32).collect())
        .collect();

    let mut total_removed = 0;
    loop {
        let mut d = 0;
        let rows = g.len();
        let cols = g[0].len();

        for r in 0..rows {
            for c in 0..cols {
                if g[r][c] == 1 {
                    let sum: i32 = (-1..=1)
                        .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
                        .filter_map(|(dx, dy)| {
                            g.get((r as isize + dx) as usize)?
                                .get((c as isize + dy) as usize)
                        })
                        .sum();

                    if sum < 5 {
                        d += 1;
                        g[r][c] = 0;
                    }
                }
            }
        }

        if d == 0 {
            break;
        }
        total_removed += d;
    }
    println!("{}", total_removed);
}
