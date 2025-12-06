use tracing::{info, instrument};

#[derive(Debug)]
pub struct Problem {
    pub numbers: Vec<u64>,
    pub operator: char,
}

impl Problem {
    pub fn solve(&self) -> u64 {
        if self.numbers.is_empty() {
            return 0;
        }

        // Use reduce to handle the first number as the initial value naturally
        self.numbers
            .iter()
            .skip(1)
            .fold(self.numbers[0], |acc, &num| match self.operator {
                '+' => acc + num,
                '*' => acc * num,
                _ => acc,
            })
    }
}

pub struct WorksheetParser;

impl WorksheetParser {
    pub fn to_grid(input: &str) -> Vec<Vec<char>> {
        let lines: Vec<&str> = input.lines().collect();
        let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

        lines
            .iter()
            .map(|line| {
                let mut chars: Vec<char> = line.chars().collect();
                chars.resize(max_width, ' ');
                chars
            })
            .collect()
    }

    pub fn is_column_empty(grid: &[Vec<char>], col_idx: usize) -> bool {
        grid.iter()
            .all(|row| row.get(col_idx).unwrap_or(&' ') == &' ')
    }

    fn parse_slice(grid: &[Vec<char>], start_col: usize, end_col: usize) -> Option<Problem> {
        let height = grid.len();

        if height < 2 {
            return None; // Need at least one number row and one operator row
        }

        // Find Operator
        let last_row = &grid[height - 1];
        let operator = last_row[start_col..end_col]
            .iter()
            .find(|&&c| c != ' ')
            .copied()?;

        // Parse Numbers
        let mut numbers = Vec::new();
        for row in grid.iter().take(height - 1) {
            let row_slice = &row[start_col..end_col];
            let row_str: String = row_slice.iter().collect();
            if let Ok(num) = row_str.trim().parse::<u64>() {
                numbers.push(num);
            }
        }

        if numbers.is_empty() {
            return None;
        }

        Some(Problem { numbers, operator })
    }

    fn parse_all(input: &str) -> Vec<Problem> {
        let grid = Self::to_grid(input);
        if grid.is_empty() {
            return vec![];
        }

        let width = grid[0].len();
        let mut problems = Vec::new();
        let mut start_col = 0;

        for col in 0..width {
            // If we hit an empty column, it's a separator.
            if Self::is_column_empty(&grid, col) {
                // If we have accumulated width, parse the slice before this separator
                if col > start_col
                    && let Some(p) = Self::parse_slice(&grid, start_col, col)
                {
                    problems.push(p);
                }

                // The next problem starts after this empty column
                start_col = col + 1;
            }
        }

        // Handle the last block if it wasn't followed by a space
        if start_col < width
            && let Some(p) = Self::parse_slice(&grid, start_col, width)
        {
            problems.push(p);
        }

        problems
    }
}

#[instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let problems = WorksheetParser::parse_all(input);

    let grand_total: u64 = problems.iter().map(|p| p.solve()).sum();

    info!(
        problem_count = problems.len(),
        grand_total, "Worksheet processed"
    );

    Ok(grand_total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test_log::test]
    fn test_to_grid() {
        let input = "12\n3";
        let grid = WorksheetParser::to_grid(input);

        assert_eq!(grid.len(), 2);
        assert_eq!(grid[0], vec!['1', '2']);
        assert_eq!(grid[1], vec!['3', ' ']); // should be padded
    }

    #[test_log::test]
    fn test_is_column_empty() {
        let grid = vec![vec!['1', ' ', '3'], vec![' ', ' ', '4']];

        // Col 0: '1', ' ' -> Not empty
        assert!(!WorksheetParser::is_column_empty(&grid, 0));
        // Col 1: ' ', ' ' -> Empty
        assert!(WorksheetParser::is_column_empty(&grid, 1));
    }

    #[test_log::test]
    fn test_parse_slice() {
        let grid = vec![
            vec!['1', '2', ' '], // "12"
            vec![' ', '5', ' '], // " 5"
            vec!['+', ' ', ' '], // Operator +
        ];

        // Slice the whole width (0..3)
        let problem = WorksheetParser::parse_slice(&grid, 0, 3).unwrap();

        assert_eq!(problem.operator, '+');
        assert_eq!(problem.numbers, vec![12, 5]);
    }

    #[test_log::test(rstest)]
    #[case(vec![10, 20, 30], '+', 60)]
    #[case(vec![2, 3, 4], '*', 24)]
    #[case(vec![5], '+', 5)]
    fn test_problem_solve(
        #[case] numbers: Vec<u64>,
        #[case] operator: char,
        #[case] expected: u64,
    ) {
        let p = Problem { numbers, operator };
        assert_eq!(p.solve(), expected)
    }

    #[test_log::test]
    fn test_parse_all_intergration() {
        let input = "123 328\n 45  64\n  6  98\n*   +";
        let problems = WorksheetParser::parse_all(input);

        assert_eq!(problems.len(), 2);

        // Check Problem 1
        assert_eq!(problems[0].operator, '*');
        assert_eq!(problems[0].numbers, vec![123, 45, 6]);

        // Check Problem 2
        assert_eq!(problems[1].operator, '+');
        assert_eq!(problems[1].numbers, vec![328, 64, 98]);
    }

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!("4277556", process(input)?);
        Ok(())
    }
}
