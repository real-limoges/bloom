/// Axis-aligned bounding box used by both the hit-testing quadtree
/// (`graph::spatial`) and the Barnes-Hut tree (`layout::barnes_hut`).
#[derive(Debug, Clone)]
pub struct AABB {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl AABB {
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }

    pub fn intersects_circle(&self, cx: f32, cy: f32, radius: f32) -> bool {
        let nearest_x = cx.clamp(self.min_x, self.max_x);
        let nearest_y = cy.clamp(self.min_y, self.max_y);
        let dx = cx - nearest_x;
        let dy = cy - nearest_y;
        dx * dx + dy * dy <= radius * radius
    }

    pub fn width(&self) -> f32 {
        self.max_x - self.min_x
    }

    pub fn height(&self) -> f32 {
        self.max_y - self.min_y
    }

    pub fn center(&self) -> (f32, f32) {
        (
            (self.min_x + self.max_x) / 2.0,
            (self.min_y + self.max_y) / 2.0,
        )
    }

    /// Compute the tight bounding box around an iterator of (x, y) points.
    pub fn enclosing(points: impl Iterator<Item = (f32, f32)>) -> Option<Self> {
        points.fold(None, |acc, (x, y)| {
            Some(match acc {
                None => AABB {
                    min_x: x,
                    min_y: y,
                    max_x: x,
                    max_y: y,
                },
                Some(b) => AABB {
                    min_x: b.min_x.min(x),
                    min_y: b.min_y.min(y),
                    max_x: b.max_x.max(x),
                    max_y: b.max_y.max(y),
                },
            })
        })
    }

    /// Expand bounds by a proportional factor plus a fixed minimum.
    pub fn padded(&self, factor: f32) -> Self {
        let pad_x = self.width() * factor + 1.0;
        let pad_y = self.height() * factor + 1.0;
        AABB {
            min_x: self.min_x - pad_x,
            min_y: self.min_y - pad_y,
            max_x: self.max_x + pad_x,
            max_y: self.max_y + pad_y,
        }
    }

    pub fn subdivide(&self) -> [AABB; 4] {
        let (cx, cy) = self.center();
        [
            AABB {
                min_x: self.min_x,
                min_y: self.min_y,
                max_x: cx,
                max_y: cy,
            },
            AABB {
                min_x: cx,
                min_y: self.min_y,
                max_x: self.max_x,
                max_y: cy,
            },
            AABB {
                min_x: self.min_x,
                min_y: cy,
                max_x: cx,
                max_y: self.max_y,
            },
            AABB {
                min_x: cx,
                min_y: cy,
                max_x: self.max_x,
                max_y: self.max_y,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn world_bounds() -> AABB {
        AABB {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 100.0,
            max_y: 100.0,
        }
    }

    #[test]
    fn contains() {
        let b = world_bounds();
        assert!(b.contains(50.0, 50.0));
        assert!(b.contains(0.0, 0.0));
        assert!(b.contains(100.0, 100.0));
        assert!(!b.contains(-1.0, 50.0));
        assert!(!b.contains(50.0, 101.0));
    }

    #[test]
    fn intersects_circle() {
        let b = world_bounds();
        assert!(b.intersects_circle(50.0, 50.0, 10.0));
        assert!(b.intersects_circle(105.0, 50.0, 10.0));
        assert!(!b.intersects_circle(115.0, 50.0, 10.0));
    }

    #[test]
    fn enclosing_empty_iterator_is_none() {
        assert!(AABB::enclosing(std::iter::empty()).is_none());
    }

    #[test]
    fn enclosing_single_point_is_degenerate() {
        let b = AABB::enclosing(std::iter::once((3.0, 4.0))).unwrap();
        assert_eq!(b.min_x, 3.0);
        assert_eq!(b.max_x, 3.0);
        assert_eq!(b.min_y, 4.0);
        assert_eq!(b.max_y, 4.0);
        assert_eq!(b.width(), 0.0);
        assert_eq!(b.height(), 0.0);
    }

    #[test]
    fn enclosing_multiple_points() {
        let pts = [(1.0, 5.0), (-2.0, 3.0), (4.0, -1.0), (0.0, 2.0)];
        let b = AABB::enclosing(pts.into_iter()).unwrap();
        assert_eq!(b.min_x, -2.0);
        assert_eq!(b.max_x, 4.0);
        assert_eq!(b.min_y, -1.0);
        assert_eq!(b.max_y, 5.0);
    }

    #[test]
    fn padded_zero_factor_still_adds_minimum() {
        let b = world_bounds().padded(0.0);
        // `padded` always adds +1.0 on each side even at factor 0
        assert_eq!(b.min_x, -1.0);
        assert_eq!(b.max_x, 101.0);
        assert_eq!(b.min_y, -1.0);
        assert_eq!(b.max_y, 101.0);
    }

    #[test]
    fn padded_positive_factor_expands() {
        let b = world_bounds().padded(0.1); // 10% + 1
        assert!(b.width() > world_bounds().width());
        assert!(b.height() > world_bounds().height());
        assert_eq!(b.center(), world_bounds().center()); // center preserved
    }

    #[test]
    fn subdivide_produces_four_quadrants() {
        let b = world_bounds();
        let quads = b.subdivide();
        assert_eq!(quads.len(), 4);
        // Each quadrant is half the width/height
        for q in &quads {
            assert!((q.width() - 50.0).abs() < f32::EPSILON);
            assert!((q.height() - 50.0).abs() < f32::EPSILON);
        }
    }
}
