use std::collections::{HashSet, VecDeque};

use tracing::{info, instrument};

#[derive(Debug)]
struct Machine {
    target_state: u32,
    buttons: Vec<u32>,
}

fn parse_light_pattern(input: &str) -> u32 {
    let content = input
        .strip_prefix('[')
        .and_then(|s| s.strip_suffix(']'))
        .expect("Invalid light pattern format! Missing brackets.");

    content
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .fold(0, |acc, (i, _)| acc | (1 << i))
}

fn parse_buttons(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|token| {
            let inner = token
                .strip_prefix('(')
                .and_then(|s| s.strip_suffix(')'))
                .expect("Invalid button format!");
            inner.split(',').map(|n| n.trim()).fold(0u32, |acc, n_str| {
                let bit = n_str.parse::<u32>().expect("Invalid bit index");
                acc | (1 << bit)
            })
        })
        .collect()
}

fn parse_line(line: &str) -> Option<Machine> {
    let target_end = line.find(']')?;
    let joltage_start = line.find('{')?;

    if target_end >= joltage_start {
        return None;
    }

    let target_str = &line[..=target_end];
    let buttons_chunk = line[(target_end + 1)..joltage_start].trim();

    Some(Machine {
        target_state: parse_light_pattern(target_str),
        buttons: parse_buttons(buttons_chunk),
    })
}

fn solve_machine(machine: &Machine) -> Option<usize> {
    if machine.target_state == 0 {
        return Some(0);
    }

    let mut queue = VecDeque::from([(0u32, 0u32)]);
    let mut visited = HashSet::from([0u32]);

    while let Some((state, dist)) = queue.pop_front() {
        for button in &machine.buttons {
            let new_state = state ^ button;
            if new_state == machine.target_state {
                return Some((dist + 1) as usize);
            }

            if visited.insert(new_state) {
                queue.push_back((new_state, dist + 1));
            }
        }
    }
    None
}

#[instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let mut total_presses = 0;

    for line in input.lines() {
        if let Some(machine) = parse_line(line) {
            match solve_machine(&machine) {
                Some(presses) => total_presses += presses,
                None => {
                    info!("No solution found for machine: {:?}", machine);
                }
            }
        }
    }

    Ok(total_presses.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_parse_light_pattern() {
        // Index 0: ., Index 1: #, Index 2: #, Index 3: .
        // Value = 2^1 + 2^2 = 6
        assert_eq!(parse_light_pattern("[.##.]"), 6);

        // Index 0: #
        // Value = 2^0 = 1
        assert_eq!(parse_light_pattern("[#....]"), 1);

        // All off
        assert_eq!(parse_light_pattern("[....]"), 0);
    }

    #[test_log::test]
    fn test_parse_buttons() {
        let input = "(1,3) (2)";
        let buttons = parse_buttons(input);

        assert_eq!(buttons.len(), 2);
        // bits 1 and 3 -> 2^1 + 2^3 = 10
        assert_eq!(buttons[0], 10);
        // bits 2 -> 2^2 = 4
        assert_eq!(buttons[1], 4);
    }

    #[test_log::test]
    fn test_parse_line_full() {
        let line = "[.##.] (3) (1,3) {ignore me}";
        let machine = parse_line(line).expect("Should parse valid line");

        assert_eq!(machine.target_state, 6); // [.##.]
        assert_eq!(machine.buttons[0], 8); // (3) -> 2^3
        assert_eq!(machine.buttons[1], 10); // (1,3) -> 10
    }

    #[test_log::test]
    fn test_solve_simple_one_step() {
        // Target: 4 (binary 100, index 2 on)
        // Button: 4 (toggles index 2)
        // Expected: 1 press
        let machine = Machine {
            target_state: 4,
            buttons: vec![4],
        };
        assert_eq!(solve_machine(&machine), Some(1));
    }

    #[test_log::test]
    fn test_solve_two_steps() {
        // Target: 6 (binary 110, index 1,2 on)
        // Button A: 2 (toggles index 1)
        // Button B: 4 (toggles index 2)
        // Expected: Press A then B (or B then A) -> 2 steps
        let machine = Machine {
            target_state: 6,
            buttons: vec![2, 4],
        };
        assert_eq!(solve_machine(&machine), Some(2));
    }

    #[test_log::test]
    fn test_solve_with_interference() {
        // Target: 1 (index 0 on)
        // Button A: 3 (toggles 0, 1) -> state 11
        // Button B: 2 (toggles 1)    -> state 10
        // Expected: Press A (11) then B (10) -> leaves 01 (Value 1) -> 2 steps
        let machine = Machine {
            target_state: 1,
            buttons: vec![3, 2],
        };
        assert_eq!(solve_machine(&machine), Some(2));
    }

    #[test_log::test]
    fn test_solve_already_solved() {
        // Target is 0 (all off), start is 0
        // Expected: 0 presses
        let machine = Machine {
            target_state: 0,
            buttons: vec![1, 2, 4],
        };
        assert_eq!(solve_machine(&machine), Some(0));
    }

    #[test_log::test]
    fn test_solve_impossible() {
        // Target: 1 (index 0 on)
        // Button: 2 (toggles index 1 only)
        // Impossible to reach state 1
        let machine = Machine {
            target_state: 1,
            buttons: vec![2],
        };
        assert_eq!(solve_machine(&machine), None);
    }

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!("7", process(input)?);
        Ok(())
    }
}
