fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> u32 {
    let output = input
        .lines()
        .map(|line| {
            let mut iter = line.chars().filter_map(|char| char.to_digit(10));
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
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = process(input);
        assert_eq!(result, 142);
    }
}
