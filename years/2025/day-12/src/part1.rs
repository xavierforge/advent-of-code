use std::collections::HashSet;

use tracing::instrument;

// --- Data Structures ---

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Shape {
    id: usize,
    // (row, col) offsets relative to top-left (0,0)
    cells: Vec<(usize, usize)>,
    height: usize,
    width: usize,
}

impl Shape {
    /// Normalizes coordinates to (0,0) and sorts cells for consistent hashing.
    fn normalize(id: usize, cells: Vec<(usize, usize)>) -> Self {
        if cells.is_empty() {
            return Self {
                id,
                cells,
                height: 0,
                width: 0,
            };
        }

        // 1. Shift to top-left (0,0)
        let min_r = cells.iter().map(|&(r, _)| r).min().unwrap_or(0);
        let min_c = cells.iter().map(|&(_, c)| c).min().unwrap_or(0);
        let mut normalized_cells: Vec<(usize, usize)> =
            cells.iter().map(|&(r, c)| (r - min_r, c - min_c)).collect();

        // 2. Sort for deduplication
        normalized_cells.sort_unstable();

        let height = normalized_cells.iter().map(|&(r, _)| r).max().unwrap_or(0) + 1;
        let width = normalized_cells.iter().map(|&(_, c)| c).max().unwrap_or(0) + 1;

        Self {
            id,
            cells: normalized_cells,
            height,
            width,
        }
    }

    fn rotate(&self) -> Self {
        let new_cells = self
            .cells
            .iter()
            .map(|&(r, c)| (c, self.height - 1 - r))
            .collect();
        Self::normalize(self.id, new_cells)
    }

    fn horizontal_flip(&self) -> Self {
        let new_cells = self
            .cells
            .iter()
            .map(|&(r, c)| (r, self.width - 1 - c))
            .collect();
        Self::normalize(self.id, new_cells)
    }

    /// Generates all unique orientations (rotations + flips).
    fn generate_unique_variations(&self) -> Vec<Shape> {
        let mut unique = HashSet::new();
        let mut curr = self.clone();

        for _ in 0..4 {
            unique.insert(curr.clone());
            unique.insert(curr.horizontal_flip());
            curr = curr.rotate();
        }

        let mut result: Vec<_> = unique.into_iter().collect();
        result.sort(); // Deterministic order
        result
    }
}

#[derive(Debug)]
struct RegionTask {
    width: usize,
    height: usize,
    // (Shape_ID, Count)
    requirements: Vec<(usize, usize)>,
}

// --- Parsing ---

fn parse_shapes(input: &str) -> Vec<Shape> {
    let mut shapes = Vec::new();
    let mut current_id = None;
    let mut current_cells = Vec::new();
    let mut current_row = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Start of a shape block (e.g., "0:")
        if line.ends_with(':') && !line.contains('x') {
            if let Some(id) = current_id {
                shapes.push(Shape::normalize(id, current_cells));
                current_cells = Vec::new();
                current_row = 0;
            }
            if let Ok(id) = line.trim_end_matches(':').trim().parse::<usize>() {
                current_id = Some(id);
            }
        // Shape grid lines (e.g., "###", "..#")
        } else if current_id.is_some() && !line.contains(':') && !line.contains('x') {
            for (c, ch) in line.chars().enumerate() {
                if ch == '#' {
                    current_cells.push((current_row, c));
                }
            }
            current_row += 1;
        // Task lines (e.g., "4x4: ...") - marks end of shape definitions
        } else if line.contains("x")
            && let Some(id) = current_id
        {
            shapes.push(Shape::normalize(id, current_cells));
            current_id = None;
            current_cells = Vec::new();
        }
    }
    // Handle last shape if file ends without tasks
    if let Some(id) = current_id
        && !current_cells.is_empty()
    {
        shapes.push(Shape::normalize(id, current_cells));
    }
    shapes
}

fn parse_tasks(input: &str) -> Vec<RegionTask> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if !line.contains('x') {
                return None;
            }

            let (size_str, req_str) = line.split_once(':')?;
            let (w_str, h_str) = size_str.split_once('x')?;
            let width = w_str.trim().parse().ok()?;
            let height = h_str.trim().parse().ok()?;

            let requirements = req_str
                .split_whitespace()
                .enumerate()
                .filter_map(|(id, count_str)| {
                    let count = count_str.parse::<usize>().ok()?;
                    if count > 0 { Some((id, count)) } else { None }
                })
                .collect();

            Some(RegionTask {
                width,
                height,
                requirements,
            })
        })
        .collect()
}

