use tracing::{debug, info};

/// Calculates the maximum possible joltage for a single battery bank.
///
/// Logic:
/// 1. Find the largest digit in the range `0..len-1` (the "Tens" place).
///    - If there are duplicates, pick the leftmost one to maximize the remaining range.
/// 2. Find the largest digit in the range `idx+1..len` (the "Ones" place).
/// 3. Combine them: tens * 10 + ones.
fn parse_bank(input: &str) -> u32 {
    let bytes = input.as_bytes();
    let n = bytes.len();
    if n < 2 {
        debug!("Input too short to form a 2-digit number");
        return 0;
    }

    let (idx1, digit1) = (b'1'..=b'9')
        .rev()
        .find_map(|digit| {
            bytes[..n - 1]
                .iter()
                .position(|&b| b == digit)
                .map(|i| (i, digit - b'0'))
        })
        .unwrap_or_else(|| (0, bytes[0].saturating_sub(b'0')));

    debug!(tens = digit1, index = idx1, "Found best tens digit");

    let digit2 = bytes[idx1 + 1..]
        .iter()
        .max()
        .map(|&b| b - b'0')
        .unwrap_or(0);

    debug!(ones = digit2, "Found best ones digits");

    let result = (digit1 as u32) * 10 + (digit2 as u32);
    info!(result, "Bank parse complete");

    result
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let total_joltage: u32 = input.lines().map(parse_bank).sum();
    Ok(total_joltage.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test_log::test(rstest)]
    #[case("987654321111111", 98)] // Case 1: Max is at the very start
    #[case("811111111111119", 89)] // Case 2: Max 'ones' is at the very end (The Gap)
    #[case("234234234234278", 78)] // Case 3: Max pair is at the end
    #[case("818181911112111", 92)] // Case 4: Max 'tens' is in the middle
    #[case("12", 12)] // Edge Case: Minimum length
    #[case("8189", 89)] // Logic Check: Should pick first '8' (idx 0) to reach '9', not second '8'
    #[case("9195", 99)] // Logic Check: Should pick first '9' to allow finding the second '9'
    #[case("11111", 11)] // Flat values
    fn test_solve_bank_cases(#[case] input: &str, #[case] expected: u32) {
        let result = parse_bank(input);
        assert_eq!(result, expected, "Failed for input: {}", input);
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!("357", process(input)?);
        Ok(())
    }
}
