use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, instructions) = parse(input).map_err(|e| miette!("parse failed {e}"))?;
    let result: u32 = instructions
        .iter()
        .map(|instruction| match instruction {
            Instruction::Mul(a, b) => a * b,
        })
        .sum();
    Ok(result.to_string())
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Mul(u32, u32),
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, parse_mul_instruction).map(|(_, instruction)| instruction))(input)
}

fn parse_mul_instruction(input: &str) -> IResult<&str, Instruction> {
    let (pair, _) = tag("mul")(input)?;
    let (remaining, (multiplier, multiplicand)) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(pair)?;

    Ok((remaining, Instruction::Mul(multiplier, multiplicand)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn can_parse_correct_multiply_instruction() {
        let input = "mul(123,4)";
        let (_, parsed) = parse_mul_instruction(input).expect("should parse ");
        assert_eq!(parsed, Instruction::Mul(123, 4));
    }

    #[test]
    fn should_raise_error_when_contain_space() {
        let input = "mul(123, 4)";
        let parsing_error = parse_mul_instruction(input);
        assert!(parsing_error.is_err());
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("161", process(EXAMPLE)?);
        Ok(())
    }
}