// --- Solver Logic ---

#[inline]
fn can_place(grid: &[bool], w: usize, shape: &Shape, r: usize, c: usize) -> bool {
    // 1. Boundary check
    if r + shape.height > (grid.len() / w) || c + shape.width > w {
        return false;
    }
    // 2. Overlap check
    for &(dr, dc) in &shape.cells {
        if grid[(r + dr) * w + (c + dc)] {
            return false;
        }
    }
    true
}

#[inline]
fn toggle_shape(grid: &mut [bool], w: usize, shape: &Shape, r: usize, c: usize, val: bool) {
    for &(dr, dc) in &shape.cells {
        grid[(r + dr) * w + (c + dc)] = val;
    }
}

/// Recursive backtracking.
/// `items` contains indices into `variations_pool` for each item that needs placement.
fn backtrack(
    grid: &mut [bool],
    w: usize,
    h: usize,
    items: &[usize], // Flattened list of shapes to place
    item_idx: usize, // Current item index
    variations_pool: &[Vec<Shape>],
    last_pos: usize, // For symmetry breaking
) -> bool {
    // Base Case: All items placed
    if item_idx == items.len() {
        return true;
    }

    let pool_idx = items[item_idx];
    let variants = &variations_pool[pool_idx];

    // Symmetry Breaking:
    // If this item is the same type as the previous one, strictly start searching
    // from where the last one was placed. This avoids N! redundant permutations.
    let start_pos = if item_idx > 0 && items[item_idx] == items[item_idx - 1] {
        last_pos
    } else {
        0
    };

    let total_cells = w * h;

    // Try to place the shape at every valid position in the grid
    for pos in start_pos..total_cells {
        let r = pos / w;
        let c = pos % w;

        for variant in variants {
            if can_place(grid, w, variant, r, c) {
                toggle_shape(grid, w, variant, r, c, true);

                // Recurse
                // Pass `pos` as the next `last_pos` for symmetry breaking
                if backtrack(grid, w, h, items, item_idx + 1, variations_pool, pos) {
                    return true;
                }

                toggle_shape(grid, w, variant, r, c, false); // Backtrack
            }
        }
    }

    false
}

fn solve_region(task: &RegionTask, all_shapes: &[Shape]) -> bool {
    let total_cells = task.width * task.height;
    let mut grid = vec![false; total_cells];

    // Data prep for recursion
    // Tuples: (id, count, area, variations)
    let mut temp_reqs = Vec::new();
    let mut total_item_area = 0;

    for &(id, count) in &task.requirements {
        if let Some(base) = all_shapes.iter().find(|s| s.id == id) {
            let area = base.cells.len();
            let vars = base.generate_unique_variations();
            temp_reqs.push((id, count, area, vars));
            total_item_area += area * count;
        } else {
            return false; // Shape missing
        }
    }

    // Pruning: Area check
    if total_item_area > total_cells {
        return false;
    }

    // Optimization: Sort by size (Largest First)
    temp_reqs.sort_by(|a, b| b.2.cmp(&a.2));

    // Flatten requirements into a linear list of jobs
    let mut variations_pool = Vec::new();
    let mut items_to_place = Vec::new(); // Stores indices pointing to variations_pool

    for (pool_idx, (_, count, _, vars)) in temp_reqs.into_iter().enumerate() {
        variations_pool.push(vars);
        for _ in 0..count {
            items_to_place.push(pool_idx);
        }
    }

    backtrack(
        &mut grid,
        task.width,
        task.height,
        &items_to_place,
        0,
        &variations_pool,
        0,
    )
}

