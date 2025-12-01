use std::str::FromStr;

fn parse_line_to_pair(line: &str) -> (u32, u32) {
    let parts = line
        .split_whitespace()
        .map(str::trim)
        .map(|num| u32::from_str(num).expect("Failed to parse number"))
        .collect::<Vec<u32>>();
    (parts[0], parts[1])
}

pub fn split_into_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    input.lines().map(parse_line_to_pair).unzip()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (mut left_list, mut right_list) = split_into_lists(input);
    left_list.sort_unstable();
    right_list.sort_unstable();

    let total_difference = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| left.abs_diff(*right))
        .sum::<u32>();
    Ok(total_difference.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn parse_one_digit_numbers() {
        let input = "3   4";
        let (left_number, right_number) = parse_line_to_pair(input);
        assert_eq!(left_number, 3);
        assert_eq!(right_number, 4);
    }

    #[test]
    fn parse_multi_digit_numbers() {
        let input = "2468   44";
        let (left_number, right_number) = parse_line_to_pair(input);
        assert_eq!(left_number, 2468);
        assert_eq!(right_number, 44);
    }

    #[test]
    fn parse_lines_as_two_lists_of_u32() {
        let (left_list, right_list) = split_into_lists(EXAMPLE);
        assert_eq!(left_list, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(right_list, vec![4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("11", process(EXAMPLE)?);
        Ok(())
    }
}
