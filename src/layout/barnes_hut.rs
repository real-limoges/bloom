use crate::graph::Node;
use crate::spatial::AABB;
use glam::Vec2;

const EPSILON: f32 = 0.01;
const MIN_DIST: f32 = 1.0;
const MAX_DEPTH: u32 = 50;

enum QuadNode {
    Empty,
    Leaf {
        pos: Vec2,
        mass: f32,
    },
    Internal {
        center_of_mass: Vec2,
        total_mass: f32,
        children: Box<[QuadNode; 4]>,
    },
}

impl QuadNode {
    fn insert(&mut self, pos: Vec2, mass: f32, bounds: &AABB, depth: u32) {
        match self {
            QuadNode::Empty => {
                *self = QuadNode::Leaf { pos, mass };
            }
            QuadNode::Leaf {
                pos: existing_pos,
                mass: existing_mass,
            } => {
                // At max depth, merge into existing leaf
                if depth >= MAX_DEPTH {
                    let total = *existing_mass + mass;
                    *existing_pos = (*existing_pos * *existing_mass + pos * mass) / total;
                    *existing_mass = total;
                    return;
                }

                let ep = *existing_pos;
                let em = *existing_mass;

                let total_mass = em + mass;
                let center_of_mass = (ep * em + pos * mass) / total_mass;
                let sub_bounds = bounds.subdivide();

                let mut children = Box::new([
                    QuadNode::Empty,
                    QuadNode::Empty,
                    QuadNode::Empty,
                    QuadNode::Empty,
                ]);

                // Re-insert existing leaf
                let eq = quadrant(ep, bounds);
                children[eq].insert(ep, em, &sub_bounds[eq], depth + 1);

                // Insert new node
                let nq = quadrant(pos, bounds);
                children[nq].insert(pos, mass, &sub_bounds[nq], depth + 1);

                *self = QuadNode::Internal {
                    center_of_mass,
                    total_mass,
                    children,
                };
            }
            QuadNode::Internal {
                center_of_mass,
                total_mass,
                children,
            } => {
                *center_of_mass = (*center_of_mass * *total_mass + pos * mass) / (*total_mass + mass);
                *total_mass += mass;

                let sub_bounds = bounds.subdivide();
                let q = quadrant(pos, bounds);
                children[q].insert(pos, mass, &sub_bounds[q], depth + 1);
            }
        }
    }

    fn compute_force(&self, pos: Vec2, theta: f32, repulsion: f32, bounds: &AABB) -> Vec2 {
        match self {
            QuadNode::Empty => Vec2::ZERO,
            QuadNode::Leaf {
                pos: leaf_pos,
                mass,
            } => {
                let delta = pos - *leaf_pos;
                let dist = delta.length();
                if dist < EPSILON {
                    return Vec2::ZERO; // self-interaction
                }
                let dist = dist.max(MIN_DIST);
                delta.normalize() * (repulsion * *mass / (dist * dist))
            }
            QuadNode::Internal {
                center_of_mass,
                total_mass,
                children,
            } => {
                let delta = pos - *center_of_mass;
                let dist = delta.length().max(EPSILON);
                let s = bounds.width().max(bounds.height());

                if s / dist < theta {
                    // Treat as single body
                    let dist = dist.max(MIN_DIST);
                    delta.normalize() * (repulsion * *total_mass / (dist * dist))
                } else {
                    // Recurse into children
                    let sub_bounds = bounds.subdivide();
                    let mut force = Vec2::ZERO;
                    for (i, child) in children.iter().enumerate() {
                        force += child.compute_force(pos, theta, repulsion, &sub_bounds[i]);
                    }
                    force
                }
            }
        }
    }
}

fn quadrant(pos: Vec2, bounds: &AABB) -> usize {
    let (cx, cy) = bounds.center();
    match (pos.x >= cx, pos.y >= cy) {
        (false, false) => 0, // bottom-left
        (true, false) => 1,  // bottom-right
        (false, true) => 2,  // top-left
        (true, true) => 3,   // top-right
    }
}

pub struct BarnesHutTree {
    root: QuadNode,
    bounds: AABB,
}

