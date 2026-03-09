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
