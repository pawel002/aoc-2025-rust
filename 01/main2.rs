fn main() {
    print!("{}", std::fs::read_to_string("01/input.txt").unwrap().lines().fold((50, 0), |(p, c), l| {
        let v: i32 = l[1..].parse().unwrap();
        let d = l.as_bytes()[0] == b'R';
        let raw_dest = if d { p + v } else { p - v };
        let laps = (raw_dest.div_euclid(100) - p.div_euclid(100)).abs();
        let p_n = raw_dest.rem_euclid(100);
        (p_n, c + laps + (p_n == 0) as i32)
    }).1);
}