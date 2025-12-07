use std::collections::HashMap;

use tracing::{info, instrument};

use crate::part1::{Position, TachyonLab};

trait QuantumManifold {
    fn count_timelines(&self) -> usize;
}

impl QuantumManifold for TachyonLab {
    #[instrument(skip(self))]
    fn count_timelines(&self) -> usize {
        let mut memo = HashMap::new();

        let total = self.solve_recursive(self.start, &mut memo);

        info!(total, "Quantum timeline analysis complete");
        total
    }
}

impl TachyonLab {
    fn solve_recursive(&self, current: Position, memo: &mut HashMap<Position, usize>) -> usize {
        if current.0 + 1 == self.height {
            return 1;
        }

        if let Some(&count) = memo.get(&current) {
            return count;
        }

        let (next_moves, _) = self.calculate_next_moves(current);

        let mut total_timelines = 0;

        if next_moves.is_empty() {
            total_timelines = 0;
        } else {
            for next_pos in next_moves {
                total_timelines += self.solve_recursive(next_pos, memo);
            }
        }

        memo.insert(current, total_timelines);
        total_timelines
    }
}

#[instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let lab = TachyonLab::new(input);
    let result = lab.count_timelines();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!("40", process(input)?);
        Ok(())
    }
}
