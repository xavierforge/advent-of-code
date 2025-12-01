fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> u32 {
    let output = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" | ").map(|s| s.trim()).collect();
            let before_pipe: Vec<u32> = parts[0]
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            let after_pipe: Vec<u32> = parts[1]
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            // Find the intersection length
            let intersection_length = before_pipe
                .iter()
                .filter(|&x| after_pipe.contains(x))
                .count();
            let result = if intersection_length > 0 {
                2u32.pow((intersection_length - 1) as u32)
            } else {
                0
            };
            result
        })
        .sum();
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = process(input);
        assert_eq!(result, 13);
    }
}
