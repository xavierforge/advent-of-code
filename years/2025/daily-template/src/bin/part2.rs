use {{crate_name}}::part2::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    let file = include_str!("../../input2.txt");
    let daily_result = process(file).context("Process part2")?;
    println!("{daily_result}");
    Ok(())
}
