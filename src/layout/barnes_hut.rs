use crate::spatial::AABB;
use glam::Vec2;

#[allow(dead_code)]
enum QuadNode {
    Empty,
    Leaf {
        pos: Vec2,
        mass: f32,
    },
    Internal {
        center_of_mass: Vec2,
        total_mass: f32,
        bounds: AABB,
        children: Box<[QuadNode; 4]>,
    },
}

impl QuadNode {
    #[allow(dead_code)]
    fn insert(&mut self, pos: Vec2, mass: f32, _bounds: &AABB) {
        match self {
            QuadNode::Empty => {
                *self = QuadNode::Leaf { pos, mass };
            }
            QuadNode::Leaf {
                pos: _existing_pos,
                mass: _existing_mass,
            } => {}
            QuadNode::Internal {
                center_of_mass: _,
                total_mass: _,
                bounds: _node_bounds,
                children: _,
            } => {}
        }
    }
}
