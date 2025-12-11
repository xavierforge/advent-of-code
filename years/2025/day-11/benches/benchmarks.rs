use day_11::{part1, part2};
use divan::Bencher;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1_bench(bencher: Bencher) {
    bencher
        .with_inputs(|| include_str!("../input1.txt"))
        .bench_values(|s| part1::process(s));
}

#[divan::bench]
fn part2_bench(bencher: Bencher) {
    bencher
        .with_inputs(|| include_str!("../input2.txt"))
        .bench_values(|s| part2::process(s));
}
