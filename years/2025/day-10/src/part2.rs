use tracing::instrument;

#[derive(Debug)]
struct MachinePart2 {
    target_state: Vec<u32>,
    buttons: Vec<Vec<usize>>,
}

fn parse_joltage(input: &str) -> Vec<u32> {
    let content = input
        .trim()
        .strip_prefix('{')
        .and_then(|s| s.strip_suffix('}'))
        .expect("i.kkiInvalid joltage format! Missing curly braces.");
    content
        .split(',')
        .map(|num_str| {
            num_str
                .trim()
                .parse::<u32>()
                .expect("Should be a valid number")
        })
        .collect()
}

fn parse_buttons_p2(input: &str) -> Vec<Vec<usize>> {
    input
        .split_whitespace()
        .map(|token| {
            let inner = token
                .strip_prefix('(')
                .and_then(|s| s.strip_suffix(')'))
                .expect("Invalid button format!");
            inner
                .split(',')
                .map(|num_str| {
                    num_str
                        .trim()
                        .parse::<usize>()
                        .expect("Should be a valid number")
                })
                .collect()
        })
        .collect()
}

fn parse_line_p2(line: &str) -> Option<MachinePart2> {
    let target_end = line.find(']')?;
    let joltage_start = line.find('{')?;

    if target_end >= joltage_start {
        return None;
    }

    let buttons_chunk = line[(target_end + 1)..joltage_start].trim();
    let joltage_str = line[joltage_start..].trim();

    Some(MachinePart2 {
        target_state: parse_joltage(joltage_str),
        buttons: parse_buttons_p2(buttons_chunk),
    })
}

// Struct to hold static context for the recursive solver
struct SolverContext<'a> {
    matrix: &'a [Vec<f64>],
    col_to_pivot: &'a [Option<usize>],
    bounds: &'a [usize],
    free_vars: &'a [usize],
}

fn solve_machine_p2(machine: &MachinePart2) -> Option<usize> {
    let num_eq = machine.target_state.len(); // Number of equations (dimensions)
    let num_vars = machine.buttons.len(); // Number of variables (buttons)

    // 1. Build Augmented Matrix [A | b]
    // Use f64 for Gaussian elimination
    let mut matrix = vec![vec![0.0; num_vars + 1]; num_eq];

    // Fill coefficient matrix A
    for (col, btn_indices) in machine.buttons.iter().enumerate() {
        for &row in btn_indices {
            if row < num_eq {
                matrix[row][col] = 1.0;
            }
        }
    }

    // Fill constant vector b (Target state)
    for (row, &val) in machine.target_state.iter().enumerate() {
        matrix[row][num_vars] = val as f64;
    }

    // 2. Gaussian Elimination (Convert to RREF)
    let mut pivot_row = 0;
    let mut col_to_pivot = vec![None; num_vars]; // Maps variable column to its pivot row
    let mut free_vars = Vec::new(); // Variables without pivots (free variables)

    for col in 0..num_vars {
        if pivot_row >= num_eq {
            free_vars.push(col);
            continue;
        }

        // Find Pivot (Partial Pivoting)
        let mut max_row = pivot_row;
        for r in (pivot_row + 1)..num_eq {
            if matrix[r][col].abs() > matrix[max_row][col].abs() {
                max_row = r;
            }
        }

        // If the column is effectively zero, it's a free variable
        if matrix[max_row][col].abs() < 1e-9 {
            free_vars.push(col);
            continue;
        }

        // Swap rows
        matrix.swap(pivot_row, max_row);

        // Normalize Pivot Row
        let pivot_val = matrix[pivot_row][col];
        for val in &mut matrix[pivot_row][col..=num_vars] {
            *val /= pivot_val;
        }

        // --- FIXED: Use a clone of the pivot row to satisfy borrow checker and Clippy ---
        // This allows us to use zip() for clean iteration without range loops.
        // Cloning a small vector of f64s is very cheap.
        let pivot_row_vals = matrix[pivot_row].clone();

        // Eliminate other rows
        for (i, row_i) in matrix.iter_mut().enumerate() {
            if i != pivot_row {
                let factor = row_i[col];
                if factor.abs() > 1e-9 {
                    // Optimized subtraction using zip
                    // Iterating over the relevant slice of both rows
                    let target_slice = &mut row_i[col..=num_vars];
                    let pivot_slice = &pivot_row_vals[col..=num_vars];

                    for (target, &pivot) in target_slice.iter_mut().zip(pivot_slice) {
                        *target -= factor * pivot;
                    }
                }
            }
        }

        col_to_pivot[col] = Some(pivot_row);
        pivot_row += 1;
    }

    // 3. Check for inconsistency (0 = non-zero)
    for row in matrix.iter().skip(pivot_row) {
        if row[num_vars].abs() > 1e-4 {
            return None; // System has no solution
        }
    }

    // 4. Calculate Upper Bounds for each button
    let mut bounds = vec![usize::MAX; num_vars];
    for (col, btn_indices) in machine.buttons.iter().enumerate() {
        for &row in btn_indices {
            if row < num_eq {
                let limit = machine.target_state[row] as usize;
                if limit < bounds[col] {
                    bounds[col] = limit;
                }
            }
        }
    }

    // 5. Recursively find the best integer solution
    let mut min_total_presses = None;
    let mut free_vals = vec![0; free_vars.len()];

    let ctx = SolverContext {
        matrix: &matrix,
        col_to_pivot: &col_to_pivot,
        bounds: &bounds,
        free_vars: &free_vars,
    };

    solve_recursive(0, &mut free_vals, &ctx, &mut min_total_presses);

    min_total_presses
}

