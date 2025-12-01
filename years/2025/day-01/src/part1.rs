// 定義常數以避免 Magic Numbers
pub const DIAL_SIZE: i32 = 100;
pub const START_POSITION: i32 = 50;
pub const TARGET_POSITION: i32 = 0;

pub fn parse_line_to_rotation(line: &str) -> i32 {
    let mut chars = line.chars();
    let direction = chars.next();

    let value: i32 = chars.as_str().parse().expect("Should be a number");

    match direction {
        Some('R') => value,
        Some('L') => -value,
        _ => panic!("Unknown direction"),
    }
}

pub fn process(input: &str) -> miette::Result<String> {
    let final_state = input.lines().map(parse_line_to_rotation).fold(
        (START_POSITION, 0),
        |(current_pos, count), rotation| {
            let new_pos = (current_pos + rotation).rem_euclid(DIAL_SIZE);

            let new_count = if new_pos == TARGET_POSITION {
                count + 1
            } else {
                count
            };

            (new_pos, new_count)
        },
    );

    Ok(final_state.1.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_parse_one_line() {
        assert_eq!(parse_line_to_rotation("R76"), 76);
        assert_eq!(parse_line_to_rotation("L30"), -30);
    }

    #[test]
    fn test_euclidean_modulo_logic() {
        let pos: i32 = 0;
        let rotation: i32 = -1;
        assert_eq!((pos + rotation).rem_euclid(100), 99);
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("3", process(EXAMPLE)?);
        Ok(())
    }
}
