use std::collections::HashSet;

use tracing::{info, instrument};

type Position = (usize, usize);

struct TachyonLab {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
    start: Position,
}

impl TachyonLab {
    fn new(input: &str) -> Self {
        let grid = Self::parse_grid(input);
        let height = grid.len();
        let width = grid.first().map(|l| l.len()).unwrap_or(0);
        let start = Self::find_start(&grid).unwrap_or((0, 0));

        info!(width, height, ?start, "Lab initialized");

        Self {
            grid,
            width,
            height,
            start,
        }
    }

    fn parse_grid(input: &str) -> Vec<Vec<char>> {
        input.lines().map(|l| l.chars().collect()).collect()
    }

    fn find_start(grid: &[Vec<char>]) -> Option<Position> {
        for (r, row) in grid.iter().enumerate() {
            for (c, &ch) in row.iter().enumerate() {
                if ch == 'S' {
                    return Some((r, c));
                }
            }
        }
        None
    }

    fn is_in_bounds(&self, pos: Position) -> bool {
        pos.0 < self.height && pos.1 < self.width
    }

    /// Core Physics Logic: Determines the next positions based on current state.
    /// Returns: (Next Positions to visit, Was a splitter activated?)
    /// This function is PURE regarding the simulation state (visited set).
    fn calculate_next_moves(&self, current: Position) -> (Vec<Position>, bool) {
        let (r, c) = current;
        let next_r = r + 1;

        if !self.is_in_bounds((next_r, c)) {
            return (vec![], false);
        }

        let tile = self.grid[next_r][c];
        let mut next_positions = Vec::new();
        let mut splitter_hit = false;

        match tile {
            '.' | 'S' => {
                next_positions.push((next_r, c));
            }
            '^' => {
                splitter_hit = true;

                // Try split left
                if c > 0 {
                    next_positions.push((next_r, c - 1));
                }
                if c + 1 < self.width {
                    next_positions.push((next_r, c + 1));
                }
            }
            _ => {
                // Obstacles or unknown chars: beam stops
            }
        }

        (next_positions, splitter_hit)
    }

    fn count_splits(&self) -> usize {
        let mut visited = HashSet::new();
        let mut activated_splitters = HashSet::new();
        let mut stack = Vec::new();

        stack.push(self.start);
        visited.insert(self.start);

        while let Some(current) = stack.pop() {
            let (next_moves, hit_splitter) = self.calculate_next_moves(current);

            // Record splitter activation logic
            // Note: If hit_splitter is true, it means the *tile below* is a splitter.
            // The splitter's coordinate is (current.0 + 1, current.1)
            if hit_splitter {
                activated_splitters.insert((current.0 + 1, current.1));
            }

            // Process next moves
            for pos in next_moves {
                if visited.insert(pos) {
                    stack.push(pos);
                }
            }
        }

        let total = activated_splitters.len();
        info!(total, "Simulation finished");
        total
    }
}

#[instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let lab = TachyonLab::new(input);
    let result = lab.count_splits();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_parse_grid() {
        let input = "S.\n.^";
        let grid = TachyonLab::parse_grid(input);
        assert_eq!(grid.len(), 2);
        assert_eq!(grid[0], vec!['S', '.']);
        assert_eq!(grid[1], vec!['.', '^']);
    }

    #[test_log::test]
    fn test_find_start() {
        let grid = vec![vec!['.', '.'], vec!['.', 'S']];
        assert_eq!(TachyonLab::find_start(&grid), Some((1, 1)))
    }

    #[test_log::test]
    fn test_physics_interaction() {
        // Setup a mini lab manually
        // S.
        // .^  <-- (1, 1) is a splitter
        // ..
        let grid = vec![vec!['S', '.'], vec!['.', '^'], vec!['.', '.']];
        let lab = TachyonLab {
            grid,
            width: 2,
            height: 3,
            start: (0, 0),
        };

        // Scenario 1: Beam at (0, 0) (The 'S'). Moving down to (1, 0) which is '.'.
        // Expectation: Beam moves to (1, 0). No splitter.
        let (moves, hit) = lab.calculate_next_moves((0, 0));
        assert_eq!(moves, vec![(1, 0)]);
        assert_eq!(hit, false);

        // Scenario 2: Beam at (0, 1) (The '.'). Moving down to (1, 1) which is '^'.
        // Expectation: Beam stops, splits to (1, 0) [Left] and (1, 2) [Right - Out of bounds].
        // Wait, width is 2. (1, 2) is out of bounds. So only (1, 0).
        let (moves, hit) = lab.calculate_next_moves((0, 1));
        assert_eq!(moves, vec![(1, 0)]); // Only left is valid
        assert_eq!(hit, true);
    }

    #[test_log::test]
    fn test_physics_out_of_bounds() {
        let grid = vec![vec!['.']]; // 1x1 grid
        let lab = TachyonLab {
            grid,
            width: 1,
            height: 1,
            start: (0, 0),
        };

        // Beam at (0,0), tries to go to (1,0). (1,0) is out of bounds.
        let (moves, hit) = lab.calculate_next_moves((0, 0));
        assert!(moves.is_empty());
        assert!(!hit);
    }

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!("21", process(input)?);
        Ok(())
    }
}