fn solve_recursive(
    idx: usize,
    free_vals: &mut Vec<usize>,
    ctx: &SolverContext,
    min_total: &mut Option<usize>,
) {
    // Base case: All free variables are assigned
    if idx == ctx.free_vars.len() {
        let mut current_total = 0;

        // Sum up free variables
        for &v in free_vals.iter() {
            current_total += v;
        }

        // Calculate Pivot Variables (Dependent variables)
        // x_pivot = Constant - sum(coeff * x_free)
        // FIXED: Removed unused 'col' variable by only iterating over values
        for &maybe_row in ctx.col_to_pivot.iter() {
            if let Some(row) = maybe_row {
                let mut val = ctx.matrix[row][ctx.matrix[0].len() - 1]; // Constant term

                for (i, &fv_idx) in ctx.free_vars.iter().enumerate() {
                    val -= ctx.matrix[row][fv_idx] * (free_vals[i] as f64);
                }

                // Check if the result is a valid non-negative integer
                if val < -1e-4 {
                    return;
                } // Cannot be negative
                let rounded = val.round();
                if (val - rounded).abs() > 1e-4 {
                    return;
                } // Must be an integer

                current_total += rounded as usize;
            }
        }

        // Update minimum total
        if min_total.is_none_or(|min| current_total < min) {
            *min_total = Some(current_total);
        }
        return;
    }

    // Recursive step: Try values for the current free variable
    let fv_col = ctx.free_vars[idx];
    let limit = ctx.bounds[fv_col];

    // Pruning: Check if current sum already exceeds known minimum
    let current_free_sum: usize = free_vals.iter().take(idx).sum();

    if min_total.is_some_and(|min| current_free_sum >= min) {
        return;
    }

    for val in 0..=limit {
        free_vals[idx] = val;
        solve_recursive(idx + 1, free_vals, ctx, min_total);
    }
}

#[instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let mut total_presses = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if let Some(machine) = parse_line_p2(line)
            && let Some(p) = solve_machine_p2(&machine)
        {
            total_presses += p;
        }
    }
    Ok(total_presses.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_parse_joltage() {
        assert_eq!(parse_joltage("{3,5,4,7}"), vec![3, 5, 4, 7]);
        assert_eq!(parse_joltage("{10,11}"), vec![10, 11]);
    }

    #[test_log::test]
    fn test_parse_buttons_p2() {
        // (1,3) (2)
        let res = parse_buttons_p2("(1,3) (2)");
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], vec![1, 3]);
        assert_eq!(res[1], vec![2]);
    }

    #[test_log::test]
    fn test_solve_example_1() {
        // Example 1:
        // Buttons: (3), (1,3), (2), (2,3), (0,2), (0,1)
        // Target: {3,5,4,7}
        // Min presses: 10
        let machine = MachinePart2 {
            target_state: vec![3, 5, 4, 7],
            buttons: vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1],
            ],
        };
        assert_eq!(solve_machine_p2(&machine), Some(10));
    }

    #[test_log::test]
    fn test_solve_example_2() {
        // Example 2:
        // Buttons: (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4)
        // Target: {7,5,12,7,2}
        // Min presses: 12
        let machine = MachinePart2 {
            target_state: vec![7, 5, 12, 7, 2],
            buttons: vec![
                vec![0, 2, 3, 4],
                vec![2, 3],
                vec![0, 4],
                vec![0, 1, 2],
                vec![1, 2, 3, 4],
            ],
        };
        assert_eq!(solve_machine_p2(&machine), Some(12));
    }

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!("33", process(input)?);
        Ok(())
    }
}
