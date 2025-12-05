use tracing::instrument;

use crate::part1::{InventorySystem, Range};

trait RangeCounter {
    fn count_ids_in_range(&self) -> u64;
}

impl RangeCounter for Range {
    fn count_ids_in_range(&self) -> u64 {
        self.end - self.start + 1
    }
}

#[instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let system = InventorySystem::new(input);
    let merged_range = system.merged_ranges;
    let total_count = merged_range
        .iter()
        .map(|&r| r.count_ids_in_range())
        .sum::<u64>();
    Ok(total_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!("14", process(input)?);
        Ok(())
    }
}
