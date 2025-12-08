use crate::part1::{Dsu, Point3D, generate_sorted_edges};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let points: Vec<Point3D> = input
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() < 3 {
                return None;
            }
            Some(Point3D {
                x: parts[0].trim().parse().ok()?,
                y: parts[1].trim().parse().ok()?,
                z: parts[2].trim().parse().ok()?,
            })
        })
        .collect();

    let n = points.len();

    let edges = generate_sorted_edges(&points);

    let mut dsu = Dsu::new(n);

    let target_merges = n - 1;
    let mut current_merges = 0;

    for edge in edges {
        if dsu.union(edge.u, edge.v) {
            current_merges += 1;
            if current_merges == target_merges {
                let p1 = &points[edge.u];
                let p2 = &points[edge.v];

                let result = p1.x * p2.x;
                return Ok(result.to_string());
            }
        }
    }

    Err(miette::miette!("Failed to connect all points"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process_example_with_limit_10() -> miette::Result<()> {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(process(input)?, "25272");
        Ok(())
    }
}
