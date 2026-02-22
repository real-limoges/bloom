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
