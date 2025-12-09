use tracing::{info, instrument};

struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn area_with(&self, other: &Point) -> i64 {
        let width = (self.x - other.x).abs() + 1;
        let height = (self.y - other.y).abs() + 1;
        width * height
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter_map(|line| {
            let (x_str, y_str) = line.split_once(',')?;
            Some(Point::new(
                x_str.trim().parse().ok()?,
                y_str.trim().parse().ok()?,
            ))
        })
        .collect()
}

#[instrument(skip(points))]
fn solve_largest_area(points: &[Point]) -> i64 {
    let n = points.len();
    if n < 2 {
        return 0;
    }

    let mut max_area = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let area = points[i].area_with(&points[j]);
            if area > max_area {
                max_area = area
            }
        }
    }

    max_area
}

#[instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let points = parse_input(input);
    let result = solve_largest_area(&points);

    info!(
        point_count = points.len(),
        result, "Largest rectangle found"
    );

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_area_calculation() {
        let p1 = Point::new(2, 5);
        let p2 = Point::new(9, 7);

        assert_eq!(p1.area_with(&p2), 24);
    }

    #[test_log::test]
    fn test_solve_largest_area_basic() {
        let points = vec![
            Point::new(0, 0),
            Point::new(1, 1),   // Area with (0,0) = 2*2 = 4
            Point::new(10, 10), // Area with (0,0) = 11*11 = 121
        ];

        assert_eq!(solve_largest_area(&points), 121);
    }

    #[test_log::test]
    fn test_solve_largest_area_insufficient_points() {
        // Case 1: Empty
        let points: Vec<Point> = vec![];
        assert_eq!(solve_largest_area(&points), 0);

        // Case 2: Single point
        let points = vec![Point::new(5, 5)];
        assert_eq!(solve_largest_area(&points), 0);
    }

    #[test_log::test]
    fn test_solve_largest_area_negative_coords() {
        let points = vec![Point::new(-2, -2), Point::new(2, 2)];

        // width = |-2 - 2| + 1 = 5
        // height = |-2 - 2| + 1 = 5
        // area = 25
        assert_eq!(solve_largest_area(&points), 25);
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!("50", process(input)?);
        Ok(())
    }
}
