use std::collections::HashMap;

use tracing::instrument;

type Graph = HashMap<String, Vec<String>>;

fn parse_input(input: &str) -> Graph {
    input
        .lines()
        .filter_map(|line| {
            let (source, dest) = line.split_once(": ")?;
            let destinations: Vec<String> =
                dest.split_whitespace().map(|s| s.to_string()).collect();
            Some((source.to_string(), destinations))
        })
        .collect()
}

fn count_paths<'a>(
    current: &'a str,
    graph: &'a Graph,
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if current == "out" {
        return 1;
    }

    if let Some(&count) = memo.get(current) {
        return count;
    }

    let mut total_paths = 0;

    if let Some(children) = graph.get(current) {
        for child in children {
            total_paths += count_paths(child.as_str(), graph, memo)
        }
    }

    memo.insert(current, total_paths);

    total_paths
}

#[instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let graph = parse_input(input);
    let mut memo = HashMap::new();
    let result = count_paths("you", &graph, &mut memo);
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_parse_simple() {
        let input = "aaa: bbb ccc\nbbb: out";
        let graph = parse_input(input);

        assert_eq!(
            graph.get("aaa"),
            Some(&vec!["bbb".to_string(), "ccc".to_string()])
        );
        assert_eq!(graph.get("bbb"), Some(&vec!["out".to_string()]));
        assert!(graph.get("ccc").is_none()); // ccc 沒有定義輸出，應該是 None 或不包含在 Key 中
    }

    #[test_log::test]
    fn test_count_direct_path() {
        let mut graph = Graph::new();
        graph.insert("you".to_string(), vec!["out".to_string()]);

        let mut memo = HashMap::new();
        assert_eq!(count_paths("you", &graph, &mut memo), 1);
    }

    #[test_log::test]
    fn test_count_dead_end() {
        let mut graph = Graph::new();
        graph.insert("you".to_string(), vec!["dead".to_string()]);
        graph.insert("dead".to_string(), vec![]);

        let mut memo = HashMap::new();
        assert_eq!(count_paths("you", &graph, &mut memo), 0);
    }

    #[test_log::test]
    fn test_diamond_structure() {
        let mut graph = Graph::new();
        graph.insert("you".to_string(), vec!["A".to_string(), "B".to_string()]);
        graph.insert("A".to_string(), vec!["out".to_string()]);
        graph.insert("B".to_string(), vec!["out".to_string()]);

        let mut memo = HashMap::new();
        assert_eq!(count_paths("you", &graph, &mut memo), 2);
    }

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        assert_eq!("5", process(input)?);
        Ok(())
    }
}
