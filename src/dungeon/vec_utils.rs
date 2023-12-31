use bevy::prelude::*;

pub trait MoveBy {
    fn move_by(self, d: MoveDirection, s: Vec2) -> Self;
}

pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
    ShiftLeft,
    HalfShiftLeft,
    ShiftRight,
    HalfShiftRight,
}

impl MoveBy for Vec3 {
    fn move_by(self, d: MoveDirection, s: Vec2) -> Self {
        match d {
            MoveDirection::Up => self + Vec3::new(0.0, s.x / 2.0, s.y / 2.0),
            MoveDirection::Down => self + Vec3::new(0.0, s.x / -2.0, s.y / 2.0),
            MoveDirection::ShiftLeft => self + Vec3::new(s.x * -1., 0.0, 0.0),
            MoveDirection::HalfShiftLeft => self + Vec3::new(s.x * -0.75, 0.0, 0.0),
            MoveDirection::Left => self + Vec3::new(s.x * -0.5, 0.0, s.y * 0.5),
            MoveDirection::ShiftRight => self + Vec3::new(s.x, 0.0, 0.0),
            MoveDirection::HalfShiftRight => self + Vec3::new(s.x * 0.75, 0.0, 0.0),
            MoveDirection::Right => self + Vec3::new(s.x * 0.5, 0.0, s.y * 0.5),
        }
    }
}
