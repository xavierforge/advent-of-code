use tracing::{debug, info, instrument};

struct Grid {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Grid {
    #[instrument(skip(input))]
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let raw_h = lines.len();
        let raw_w = lines.first().map(|l| l.len()).unwrap_or(0);

        if raw_h == 0 || raw_w == 0 {
            debug!("Input grid is empty");
            return Grid {
                width: 0,
                height: 0,
                data: vec![],
            };
        }

        let padded_h = raw_h + 2;
        let padded_w = raw_w + 2;
        let mut data = vec![b'.'; padded_h * padded_w];

        // Fill in the inner part of the grid with actual value
        for (r, line) in lines.iter().enumerate() {
            for (c, val) in line.bytes().enumerate() {
                // Map (r, c) to (r + 1, c + 1) in the flat array
                let idx = (r + 1) * padded_w + (c + 1);
                data[idx] = val;
            }
        }

        Grid {
            width: padded_w,
            height: padded_h,
            data,
        }
    }

    fn count_accessible_rolls(&self) -> usize {
        if self.data.is_empty() {
            return 0;
        }

        let mut count = 0;

        let offsets = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for r in 1..self.height - 1 {
            for c in 1..self.width - 1 {
                let current_idx = r * self.width + c;

                if self.data[current_idx] != b'@' {
                    continue;
                }

                let mut neighbor_count = 0;

                for (dr, dc) in offsets {
                    let nr = (r as isize + dr) as usize;
                    let nc = (c as isize + dc) as usize;
                    let n_idx = nr * self.width + nc;

                    if self.data[n_idx] == b'@' {
                        neighbor_count += 1;
                    }
                }

                if neighbor_count < 4 {
                    count += 1
                }
            }
        }

        info!(accessible_count = count, "Grid scan complete");
        count
    }
}

#[instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let grid = Grid::new(input);
    let result = grid.count_accessible_rolls();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test_log::test]
    fn test_grid_initialization_and_padding() {
        let input = "@.\n.@";
        let grid = Grid::new(input);

        // Verify Dimensions
        assert_eq!(grid.width, 4, "Width should be original (2) + padding (2)");
        assert_eq!(grid.height, 4, "Width should be original (2) + padding (2)");
        assert_eq!(grid.data.len(), 16, "Data vector should be 4*4 size");

        // Verify Padding (Border should be '.')
        assert_eq!(grid.data[0], b'.', "Top-left padding should be a dot");
        assert_eq!(grid.data[15], b'.', "bottom-right padding should be a dot");

        // Verify Data Placement (Offset by 1, 1)
        let idx_0_0 = 1 * grid.width + 1;
        assert_eq!(
            grid.data[idx_0_0], b'@',
            "Original (0, 0) should be at padded (1, 1)"
        );
        let idx_1_1 = 2 * grid.width + 2;
        assert_eq!(
            grid.data[idx_1_1], b'@',
            "Original (1, 1) should be at padded (2, 2)"
        );

        let idx_0_1 = 1 * grid.width + 2;
        assert_eq!(
            grid.data[idx_0_1], b'.',
            "Original (0, 1) should be at padded (1, 2)"
        );
    }

    #[test_log::test]
    fn test_grid_initialization_empty() {
        let grid = Grid::new("");
        assert_eq!(grid.height, 0);
        assert_eq!(grid.width, 0);
        assert!(grid.data.is_empty());
    }

    #[test_log::test(rstest)]
    #[case("@@@\n@@@\n@@@", 4)] // Dense block. Center has 8 neighbors, Edges have 5. Only 4 corners have 3 (<4).
    #[case("...", 0)] // No paper rolls
    #[case("@", 1)] // Single roll (0 neighbors < 4) -> Accessible
    #[case("@@", 2)] // Two rolls (1 neighbor each < 4) -> Both accessible
    #[case("@.@\n.@.\n@.@", 4)] // Cross shape. Center has 4 neighbors (==4, not <4) -> Not accessible. Leaves (1 neighbor) -> Accessible.
    fn test_count_accessible_rolls(#[case] input: &str, #[case] expected: usize) {
        let grid = Grid::new(input);
        let result = grid.count_accessible_rolls();
        assert_eq!(result, expected, "Failed logic check for input:\n{}", input);
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!("13", process(input)?);
        Ok(())
    }
}
