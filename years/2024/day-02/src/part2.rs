use itertools::Itertools;
use std::{cmp::Ordering, ops::RangeBounds};

use crate::part1::{parser::*, Report};

#[derive(Debug)]
enum LevelTrending {
    Increasing,
    Decreasing,
}

fn validate_trending(report: &Report) -> bool {
    if report.len() < 2 {
        return false;
    }

    let initial_trending = match report[0].cmp(&report[1]) {
        Ordering::Greater => LevelTrending::Decreasing,
        Ordering::Less => LevelTrending::Increasing,
        Ordering::Equal => return false,
    };

    let mut violations = 0;
    for (a, b) in report.iter().tuple_windows() {
        let is_violation = match initial_trending {
            LevelTrending::Increasing => a > b,
            LevelTrending::Decreasing => a < b,
        };

        if is_violation {
            violations += 1;
            if violations > 1 {
                return false;
            }
        }
    }
    true
}

fn validate_differ_range(report: &Report) -> bool {
    let mut violations = 0;

    for (a, b) in report.iter().tuple_windows() {
        let diff = a.abs_diff(*b);
        println!("diff: {diff}");
        if !(1..=3).contains(&diff) {
            violations += 1;
            println!("violate");
            if violations > 1 {
                return false;
            }
        }
    }
    true
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let reports = parse_multiline_input(_input);
    let is_monotonic = reports.iter().map(validate_trending);
    let is_within_range = reports.iter().map(validate_differ_range);
    println!("{:?}", is_monotonic.clone().collect::<Vec<_>>());
    println!("{:?}", is_within_range.clone().collect::<Vec<_>>());
    let safe_count = is_monotonic
        .zip(is_within_range)
        .filter(|(a, b)| *a && *b)
        .count();
    Ok(safe_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("4", process(EXAMPLE)?);
        Ok(())
    }
}
