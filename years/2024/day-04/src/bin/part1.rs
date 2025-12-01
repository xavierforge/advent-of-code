use day_04::part1::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input1.txt");
    let daily_result = process(file).context("Process part1")?;
    println!("{daily_result}");
    Ok(())
}
