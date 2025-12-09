use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("06/input.txt")?;
    let mut lines = content.lines();

    let mut numbers: Vec<Vec<i64>> = Vec::new();
    for _ in 0..4 {
        if let Some(line) = lines.next() {
            let row: Vec<i64> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            numbers.push(row);
        }
    }

    let operations: Vec<&str> = lines
        .next()
        .ok_or("Missing operations line")?
        .split_whitespace()
        .collect();

    let mut c: i64 = 0;

    for (i, op) in operations.iter().enumerate() {
        let col_values: Vec<i64> = numbers.iter().map(|row| row[i]).collect();
        let result = apply_operation(op, &col_values);
        c += result;
    }

    println!("{}", c);
    Ok(())
}

fn apply_operation(op: &str, values: &[i64]) -> i64 {
    if values.is_empty() {
        return 0;
    }

    let mut iter = values.iter();
    let first = *iter.next().unwrap();

    match op {
        "+" => values.iter().sum(),
        "*" => values.iter().product(),
        _ => panic!("Unknown operator: {}", op),
    }
}
