use crate::part1::{DIAL_SIZE, START_POSITION, TARGET_POSITION, parse_line_to_rotation};

pub fn process(input: &str) -> miette::Result<String> {
    let final_state = input.lines().map(parse_line_to_rotation).fold(
        (START_POSITION, 0),
        |(current_pos, total_hits), rotation| {
            let distance = rotation.abs();
            let direction = rotation.signum();
            let full_cycles_hits = distance / DIAL_SIZE;
            let remainder = distance % DIAL_SIZE;

            let (next_pos, remainder_hits) =
                (0..remainder).fold((current_pos, 0), |(pos, hits), _| {
                    let next = (pos + direction).rem_euclid(DIAL_SIZE);
                    let hit = if next == TARGET_POSITION { 1 } else { 0 };
                    (next, hits + hit)
                });
            (next_pos, total_hits + full_cycles_hits + remainder_hits)
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
    fn test_large_rotation() {
        // Start 50, R1000 should hit 0 ten times
        // 50 + 1000 = 1050. 1050 % 100 = 50. End position is 50.
        // It passes 0 at: 100, 200, ... 1000 (absolute coords if linear).
        let input = "R1000";
        let result = process(input).unwrap();
        assert_eq!(result, "10");
    }

    #[test]
    fn test_start_near_zero() {
        // Start 50.
        // Move to 1: L49. (No hit)
        // Move to 0: L1. (Hit)
        let input = "L49
L1";
        assert_eq!(process(input).unwrap(), "1");
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("6", process(EXAMPLE)?);
        Ok(())
    }
}
