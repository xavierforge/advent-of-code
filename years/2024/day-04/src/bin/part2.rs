use day_04::part2::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input2.txt");
    let daily_result = process(file).context("Process part2")?;
    println!("{daily_result}");
    Ok(())
}
