fn main() {
    println!(
        "{}",
        std::fs::read_to_string("02/input.txt")
            .unwrap()
            .trim()
            .split(',')
            .filter_map(|s| s.split_once('-'))
            .flat_map(|(a, b)| a.parse::<u64>().unwrap()..=b.parse::<u64>().unwrap())
            .filter(|x| {
                let s = x.to_string();
                (1..=s.len() / 2).any(|i| s.len() % i == 0 && s == s[..i].repeat(s.len() / i))
            })
            .sum::<u64>()
    );
}