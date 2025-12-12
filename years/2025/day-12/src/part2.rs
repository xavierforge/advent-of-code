#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    Ok("Merry Xmas! 🎄".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "";
        assert_eq!("Merry Xmas! 🎄", process(input)?);
        Ok(())
    }
}
