use tracing::{debug, info, instrument};

struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }
        let root = self.find(self.parent[i]);
        self.parent[i] = root; // Path compression: point directly to root
        root
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i == root_j {
            return false;
        }

        if self.size[root_i] > self.size[root_j] {
            self.parent[root_j] = root_i;
            self.size[root_i] += self.size[root_j];
        } else {
            self.parent[root_i] = root_j;
            self.size[root_j] += self.size[root_i];
        }
        true
    }

    fn get_all_circuit_sizes(&mut self) -> Vec<usize> {
        let mut sizes = Vec::new();
        // We must iterate through all indices to find the roots
        // Note: We need to use 0..len because self is borrowed mutably inside
        let len = self.parent.len();

        for i in 0..len {
            // Check if i is a root
            if self.parent[i] == i {
                sizes.push(self.size[i]);
            }
        }

        // Sort descending
        sizes.sort_by(|a, b| b.cmp(a));
        sizes
    }
}

struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3D {
    fn dist_sq(&self, other: &Point3D) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

struct Edge {
    u: usize,
    v: usize,
    dist_sq: i64,
}

fn generate_sorted_edges(points: &[Point3D]) -> Vec<Edge> {
    let n = points.len();
    let mut edges = Vec::with_capacity(n * (n - 1) / 2);

    for i in 0..n {
        for j in (i + 1)..n {
            let d = points[i].dist_sq(&points[j]);
            edges.push(Edge {
                u: i,
                v: j,
                dist_sq: d,
            });
        }
    }

    edges.sort_by_key(|e| e.dist_sq);

    info!(total_edges = edges.len(), "Generated and sorted edges");
    edges
}

fn solve(input: &str, limit: usize) -> Option<usize> {
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
    if n == 0 {
        return Some(0);
    }

    let edges = generate_sorted_edges(&points);

    let mut dsu = Dsu::new(n);

    let actual_limit = limit.min(edges.len());

    let mut actual_merges = 0;
    for edge in edges.iter().take(actual_limit) {
        if dsu.union(edge.u, edge.v) {
            actual_merges += 1;
        }
    }

    debug!(
        limit = actual_limit,
        merges = actual_merges,
        "DSU pass complete"
    );

    let sizes = dsu.get_all_circuit_sizes();

    Some(sizes.iter().take(3).product())
}

#[instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let result = solve(input, 1000).unwrap_or(0);
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_dsu_initialization() {
        let dsu = Dsu::new(3);

        assert_eq!(dsu.parent, vec![0, 1, 2]);
        assert_eq!(dsu.size, vec![1, 1, 1]);
    }

    #[test_log::test]
    fn test_dsu_find() {
        // 0 -> 1 -> 2
        let mut dsu = Dsu::new(3);
        dsu.parent[0] = 1;
        dsu.parent[1] = 2;

        assert_eq!(dsu.find(0), 2);
    }

    #[test_log::test]
    fn test_dsu_simple_union() {
        let mut dsu = Dsu::new(2);

        assert!(dsu.union(0, 1), "Should be success for the first time");
        assert!(!dsu.union(0, 1), "Should fail when duplicate");
        assert!(!dsu.union(1, 0), "Should fail when duplicate");
    }

    #[test_log::test]
    fn test_dsu_union_by_size_logic() {
        let mut dsu = Dsu::new(10);

        dsu.union(0, 1);
        dsu.union(0, 2);
        let root_a = dsu.find(0);
        assert_eq!(dsu.size[root_a], 3);

        let root_b = dsu.find(3);
        assert_eq!(dsu.size[root_b], 1);

        dsu.union(3, 0);
        assert_eq!(
            dsu.parent[root_b], root_a,
            "Small root should point to big root"
        );
        assert_eq!(dsu.parent[root_a], root_a, "Big root should remain root");
        assert_eq!(dsu.size[root_a], 4);
    }

    #[test_log::test]
    fn test_dsu_path_compression() {
        let mut dsu = Dsu::new(5);
        // 0 -> 1 -> 2 -> 3 -> 4 (Root)
        dsu.parent = vec![1, 2, 3, 4, 4];
        assert_eq!(dsu.parent[0], 1);
        let root = dsu.find(0);
        assert_eq!(root, 4);

        assert_eq!(
            dsu.parent[0], 4,
            "Path should be compressed directly to root"
        );
        assert_eq!(
            dsu.parent[1], 4,
            "Intermediate nodes should also be compressed"
        );
        assert_eq!(dsu.parent[2], 4);
        assert_eq!(dsu.parent[3], 4);
    }

    #[test_log::test]
    fn test_get_all_circuit_sizes() {
        let mut dsu = Dsu::new(5);
        assert_eq!(dsu.get_all_circuit_sizes(), vec![1; 5]);
    }

    #[test_log::test]
    fn test_sorted_edges_logic() {
        let points = vec![
            Point3D { x: 0, y: 0, z: 0 },
            Point3D { x: 10, y: 0, z: 0 }, // dist_sq = 100
            Point3D { x: 2, y: 0, z: 0 },  // dist_sq = 4
        ];

        let edges = generate_sorted_edges(&points);

        assert_eq!(edges.len(), 3);

        // Expected order：
        // 1. (0, 2) dist 4
        // 2. (2, 1) dist 64 (10-2=8, 8^2=64)
        // 3. (0, 1) dist 100
        assert_eq!(edges[0].dist_sq, 4);
        assert_eq!(edges[1].dist_sq, 64);
        assert_eq!(edges[2].dist_sq, 100);
    }

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
        assert_eq!(solve(input, 10), Some(40));
        Ok(())
    }
}
