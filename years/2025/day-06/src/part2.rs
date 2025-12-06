use tracing::{info, instrument};

use crate::part1::{Problem, WorksheetParser};

trait VerticalScanner {
    fn parse_all_vertical(input: &str) -> Vec<Problem>;
}

impl VerticalScanner for WorksheetParser {
    #[instrument(skip(input))]
    fn parse_all_vertical(input: &str) -> Vec<Problem> {
        let grid = Self::to_grid(input);
        if grid.is_empty() {
            return vec![];
        }

        let width = grid[0].len();
        let mut problems = Vec::new();
        let mut start_col = 0;

        for col in 0..width {
            if Self::is_column_empty(&grid, col) {
                if col > start_col
                    && let Some(p) = parse_slice_vertical(&grid, start_col, col)
                {
                    problems.push(p);
                }
                start_col = col + 1;
            }
        }

        if start_col < width
            && let Some(p) = parse_slice_vertical(&grid, start_col, width)
        {
            problems.push(p);
        }

        problems
    }
}

fn parse_slice_vertical(grid: &[Vec<char>], start_col: usize, end_col: usize) -> Option<Problem> {
    let height = grid.len();
    if height < 2 {
        return None;
    }

    let operator = grid[height - 1][start_col..end_col]
        .iter()
        .find(|&&c| c != ' ')
        .copied()?;

    let mut numbers = Vec::new();

    for c in (start_col..end_col).rev() {
        let mut digit_str = String::new();

        for row in grid.iter().take(height - 1) {
            let char_at = row[c];
            if char_at.is_ascii_digit() {
                digit_str.push(char_at);
            }
        }

        if !digit_str.is_empty()
            && let Ok(num) = digit_str.parse::<u64>()
        {
            numbers.push(num);
        }
    }

    if numbers.is_empty() {
        None
    } else {
        Some(Problem { numbers, operator })
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let problems = WorksheetParser::parse_all_vertical(input);

    let total: u64 = problems.iter().map(|p| p.solve()).sum();

    info!(count = problems.len(), total, "Part 2 processing complete");
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_parse_slice_vertical() {
        let grid = vec![
            vec!['1', '2', '3'],
            vec![' ', '4', '5'],
            vec![' ', ' ', '6'],
            vec!['*', ' ', ' '],
        ];

        let problem = parse_slice_vertical(&grid, 0, 3).unwrap();

        assert_eq!(problem.operator, '*');
        // Remember: Right-to-Left
        assert_eq!(problem.numbers, vec![356, 24, 1]);
        assert_eq!(problem.solve(), 8544);
    }

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!("3263827", process(input)?);
        Ok(())
    }
}
