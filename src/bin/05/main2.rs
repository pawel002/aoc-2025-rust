fn main() {
    let mut ranges: Vec<(u32, u32)> = include_str!("05/input.txt")
        .lines()
        .filter_map(|line| line.split_once('-'))
        .map(|(s, e)| (s.parse().unwrap(), e.parse().unwrap()))
        .collect();

    ranges.sort_unstable();

    let merged = ranges
        .into_iter()
        .fold(Vec::new(), |mut acc: Vec<(u32, u32)>, (start, end)| {
            match acc.last_mut() {
                Some((_, last_end)) if start <= *last_end + 1 => *last_end = (*last_end).max(end),
                _ => acc.push((start, end)),
            };
            acc
        });

    let total: u32 = merged.iter().map(|(s, e)| e - s + 1).sum();

    println!("{}", total);
}
