use itertools::Itertools;
use std::cmp::Ordering;

pub type Report = Vec<u32>;

pub mod parser {
    use super::Report;

    pub fn parse_line_to_report(line: &str) -> Report {
        line.split_whitespace()
            .map(|num| num.parse::<u32>().expect("Each value should parse to u32"))
            .collect()
    }

    pub fn parse_multiline_input(input: &str) -> Vec<Report> {
        input.lines().map(parse_line_to_report).collect()
    }
}

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

    report
        .iter()
        .tuple_windows()
        .all(|(a, b)| match initial_trending {
            LevelTrending::Decreasing => a > b,
            LevelTrending::Increasing => a < b,
        })
}

fn validate_differ_range(report: &Report) -> bool {
    report
        .iter()
        .tuple_windows()
        .all(|(a, b)| (1..=3).contains(&a.abs_diff(*b)))
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let reports = parser::parse_multiline_input(_input);
    let safe_count = reports
        .iter()
        .filter(|report| validate_differ_range(report) && validate_trending(report))
        .count();
    Ok(safe_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::part1::parser::{parse_line_to_report, parse_multiline_input};

    const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn can_parse_line_into_report() {
        let input = "7 6 4 2 1";
        let report = parse_line_to_report(input);
        assert_eq!(report, vec![7, 6, 4, 2, 1]);
    }

    #[test]
    fn can_parse_multiline_input() {
        let reports = parse_multiline_input(EXAMPLE);
        assert_eq!(
            reports,
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ]
        )
    }

    #[test]
    fn should_be_monotonic() {
        let is_monotonic = EXAMPLE
            .lines()
            .map(parse_line_to_report)
            .map(|line| validate_trending(&line))
            .collect::<Vec<bool>>();
        assert_eq!(is_monotonic, vec![true, true, true, false, false, true])
    }

    #[test]
    fn should_differ_in_range() {
        let is_within_range = EXAMPLE
            .lines()
            .map(parse_line_to_report)
            .map(|line| validate_differ_range(&line))
            .collect::<Vec<bool>>();
        assert_eq!(is_within_range, vec![true, false, false, true, false, true])
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("2", process(EXAMPLE)?);
        Ok(())
    }
}
