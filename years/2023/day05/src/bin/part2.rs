use itertools::Itertools;
fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct Map {
    src_start: u64,
    src_end: u64,
    target_start: u64,
}

fn parse(input: &str) -> (Vec<u64>, Vec<Vec<Map>>) {
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let seeds: Vec<u64> = seeds
        .split_whitespace()
        .skip(1)
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect();
    let seeds = seeds
        .into_iter()
        .tuples()
        .map(|(a, len)| (a, a + len))
        .collect::<Vec<_>>();
    println!("{seeds:?}");

    let maps = maps
        .trim_end()
        .split("\n\n")
        .map(|map| {
            map.split("\n")
                .skip(1)
                .map(|line| {
                    let line: Vec<u64> = line
                        .split_whitespace()
                        .map(|num| num.parse::<u64>().unwrap())
                        .collect();
                    Map {
                        src_start: line[1],
                        src_end: line[1] + line[2],
                        target_start: line[0],
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (seeds, maps)
}

fn process(input: &str) -> u64 {
    let (seeds, maps) = parse(input);
    let locations = maps.iter().fold(seeds, |seeds, map_stages| {
        seeds
            .into_iter()
            .map(|seed| {
                map_stages
                    .iter()
                    .find(|map| (map.src_start..map.src_end).contains(&seed))
                    .map(|map| map.target_start + (seed - map.src_start))
                    .unwrap_or(seed)
            })
            .collect()
    });
    *locations.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = process(input);
        assert_eq!(result, 46);
    }
}
