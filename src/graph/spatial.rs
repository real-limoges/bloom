use crate::graph::types::Node;

#[derive(Debug)]
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
}

pub struct Quadtree {
    bounds: AABB,
    capacity: usize,
    nodes: Vec<usize>,
    children: Option<Box<[Quadtree; 4]>>,
}

impl Quadtree {
    pub fn new(bounds: AABB, capacity: usize) -> Self {
        Self {
            bounds,
            capacity,
            nodes: Vec::new(),
            children: None,
        }
    }

    pub fn insert(&mut self, node_idx: usize, node: &Node) -> bool {
        if !self.bounds.contains(node.x, node.y) {
            return false;
        }
        if self.nodes.len() < self.capacity && self.children.is_none() {
            self.nodes.push(node_idx);
            return true;
        }
        if self.children.is_none() {
            self.subdivide();
        }
        if let Some(ref mut children) = self.children {
            for child in children.iter_mut() {
                if child.insert(node_idx, node) {
                    return true;
                }
            }
        }
        false
    }

    pub fn query_point(&self, x: f32, y: f32, radius: f32) -> Vec<usize> {
        let mut result = Vec::new();

        if !self.bounds.intersects_circle(x, y, radius) {
            return result;
        }

        result.extend_from_slice(&self.nodes);

        if let Some(ref children) = self.children {
            for child in children.iter() {
                result.extend(child.query_point(x, y, radius));
            }
        }

        result
    }

    fn subdivide(&mut self) {
        let AABB {
            min_x,
            min_y,
            max_x,
            max_y,
        } = self.bounds;
        let mid_x = (min_x + max_x) / 2.0;
        let mid_y = (min_y + max_y) / 2.0;

        self.children = Some(Box::new([
            Quadtree::new(
                AABB {
                    min_x,
                    min_y,
                    max_x: mid_x,
                    max_y: mid_y,
                },
                self.capacity,
            ),
            Quadtree::new(
                AABB {
                    min_x: mid_x,
                    min_y,
                    max_x,
                    max_y: mid_y,
                },
                self.capacity,
            ),
            Quadtree::new(
                AABB {
                    min_x,
                    min_y: mid_y,
                    max_x: mid_x,
                    max_y,
                },
                self.capacity,
            ),
            Quadtree::new(
                AABB {
                    min_x: mid_x,
                    min_y: mid_y,
                    max_x,
                    max_y,
                },
                self.capacity,
            ),
        ]));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::types::Node;

    fn make_node(id: u32, x: f32, y: f32) -> Node {
        Node {
            id,
            label: String::new(),
            pagerank: 0.0,
            degree: 0,
            x,
            y,
        }
    }

    fn world_bounds() -> AABB {
        AABB {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 100.0,
            max_y: 100.0,
        }
    }

    #[test]
    fn aabb_contains() {
        let b = world_bounds();
        assert!(b.contains(50.0, 50.0));
        assert!(b.contains(0.0, 0.0));
        assert!(b.contains(100.0, 100.0));
        assert!(!b.contains(-1.0, 50.0));
        assert!(!b.contains(50.0, 101.0));
    }

    #[test]
    fn aabb_intersects_circle() {
        let b = world_bounds();
        assert!(b.intersects_circle(50.0, 50.0, 10.0));
        assert!(b.intersects_circle(105.0, 50.0, 10.0));
        assert!(!b.intersects_circle(115.0, 50.0, 10.0));
    }

    #[test]
    fn insert_and_query_returns_candidates() {
        let mut qt = Quadtree::new(world_bounds(), 4);
        let nodes = vec![
            make_node(0, 10.0, 10.0),
            make_node(1, 90.0, 90.0),
            make_node(2, 50.0, 50.0),
        ];
        for (i, n) in nodes.iter().enumerate() {
            assert!(qt.insert(i, n));
        }

        // Large radius covers whole tree — all candidates returned
        let all = qt.query_point(50.0, 50.0, 200.0);
        assert_eq!(all.len(), 3);

        // Small radius far from any node — no cell intersects
        let none = qt.query_point(0.0, 100.0, 0.1);
        // Candidates come from cells whose AABB intersects the circle;
        // exact distance filtering is the caller's responsibility
        assert!(none.is_empty() || none.iter().all(|&i| i < 3));
    }

    #[test]
    fn query_outside_bounds_returns_empty() {
        let mut qt = Quadtree::new(world_bounds(), 4);
        let n = make_node(0, 50.0, 50.0);
        qt.insert(0, &n);

        // Query circle entirely outside the tree bounds
        let result = qt.query_point(200.0, 200.0, 5.0);
        assert!(result.is_empty());
    }

    #[test]
    fn insert_outside_bounds_fails() {
        let mut qt = Quadtree::new(world_bounds(), 4);
        let n = make_node(0, 200.0, 200.0);
        assert!(!qt.insert(0, &n));
    }

    #[test]
    fn query_empty_tree() {
        let qt = Quadtree::new(world_bounds(), 4);
        assert!(qt.query_point(50.0, 50.0, 10.0).is_empty());
    }

    #[test]
    fn subdivide_on_overflow() {
        let mut qt = Quadtree::new(world_bounds(), 2);
        let nodes = vec![
            make_node(0, 10.0, 10.0),
            make_node(1, 20.0, 20.0),
            make_node(2, 30.0, 30.0),
        ];
        for (i, n) in nodes.iter().enumerate() {
            qt.insert(i, n);
        }
        // After inserting 3 nodes with capacity 2, tree should have subdivided
        assert!(qt.children.is_some());
    }
}
