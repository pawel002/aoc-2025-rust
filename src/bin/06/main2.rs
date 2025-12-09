use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("06/input.txt")?;
    let raw_lines: Vec<&str> = content.lines().collect();

    if raw_lines.is_empty() {
        return Ok(());
    }

    let height = raw_lines.len();
    let width = raw_lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut grid = vec![vec![' '; width]; height];
    for (row_idx, line) in raw_lines.iter().enumerate() {
        for (col_idx, ch) in line.chars().enumerate() {
            grid[row_idx][col_idx] = ch;
        }
    }

    let mut grand_total: i64 = 0;

    let mut current_numbers: Vec<i64> = Vec::new();
    let mut current_op: Option<char> = None;

    for col in (0..width).rev() {
        let is_separator = (0..height).all(|row| grid[row][col] == ' ');

        if is_separator {
            if !current_numbers.is_empty() {
                grand_total += solve(&current_numbers, current_op);
                current_numbers.clear();
                current_op = None;
            }
        } else {
            let bottom_char = grid[height - 1][col];
            if bottom_char == '+' || bottom_char == '*' {
                current_op = Some(bottom_char);
            }

            let mut digits_str = String::new();
            for row in 0..height - 1 {
                let ch = grid[row][col];
                if ch.is_ascii_digit() {
                    digits_str.push(ch);
                }
            }

            if !digits_str.is_empty() {
                if let Ok(num) = digits_str.parse::<i64>() {
                    current_numbers.push(num);
                }
            }
        }
    }

    if !current_numbers.is_empty() {
        grand_total += solve(&current_numbers, current_op);
    }

    println!("Grand Total: {}", grand_total);

    Ok(())
}

fn solve(numbers: &[i64], op: Option<char>) -> i64 {
    let operator = op.unwrap_or('+');

    if numbers.is_empty() {
        return 0;
    }

    let mut iter = numbers.iter();
    let first = *iter.next().unwrap();

    match operator {
        '+' => numbers.iter().sum(),
        '*' => numbers.iter().product(),
        _ => panic!("Unknown operator: {}", operator),
    }
}