#[instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let shapes = parse_shapes(input);
    let tasks = parse_tasks(input);
    let mut success_count = 0;

    for task in tasks {
        if solve_region(&task, &shapes) {
            success_count += 1;
        }
    }

    Ok(success_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_parse_shape_0() {
        let input = "0:
###
##.
##.";
        let shapes = parse_shapes(input);
        assert_eq!(shapes.len(), 1);
        let s = &shapes[0];
        assert_eq!(s.id, 0);
        assert_eq!(s.width, 3);
        assert_eq!(s.height, 3);
        assert!(s.cells.contains(&(0, 0)));
        assert!(s.cells.contains(&(2, 1)));
        assert!(!s.cells.contains(&(1, 2)));
    }

    #[test_log::test]
    fn test_parse_multiple_shapes() {
        let input = "0:
###
..#
###

1:
..#
.##
##.

2:
#..
###
###";
        let shapes = parse_shapes(input);

        assert_eq!(shapes.len(), 3);

        // Shape 1 logic verification
        let s1 = &shapes[1]; // Note: shapes vector index might not match ID if sorted, but here input order is preserved
        assert_eq!(s1.id, 1);

        // After normalize:
        // Input:
        // ..# (0,2) -> min_c=0, min_r=0
        // .## (1,1), (1,2)
        // ##. (2,0), (2,1)
        // Normalized should maintain relative structure.

        assert!(s1.cells.contains(&(0, 2)));
        assert!(s1.cells.contains(&(1, 1)));
        assert!(s1.cells.contains(&(1, 2)));
        assert!(s1.cells.contains(&(2, 0)));
        assert!(s1.cells.contains(&(2, 1)));
    }

    #[test_log::test]
    fn test_parse_single_task_line() {
        let input = "12x5: 1 0 1 0 2 2";
        let tasks = parse_tasks(input);

        assert_eq!(tasks.len(), 1);
        let t = &tasks[0];
        assert_eq!(t.width, 12);
        assert_eq!(t.height, 5);

        let reqs = &t.requirements;
        assert!(reqs.contains(&(0, 1)));
        assert!(reqs.contains(&(2, 1)));
        assert!(reqs.contains(&(4, 2)));
        assert!(reqs.contains(&(5, 2)));

        assert!(!reqs.iter().any(|&(id, _)| id == 1));
        assert!(!reqs.iter().any(|&(id, _)| id == 3));
    }

    #[test_log::test]
    fn test_parse_multiple_task_lines() {
        let input = "4x4: 0 0 0 0 2 0\n10x10: 5 0";
        let tasks = parse_tasks(input);

        assert_eq!(tasks.len(), 2);

        assert_eq!(tasks[0].width, 4);
        assert_eq!(tasks[0].height, 4);
        assert_eq!(tasks[0].requirements, vec![(4, 2)]);

        assert_eq!(tasks[1].width, 10);
        assert_eq!(tasks[1].height, 10);
        assert_eq!(tasks[1].requirements, vec![(0, 5)]);
    }

    #[test_log::test]
    fn test_rotate_shape() {
        // ##
        // .#
        let shape = Shape::normalize(0, vec![(0, 0), (0, 1), (1, 1)]);
        let rotated = shape.rotate();

        // Original 2x2, rotated is still 2x2
        assert_eq!(rotated.height, 2);
        assert_eq!(rotated.width, 2);

        // Rotated logic: (r,c) -> (c, h-1-r)
        // (0,0) -> (0,1)
        // (0,1) -> (1,1)
        // (1,1) -> (1,0)
        assert!(rotated.cells.contains(&(0, 1)));
        assert!(rotated.cells.contains(&(1, 1)));
        assert!(rotated.cells.contains(&(1, 0)));
    }

    #[test_log::test]
    fn test_deduplication_square() {
        // A 2x2 square should only have 1 unique variation
        let s = Shape::normalize(0, vec![(0, 0), (0, 1), (1, 0), (1, 1)]);
        let vars = s.generate_unique_variations();
        assert_eq!(vars.len(), 1);
    }

    #[test_log::test]
    fn test_horizontal_flip_l_shape() {
        // #.  (0,0)
        // #.  (1,0)
        // ##  (2,0), (2,1)
        let l_shape = Shape::normalize(0, vec![(0, 0), (1, 0), (2, 0), (2, 1)]);
        let flipped = l_shape.horizontal_flip();

        // .#  (0,1)
        // .#  (1,1)
        // ##  (2,0), (2,1)
        assert!(flipped.cells.contains(&(0, 1)));
        assert!(flipped.cells.contains(&(1, 1)));
        assert!(flipped.cells.contains(&(2, 1)));
        assert!(flipped.cells.contains(&(2, 0)));
    }

    #[test_log::test]
    fn test_generate_variations_includes_flips() {
        let l_shape = Shape::normalize(0, vec![(0, 0), (1, 0), (2, 0), (2, 1)]);
        let vars = l_shape.generate_unique_variations();

        let has_j_shape = vars
            .iter()
            .any(|s| s.cells.contains(&(0, 1)) && s.cells.contains(&(2, 0)));

        assert!(
            has_j_shape,
            "Variations should include the horizontally flipped version"
        );
        // Note: Unique variations for L-shape is usually 8
        assert_eq!(vars.len(), 8);
    }

    #[test_log::test]
    fn test_process_full_example() {
        let input = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

        // This runs the full logic
        assert_eq!("2", process(input).unwrap());
    }
}
