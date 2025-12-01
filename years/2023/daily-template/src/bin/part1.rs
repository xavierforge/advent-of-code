fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    todo!("Let the challenge begin")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "";
        let result = process(input);
        assert_eq!(result, "");
    }
}
