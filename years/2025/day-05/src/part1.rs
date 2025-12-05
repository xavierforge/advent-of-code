use std::cmp::max;

use tracing::{info, instrument};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start: u64,
    pub end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Range { start, end }
    }

    fn contains(&self, value: u64) -> bool {
        value >= self.start && value <= self.end
    }
}

pub struct InventorySystem {
    pub merged_ranges: Vec<Range>,
    pub candidate_ids: Vec<u64>,
}

impl InventorySystem {
    #[instrument(skip(input))]
    pub fn new(input: &str) -> Self {
        let (range_block, id_block) = match input.split_once("\n\n") {
            Some((r, i)) => (r, i),
            None => (input, ""), // Handle edge case: only ranges, no IDs
        };

        let raw_ranges = Self::parse_ranges(range_block);
        let merged_ranges = Self::merge_intervals(raw_ranges);

        let candidate_ids = Self::parse_ids(id_block);

        info!(
            range_count = merged_ranges.len(),
            id_count = candidate_ids.len(),
            "System initialized"
        );

        Self {
            merged_ranges,
            candidate_ids,
        }
    }

    fn parse_ranges(input: &str) -> Vec<Range> {
        input
            .lines()
            .filter_map(|line| {
                let (start, end) = line.split_once('-')?;
                Some(Range::new(start.parse().ok()?, end.parse().ok()?))
            })
            .collect()
    }

    fn parse_ids(input: &str) -> Vec<u64> {
        input
            .lines()
            .filter_map(|line| line.parse::<u64>().ok())
            .collect()
    }

    fn merge_intervals(mut ranges: Vec<Range>) -> Vec<Range> {
        if ranges.is_empty() {
            return vec![];
        }

        ranges.sort_by_key(|r| r.start);

        let mut merged = Vec::new();
        let mut current = ranges[0];

        for next in ranges.into_iter().skip(1) {
            if next.start <= current.end {
                current.end = max(current.end, next.end);
            } else {
                merged.push(current);
                current = next;
            }
        }
        merged.push(current);

        merged
    }

    /// Logic: Binary search to check if an ID exists in the merged ranges.
    fn is_fresh(&self, id: u64) -> bool {
        // partition_point returns the index of the first element NOT satisfying the predicate.
        // We want ranges that start <= id.
        let idx = self.merged_ranges.partition_point(|r| r.start <= id);

        if idx == 0 {
            return false;
        }

        // Check the candidate range (the one immediately before the partition point)
        self.merged_ranges[idx - 1].contains(id)
    }

    fn count_fresh_ids(&self) -> usize {
        self.candidate_ids
            .iter()
            .filter(|&&id| self.is_fresh(id))
            .count()
    }
}

#[instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let system: InventorySystem = InventorySystem::new(input);
    let result = system.count_fresh_ids();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test_log::test(rstest)]
    #[case(10, 20, 15, true)]
    #[case(10, 20, 10, true)]
    #[case(10, 20, 20, true)]
    #[case(10, 20, 9, false)]
    #[case(10, 20, 21, false)]
    fn test_range_contains(
        #[case] start: u64,
        #[case] end: u64,
        #[case] val: u64,
        #[case] expected: bool,
    ) {
        let range = Range::new(start, end);
        assert_eq!(range.contains(val), expected)
    }

    #[test_log::test]
    fn test_parse_range_logic() {
        let input = "3-5\n10-14\ninvalid\n16-20";
        let result = InventorySystem::parse_ranges(input);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], Range::new(3, 5));
        assert_eq!(result[1], Range::new(10, 14));
        assert_eq!(result[2], Range::new(16, 20));
    }

    #[test_log::test]
    fn test_parse_ids_logic() {
        let input = "1\n5\nnot_a_number\n11";
        let result = InventorySystem::parse_ids(input);

        assert_eq!(result, vec![1, 5, 11])
    }

    #[test_log::test]
    fn test_merge_intervals_complex() {
        let input = vec![
            Range::new(1, 5),
            Range::new(2, 6),
            Range::new(8, 10),
            Range::new(15, 20),
            Range::new(16, 18),
        ];

        let merged = InventorySystem::merge_intervals(input);

        assert_eq!(merged.len(), 3);
        assert_eq!(merged[0], Range::new(1, 6));
        assert_eq!(merged[1], Range::new(8, 10));
        assert_eq!(merged[2], Range::new(15, 20));
    }

    #[test_log::test]
    fn test_merge_intervals_unordered() {
        let input = vec![
            Range::new(10, 15),
            Range::new(1, 5), // Should be sorted to first
        ];
        let merged = InventorySystem::merge_intervals(input);

        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0], Range::new(1, 5));
        assert_eq!(merged[1], Range::new(10, 15));
    }

    #[test_log::test(rstest)]
    #[case(4, true)]
    #[case(3, true)]
    #[case(5, true)]
    #[case(1, false)]
    #[case(8, false)]
    #[case(12, true)]
    #[case(32, false)]
    fn test_is_fresh_logic(#[case] id: u64, #[case] expected: bool) {
        let system = InventorySystem {
            merged_ranges: vec![Range::new(3, 5), Range::new(10, 14), Range::new(16, 20)],
            candidate_ids: Vec::new(),
        };

        assert_eq!(system.is_fresh(id), expected, "Failed for ID {}", id);
    }

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
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
