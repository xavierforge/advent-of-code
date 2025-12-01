#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    const SEQUENCE_LENGTH: isize = 3; // Length of the pattern to check (MAS)

    // Parse the input into a grid of characters
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    let mut count = 0;

    // Helper function to check if a position is within bounds
    let is_in_bounds =
        |row: isize, col: isize| -> bool { row >= 0 && row < height && col >= 0 && col < width };

    for row in 0..height {
        for col in 0..width {
            // Process cells containing 'X'
            if grid[row as usize][col as usize] != 'X' {
                continue;
            }

            // Check in all 8 directions
            for dr in [-1, 0, 1] {
                for dc in [-1, 0, 1] {
                    // Skip the center direction (no movement)
                    if dr == 0 && dc == 0 {
                        continue;
                    }

                    let mut valid = true;
                    let pattern = ['M', 'A', 'S'];

                    for i in 1..=SEQUENCE_LENGTH {
                        let next_row = row + i * dr;
                        let next_col = col + i * dc;

                        if !is_in_bounds(next_row, next_col)
                            || grid[next_row as usize][next_col as usize]
                                != pattern[(i - 1) as usize]
                        {
                            valid = false;
                            break;
                        }
                    }

                    if valid {
                        count += 1;
                    }
                }
            }
        }
    }

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("18", process(EXAMPLE)?);
        Ok(())
    }
}