impl BarnesHutTree {
    pub fn build(nodes: &[Node]) -> Self {
        if nodes.is_empty() {
            return Self {
                root: QuadNode::Empty,
                bounds: AABB {
                    min_x: -1.0,
                    min_y: -1.0,
                    max_x: 1.0,
                    max_y: 1.0,
                },
            };
        }

        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for n in nodes {
            min_x = min_x.min(n.x);
            min_y = min_y.min(n.y);
            max_x = max_x.max(n.x);
            max_y = max_y.max(n.y);
        }

        // 5% padding + minimum size
        let w = (max_x - min_x).max(1.0);
        let h = (max_y - min_y).max(1.0);
        let pad_x = w * 0.05;
        let pad_y = h * 0.05;
        let bounds = AABB {
            min_x: min_x - pad_x,
            min_y: min_y - pad_y,
            max_x: max_x + pad_x,
            max_y: max_y + pad_y,
        };

        let mut root = QuadNode::Empty;
        for n in nodes {
            root.insert(Vec2::new(n.x, n.y), 1.0, &bounds, 0);
        }

        Self { root, bounds }
    }

    pub fn compute_repulsion(
        &self,
        node_idx: usize,
        nodes: &[Node],
        repulsion: f32,
        theta: f32,
    ) -> Vec2 {
        let pos = Vec2::new(nodes[node_idx].x, nodes[node_idx].y);
        self.root.compute_force(pos, theta, repulsion, &self.bounds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn insert_single_node() {
        let nodes = vec![make_node(1, 5.0, 5.0)];
        let tree = BarnesHutTree::build(&nodes);
        assert!(matches!(tree.root, QuadNode::Leaf { .. }));
    }

    #[test]
    fn insert_multiple_nodes() {
        let nodes = vec![
            make_node(1, 1.0, 1.0),
            make_node(2, 9.0, 9.0),
            make_node(3, 1.0, 9.0),
            make_node(4, 9.0, 1.0),
            make_node(5, 5.0, 5.0),
        ];
        let tree = BarnesHutTree::build(&nodes);
        assert!(matches!(tree.root, QuadNode::Internal { .. }));
    }

    #[test]
    fn repulsion_pushes_apart() {
        let nodes = vec![make_node(1, 0.0, 0.0), make_node(2, 2.0, 0.0)];
        let tree = BarnesHutTree::build(&nodes);

        let f0 = tree.compute_repulsion(0, &nodes, 100.0, 0.7);
        let f1 = tree.compute_repulsion(1, &nodes, 100.0, 0.7);

        // Node 0 should be pushed left (negative x)
        assert!(f0.x < 0.0, "f0.x = {} should be negative", f0.x);
        // Node 1 should be pushed right (positive x)
        assert!(f1.x > 0.0, "f1.x = {} should be positive", f1.x);
    }

    #[test]
    fn closer_nodes_repel_more() {
        let close = vec![make_node(1, 0.0, 0.0), make_node(2, 1.0, 0.0)];
        let far = vec![make_node(1, 0.0, 0.0), make_node(2, 10.0, 0.0)];

        let tree_close = BarnesHutTree::build(&close);
        let tree_far = BarnesHutTree::build(&far);

        let f_close = tree_close.compute_repulsion(0, &close, 100.0, 0.7);
        let f_far = tree_far.compute_repulsion(0, &far, 100.0, 0.7);

        assert!(
            f_close.length() > f_far.length(),
            "close force {} should exceed far force {}",
            f_close.length(),
            f_far.length()
        );
    }

    #[test]
    fn force_approximation_accuracy() {
        let nodes: Vec<Node> = (0..10)
            .map(|i| {
                let angle = i as f32 * std::f32::consts::TAU / 10.0;
                make_node(i, angle.cos() * 10.0, angle.sin() * 10.0)
            })
            .collect();

        let tree = BarnesHutTree::build(&nodes);
        let repulsion = 100.0;
        let theta = 0.7;

        for i in 0..nodes.len() {
            let bh_force = tree.compute_repulsion(i, &nodes, repulsion, theta);

            // Compute naive O(n²) force
            let pi = Vec2::new(nodes[i].x, nodes[i].y);
            let mut naive_force = Vec2::ZERO;
            for j in 0..nodes.len() {
                if i == j {
                    continue;
                }
                let pj = Vec2::new(nodes[j].x, nodes[j].y);
                let delta = pi - pj;
                let dist = delta.length().max(MIN_DIST);
                naive_force += delta.normalize() * (repulsion / (dist * dist));
            }

            let error = (bh_force - naive_force).length();
            let magnitude = naive_force.length();
            if magnitude > 0.01 {
                let relative_error = error / magnitude;
                assert!(
                    relative_error < 0.15,
                    "Node {}: relative error {:.3} exceeds 15% (bh={:?}, naive={:?})",
                    i,
                    relative_error,
                    bh_force,
                    naive_force
                );
            }
        }
    }
}
