use std::collections::HashMap;

use tracing::instrument;

use crate::part1::{Graph, parse_input};

fn count_paths_between<'a>(
    current: &'a str,
    target: &'a str,
    graph: &'a Graph,
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if current == target {
        return 1;
    }
    if current == "out" && target != "out" {
        return 0;
    }
    if let Some(&count) = memo.get(current) {
        return count;
    }

    let mut total_paths = 0;

    if let Some(children) = graph.get(current) {
        for child in children {
            total_paths += count_paths_between(child, target, graph, memo)
        }
    }

    memo.insert(current, total_paths);
    total_paths
}

#[instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let graph = parse_input(input);
    let count = |start, end| {
        let mut memo = HashMap::new();
        count_paths_between(start, end, &graph, &mut memo)
    };

    let p_svr_dac = count("svr", "dac");
    let p_dac_fft = count("dac", "fft");
    let p_fft_out = count("fft", "out");
    let p_svr_fft = count("svr", "fft");
    let p_fft_dac = count("fft", "dac");
    let p_dac_out = count("dac", "out");

    let total = (p_svr_dac * p_dac_fft * p_fft_out) + (p_svr_fft * p_fft_dac * p_dac_out);

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_paths_between_nodes() {
        // A -> B -> C
        let mut graph = Graph::new();
        graph.insert("A".to_string(), vec!["B".to_string()]);
        graph.insert("B".to_string(), vec!["C".to_string()]);

        let mut memo = HashMap::new();
        assert_eq!(count_paths_between("A", "C", &graph, &mut memo), 1);

        memo.clear();
        assert_eq!(count_paths_between("A", "B", &graph, &mut memo), 1);

        memo.clear();
        assert_eq!(count_paths_between("A", "Z", &graph, &mut memo), 0); // 不存在
    }

    #[test_log::test]
    fn test_reverse_order() {
        // svr -> dac -> fft -> out
        let input = "svr: dac
dac: fft
fft: out";
        assert_eq!("1", process(input).unwrap());
    }

    #[test]
    fn test_disjoint_paths() {
        // svr -> dac -> out
        // svr -> fft -> out
        let input = "svr: dac fft
dac: out
fft: out";
        assert_eq!("0", process(input).unwrap());
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
