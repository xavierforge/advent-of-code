use tracing::debug;

fn parse_bank_twelve(input: &str) -> u64 {
    let bytes = input.as_bytes();
    let n = bytes.len();
    let target_count = 12;

    if n < target_count {
        debug!("Input too short to form a {}-digit number", target_count);
        return 0;
    }

    let mut current_idx = 0;
    let mut result = 0;

    for i in 0..target_count {
        let remaining_needed = target_count - 1 - i;
        let search_end = n - remaining_needed;

        let (found_offset, digit_val) = (b'1'..=b'9')
            .rev()
            .find_map(|digit| {
                bytes[current_idx..search_end]
                    .iter()
                    .position(|&b| b == digit)
                    .map(|offset| (offset, digit - b'0'))
            })
            .unwrap_or_else(|| (0, bytes[current_idx] - b'0'));

        result = result * 10 + (digit_val as u64);

        let absolute_idx = current_idx + found_offset;
        current_idx = absolute_idx + 1;
    }
    result
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let total_joltage: u64 = input.lines().map(parse_bank_twelve).sum();
    Ok(total_joltage.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test_log::test(rstest)]
    #[case("987654321111111", 987654321111)]
    #[case("811111111111119", 811111111119)]
    #[case("234234234234278", 434234234278)]
    #[case("818181911112111", 888911112111)]
    fn test_parse_bank_twelve(#[case] input: &str, #[case] expected: u64) {
        let result = parse_bank_twelve(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!("3121910778619", process(input)?);
        Ok(())
    }
}
