use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;

use crate::GameState;

use super::{
    config::DungeonConfig,
    vec_utils::{MoveBy, MoveDirection},
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(Update, draw_debug.run_if(in_state(GameState::Playing)));
    }
}

pub fn setup(mut gizmo_config: ResMut<GizmoConfig>) {
    gizmo_config.depth_bias = -1.0;
}

/// A rectangle triangle gizmo, with the right angle at the bottom left
pub trait TriangleGizmos {
    fn triangle_rect(&mut self, position: Vec3, rotation: Quat, a: f32, b: f32, color: Color);
}

impl<'s> TriangleGizmos for Gizmos<'s> {
    fn triangle_rect(&mut self, position: Vec3, rotation: Quat, a: f32, b: f32, color: Color) {
        let [tl, tr, bl] = rect_inner(a, b).map(|vec2| position + rotation * vec2.extend(0.));
        self.linestrip([tl, tr, bl, tl], color);
        fn rect_inner(a: f32, b: f32) -> [Vec2; 3] {
            let half_a = a / 2.;
            let half_b = b / 2.;
            let tl = Vec2::new(-half_a, half_b);
            let tr = Vec2::new(half_a, half_b);
            let bl = Vec2::new(half_a, -half_b);
            [tl, tr, bl]
        }
    }
}

pub fn draw_debug(mut gizmos: Gizmos, config: Res<DungeonConfig>) {
    if config.debug == 0 {
        return;
    }
    let size = config.size;
    let distance_field = config.distance_field;
    let left = config.left == 1;
    let right = config.right == 1;
    let center = config.center == 1;
    let center_center = config.center_center == 1;

    display_center(
        size,
        distance_field,
        &mut gizmos,
        left,
        right,
        center,
        center_center,
        0,
    );
}

fn display_center(
    size: f32,
    distance_field: f32,
    gizmos: &mut Gizmos<'_>,
    has_left: bool,
    has_right: bool,
    has_center: bool,
    has_center_center: bool,
    depth: usize,
) {
    let z = if depth > 0 { distance_field * -1. } else { 0. };
    let mut center = Vec3::new(0.0, 0.0, z);

    let square = Vec2::splat(size);
    let half = |s: Vec2| s * Vec2::new(0.5, 1.0);
    let flattened_square = square * Vec2::new(1.0, distance_field / 2.);

    // Layer 1
    // middle of the screen
    if has_center {
        gizmos.rect(center, Quat::IDENTITY, square, Color::BLUE);
    } else if depth < 2 {
        let depth = depth + 1;
        display_center(
            size,
            distance_field * (2.0 * depth as f32),
            gizmos,
            true,
            true,
            has_center_center,
            false,
            depth,
        );
    }

    // ceiling
    gizmos.rect(
        center.move_by(MoveDirection::Up, flattened_square),
        Quat::from_rotation_x(FRAC_PI_2),
        flattened_square,
        Color::RED,
    );

    // floor
    gizmos.rect(
        center.move_by(MoveDirection::Down, flattened_square),
        Quat::from_rotation_x(FRAC_PI_2),
        flattened_square,
        Color::RED,
    );
    // might be hidden left/right
    {
        if !has_left {
            // middle left
            gizmos.rect(
                center.move_by(MoveDirection::HalfShiftLeft, square),
                Quat::IDENTITY,
                half(square),
                Color::BLUE,
            );

            // floor left
            gizmos.triangle_rect(
                center
                    .move_by(MoveDirection::Down, flattened_square)
                    .move_by(MoveDirection::HalfShiftLeft, flattened_square),
                Quat::from_rotation_x(-FRAC_PI_2),
                half(flattened_square).x,
                flattened_square.y,
                Color::RED,
            );

            // ceiling left
            gizmos.triangle_rect(
                center
                    .move_by(MoveDirection::Up, flattened_square)
                    .move_by(MoveDirection::HalfShiftLeft, flattened_square),
                Quat::from_rotation_x(-FRAC_PI_2),
                half(flattened_square).x,
                flattened_square.y,
                Color::RED,
            );
        } else {
            // left wall
            gizmos.rect(
                center.move_by(MoveDirection::Left, flattened_square),
                Quat::from_rotation_y(FRAC_PI_2).mul_quat(Quat::from_rotation_z(FRAC_PI_2)),
                flattened_square.xy(),
                Color::GREEN,
            );
        }
        if !has_right {
            // middle right
            gizmos.rect(
                center.move_by(MoveDirection::HalfShiftRight, square),
                Quat::IDENTITY,
                half(square),
                Color::BLUE,
            );
            // floor right
            gizmos.triangle_rect(
                center
                    .move_by(MoveDirection::Down, flattened_square)
                    .move_by(MoveDirection::HalfShiftRight, flattened_square),
                Quat::from_rotation_x(FRAC_PI_2).mul_quat(Quat::from_rotation_z(PI)),
                half(flattened_square).x,
                flattened_square.y,
                Color::RED,
            );

            // ceiling right
            gizmos.triangle_rect(
                center
                    .move_by(MoveDirection::Up, flattened_square)
                    .move_by(MoveDirection::HalfShiftRight, flattened_square),
                Quat::from_rotation_x(FRAC_PI_2).mul_quat(Quat::from_rotation_z(PI)),
                half(flattened_square).x,
                flattened_square.y,
                Color::RED,
            );
        } else {
            // right wall
            gizmos.rect(
                center.move_by(MoveDirection::Right, flattened_square),
                Quat::from_rotation_y(FRAC_PI_2).mul_quat(Quat::from_rotation_z(FRAC_PI_2)),
                flattened_square.xy(),
                Color::GREEN,
            );
        }
    }
}
