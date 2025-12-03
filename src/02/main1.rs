fn main() {
    println!(
        "{}",
        std::fs::read_to_string("input.txt")
            .unwrap()
            .trim()
            .split(',')
            .filter_map(|s| s.split_once('-'))
            .flat_map(|(a, b)| a.parse::<u64>().unwrap()..=b.parse::<u64>().unwrap())
            .filter(|x| {
                let s = x.to_string();
                s.len() % 2 == 0 && s[..s.len() / 2] == s[s.len() / 2..]
            })
            .sum::<u64>()
    );
}