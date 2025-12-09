fn main() {
    print!("{}", {
        let (intervals, numbers) = std::fs::read_to_string("05/input.txt")
            .unwrap()
            .lines()
            .fold((vec![], vec![]), |(mut ints, mut nums), l| {
                match l.split_once('-') {
                    Some((a, b)) => ints.push((
                        a.trim().parse::<i32>().unwrap(),
                        b.trim().parse::<i32>().unwrap(),
                    )),
                    None => nums.push(l.trim().parse::<i32>().unwrap()),
                }
                (ints, nums)
            });
        numbers
            .iter()
            .filter(|&&n| intervals.iter().any(|&(a, b)| n >= a && n <= b))
            .count()
    });
}
