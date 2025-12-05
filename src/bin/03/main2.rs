fn main() {
    print!(
        "{}",
        std::fs::read_to_string("03/input.txt")
            .unwrap()
            .lines()
            .map(|l| {
                let mut dp = [0u64; 13];
                for b in l.bytes() {
                    let d = (b - b'0') as u64;
                    for k in (1..=12).rev() {
                        dp[k] = dp[k].max(dp[k - 1] * 10 + d);
                    }
                }
                dp[12]
            })
            .sum::<u64>()
    );
}
