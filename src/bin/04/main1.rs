fn main() {
    let g: Vec<Vec<_>> = std::fs::read_to_string("04/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.bytes().map(|b| (b == b'@') as i32).collect())
        .collect();

    println!(
        "{}",
        (0..g.len())
            .flat_map(|r| (0..g[0].len()).map(move |c| (r, c)))
            .filter(|&(r, c)| g[r][c] == 1
                && (-1..=1)
                    .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
                    .filter_map(|(dx, dy)| g
                        .get((r as isize + dx) as usize)?
                        .get((c as isize + dy) as usize))
                    .sum::<i32>()
                    < 5)
            .count()
    );
}
