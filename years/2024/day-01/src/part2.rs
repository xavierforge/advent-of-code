use crate::part1::split_into_lists;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (left_list, right_list) = split_into_lists(input);

    let similarity: usize = left_list
        .iter()
        .map(|&left_number| {
            let matching_count = right_list
                .iter()
                .filter(|&&right_number| right_number == left_number)
                .count();
            matching_count * left_number as usize
        })
        .sum();

    Ok(similarity.to_string())
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
    fn test_process() -> miette::Result<()> {
        assert_eq!("31", process(EXAMPLE)?);
        Ok(())
    }
}
