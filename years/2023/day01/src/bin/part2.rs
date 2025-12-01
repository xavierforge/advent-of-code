fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> u32 {
    // Word mappings with tuples (original word, replacement)
    let word_mapping = vec![
        ("one", "one1one"),
        ("two", "two2two"),
        ("three", "three3three"),
        ("four", "four4four"),
        ("five", "five5five"),
        ("six", "six6six"),
        ("seven", "seven7seven"),
        ("eight", "eight8eight"),
        ("nine", "nine9nine"),
        ("zero", "zero0zero"),
    ];

    let output = input
        .lines()
        .map(|line| {
            let modified_line = word_mapping
                .iter()
                .fold(line.to_string(), |acc, (word, replacement)| {
                    acc.replace(word, replacement)
                });
            let mut iter = modified_line.chars().filter_map(|char| char.to_digit(10));
            let first = iter.next().expect("should be a number");
            let last = iter.last();

            match last {
                Some(num) => first * 10 + num,
                None => first * 10 + first,
            }
        })
        .sum::<u32>();
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = process(input);
        assert_eq!(result, 281);
    }
}
