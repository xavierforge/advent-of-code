use tracing::{debug, info, instrument};

use crate::part1::Grid;

// Extension trait to add optimization logic to Grid
trait GridPruner {
    fn prune_until_stable(&mut self) -> usize;
}

impl GridPruner for Grid {
    #[instrument(skip(self))]
    fn prune_until_stable(&mut self) -> usize {
        let mut totoal_removed = 0;
        let mut round = 0;
        let mut to_remove = Vec::new();

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

        loop {
            round += 1;
            to_remove.clear();

            for r in 1..self.height - 1 {
                for c in 1..self.width - 1 {
                    let idx = r * self.width + c;

                    if self.data[idx] != b'@' {
                        continue;
                    }

                    let mut neighbors = 0;
                    for (dr, dc) in offsets {
                        let n_idx = ((r as isize + dr) as usize) * self.width
                            + ((c as isize + dc) as usize);
                        if self.data[n_idx] == b'@' {
                            neighbors += 1;
                        }
                    }

                    if neighbors < 4 {
                        to_remove.push(idx);
                    }
                }
            }

            if to_remove.is_empty() {
                debug!("Grid stabilized after {} rounds", round - 1);
                break;
            }

            let count = to_remove.len();
            totoal_removed += count;

            for &idx in &to_remove {
                self.data[idx] = b'.'
            }
        }

        info!(totoal_removed, rounds = round, "Pruning complete");
        totoal_removed
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut grid = Grid::new(input);
    let result = grid.prune_until_stable();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!("43", process(input)?);
        Ok(())
    }
}
