use std::collections::HashSet;

use crate::part1::parse_range;

fn generate_repeated_id(seed_str: &str, times: usize) -> Option<u64> {
    let repeated_str = seed_str.repeat(times);
    repeated_str.parse::<u64>().ok()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let ranges = parse_range(input);
    if ranges.is_empty() {
        return Ok("0".to_string());
    }

    let max_bound = ranges.iter().map(|r| r.end).max().unwrap_or(0);
    let mut unique_invalid_ids: HashSet<u64> = HashSet::new();
    let mut seed: u64 = 1;

    loop {
        let seed_str = seed.to_string();

        if let Some(min_val) = generate_repeated_id(&seed_str, 2) {
            if min_val > max_bound {
                break;
            }
        } else {
            break;
        }

        let mut k = 2;
        while let Some(candidate) = generate_repeated_id(&seed_str, k) {
            if candidate > max_bound {
                break;
            }

            if ranges.iter().any(|range| range.contains(candidate)) {
                unique_invalid_ids.insert(candidate);
            }
            k += 1;
        }

        seed += 1;
    }

    let result = unique_invalid_ids.iter().sum::<u64>();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_repeated_id() {
        assert_eq!(generate_repeated_id("1", 7), Some(1111111));
        assert_eq!(generate_repeated_id("12", 3), Some(121212));
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
        assert_eq!("4174379265", process(input)?);
        Ok(())
    }
}
