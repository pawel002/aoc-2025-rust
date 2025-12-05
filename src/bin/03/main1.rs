fn main() {
    print!(
        "{}",
        std::fs::read_to_string("03/input.txt")
            .unwrap()
            .lines()
            .map(|l| {
                let d: Vec<u32> = l.chars().filter_map(|c| c.to_digit(10)).collect();

                d.iter()
                    .enumerate()
                    .flat_map(|(i, &a)| d[i + 1..].iter().map(move |&b| a * 10 + b))
                    .max()
                    .unwrap_or(0)
            })
            .sum::<u32>()
    );
}
