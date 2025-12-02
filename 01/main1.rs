fn main() {
    print!(
        "{}",
        std::fs::read_to_string("01/input.txt")
        .unwrap()
        .lines()
        .fold((50, 0), |(p, c), l| {
            let v: i32 = l[1..].parse().unwrap();
            let p = (p + if l.as_bytes()[0] == b'R' { v } else { -v }).rem_euclid(100);
            (p, c + (p == 0) as i32)
        }).1
    );
}