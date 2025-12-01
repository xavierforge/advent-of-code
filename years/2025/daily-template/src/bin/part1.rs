use {{crate_name}}::part1::process;
use miette::Context;

fn main() -> miette::Result<()> {
    let file = include_str!("../../input1.txt");
    let daily_result = process(file).context("Process part1")?;
    println!("{daily_result}");
    Ok(())
}
