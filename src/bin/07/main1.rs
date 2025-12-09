use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("07/input.txt")?;

    let mut grid: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    if grid.is_empty() {
        println!("0");
        return Ok(());
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 'S' {
                if r + 1 < rows {
                    grid[r + 1][c] = '|';
                }
            }

            if r > 0 && grid[r - 1][c] == '|' {
                if grid[r][c] == '^' {
                    count += 1;

                    if c > 0 {
                        grid[r][c - 1] = '|';
                    }

                    if c + 1 < cols {
                        grid[r][c + 1] = '|';
                    }
                } else {
                    grid[r][c] = '|';
                }
            }
        }
    }

    println!("{}", count);

    Ok(())
}
