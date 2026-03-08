use glam::Vec2;
use crate::spatial::AABB;

enum QuadNode {
    Empty,
    Leaf { pos: Vec2, mass: f32 },
    Internal {
        center_of_mass: Vec2,
        total_mass: f32,
        bounds: AABB,
        children: Box<[QuadNode; 4]>,
    },
}

impl QuadNode {
    fn insert(&mut self, pos: Vec2, mass: f32, bounds: &AABB) {
        match self {
            QuadNode::Empty => {
                *self = QuadNode::Leaf { pos, mass };
            }
            QuadNode::Leaf { pos: existing_pos, mass: existing_mass } => {}
            QuadNode::Internal { center_of_mass, total_mass, bounds: node_bounds, children } => {}
        }
    }
}