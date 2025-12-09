fn main() {
    let s = std::fs::read_to_string("07/input.txt").unwrap();
    let w = s.lines().next().unwrap().len();
    let mut dp = vec![0u64; w];

    for line in s.lines() {
        let b = line.as_bytes();
        if let Some(i) = b.iter().position(|&c| c == b'S') {
            dp[i] = 1;
            continue;
        }

        if dp.iter().sum::<u64>() == 0 {
            continue;
        }

        let mut next = vec![0; w];
        for (i, &n) in dp.iter().enumerate() {
            if n == 0 {
                continue;
            }
            match b[i] {
                b'^' => {
                    if i > 0 {
                        next[i - 1] += n;
                    }
                    if i + 1 < w {
                        next[i + 1] += n;
                    }
                }
                _ => next[i] += n,
            }
        }
        dp = next;
    }
    println!("{}", dp.iter().sum::<u64>());
}
