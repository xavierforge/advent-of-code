use std::cmp::{max, min};

use tracing::{info, instrument};

use crate::part1::{Point, parse_input};

struct Rect {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl Rect {
    fn from_points(p1: Point, p2: Point) -> Self {
        Self {
            min_x: min(p1.x, p2.x),
            max_x: max(p1.x, p2.x),
            min_y: min(p1.y, p2.y),
            max_y: max(p1.y, p2.y),
        }
    }

    fn is_cut_by_segment(&self, start: Point, end: Point) -> bool {
        let is_vertical = start.x == end.x;

        if is_vertical {
            let x = start.x;
            let x_inside = x > self.min_x && x < self.max_x;

            let seg_min = min(start.y, end.y);
            let seg_max = max(start.y, end.y);
            let y_overlap = seg_min < self.max_y && seg_max > self.min_y;

            x_inside && y_overlap
        } else {
            let y = start.y;
            let y_inside = y > self.min_y && y < self.max_y;

            let seg_min = min(start.x, end.x);
            let seg_max = max(start.x, end.x);
            let x_overlap = seg_min < self.max_x && seg_max > self.min_x;

            y_inside && x_overlap
        }
    }
}

struct Polygon {
    vertices: Vec<Point>,
}

impl Polygon {
    fn new(vertices: Vec<Point>) -> Self {
        Self { vertices }
    }

    fn contains_center_of(&self, rect: &Rect) -> bool {
        let mid_x = (rect.min_x as f64 + rect.max_x as f64) / 2.0;
        let mid_y = (rect.min_y as f64 + rect.max_y as f64) / 2.0;

        let mut inside = false;
        let n = self.vertices.len();

        for i in 0..n {
            let p1 = self.vertices[i];
            let p2 = self.vertices[(i + 1) % n]; // Make sure the list is wrapped

            let p1y = p1.y as f64;
            let p2y = p2.y as f64;
            let p1x = p1.x as f64;
            let p2x = p2.x as f64;

            // Ray casting logic:
            // 1. Check if the edge's Y-range spans the center point's Y (must straddle the line).
            // 2. Check if there is an intersection to the "right" of the center point.
            if (p1y > mid_y) != (p2y > mid_y) {
                let intersect_x = (p2x - p1x) * (mid_y - p1y) / (p2y - p1y) + p1x;
                if mid_x < intersect_x {
                    inside = !inside;
                }
            }
        }
        inside
    }

    fn boundaries_cut_through(&self, rect: &Rect) -> bool {
        let n = self.vertices.len();
        for i in 0..n {
            let p1 = self.vertices[i];
            let p2 = self.vertices[(i + 1) % n];

            if rect.is_cut_by_segment(p1, p2) {
                return true;
            }
        }
        false
    }

    fn fully_contains_rect(&self, rect: &Rect) -> bool {
        self.contains_center_of(rect) && !self.boundaries_cut_through(rect)
    }
}

#[instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let points = parse_input(input);
    let n = points.len();

    let polygon = Polygon::new(points.clone());
    let mut max_area = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let p1 = points[i];
            let p2 = points[j];

            let area = p1.area_with(&p2);

            if area <= max_area {
                continue;
            }

            let rect = Rect::from_points(p1, p2);
            if polygon.fully_contains_rect(&rect) {
                max_area = area;
            }
        }
    }

    info!(max_area, "Calculation complete");
    Ok(max_area.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_interior_intersection() {
        let rect = Rect {
            min_x: 0,
            max_x: 10,
            min_y: 0,
            max_y: 10,
        };

        // Wall cut through
        assert!(rect.is_cut_by_segment(Point::new(5, -5), Point::new(5, 15)));
        // Wall on boundary
        assert!(!rect.is_cut_by_segment(Point::new(0, -5), Point::new(0, 15)));
        // Wall completely outside
        assert!(!rect.is_cut_by_segment(Point::new(20, -5), Point::new(20, 15)));
    }

    #[test_log::test]
    fn test_ray_casting_c_shape() {
        // Create a C shape
        // (0,0) -> (4,0) -> (4,1) -> (1,1) -> (1,3) -> (4,3) -> (4,4) -> (0,4)
        let vertices = vec![
            Point::new(0, 0),
            Point::new(4, 0),
            Point::new(4, 1),
            Point::new(1, 1),
            Point::new(1, 3),
            Point::new(4, 3),
            Point::new(4, 4),
            Point::new(0, 4),
        ];

        let poly = Polygon::new(vertices);

        let rect = Rect {
            min_x: 0,
            max_x: 4,
            min_y: 0,
            max_y: 4,
        };

        assert!(!poly.contains_center_of(&rect));
    }

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "7,1
    11,1
    11,7
    9,7
    9,5
    2,5
    2,3
    7,3";
        assert_eq!("24", process(input)?);
        Ok(())
    }
}
