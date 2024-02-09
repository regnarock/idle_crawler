use bevy::prelude::*;

pub trait MoveBy {
    fn move_by(self, d: MoveDirection, s: f32) -> Self;
}

#[derive(Debug, Clone, Copy)]
pub enum MoveDirection {
    Forward,
    Backward,
    Up,
    Down,
    Left,
    Right,
    ShiftLeft,
    ShiftRight,
}

impl MoveBy for Vec3 {
    fn move_by(self, direction: MoveDirection, step: f32) -> Self {
        let half_s = step * 0.5;
        match direction {
            MoveDirection::Forward => self + Vec3::new(0.0, 0.0, -step),
            MoveDirection::Backward => self + Vec3::new(0.0, 0.0, step),
            MoveDirection::Up => self + Vec3::new(0.0, half_s, half_s),
            MoveDirection::Down => self + Vec3::new(0.0, -half_s, half_s),
            MoveDirection::Left => self + Vec3::new(-half_s, 0.0, half_s),
            MoveDirection::Right => self + Vec3::new(half_s, 0.0, half_s),
            MoveDirection::ShiftLeft => self + Vec3::new(-step, 0.0, 0.0),
            MoveDirection::ShiftRight => self + Vec3::new(step, 0.0, 0.0),
        }
    }
}
