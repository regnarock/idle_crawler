use std::f32::consts::FRAC_PI_2;

use super::config::DungeonConfig;
use crate::{ui::config::UIConfig, GameState};
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::Viewport};
use leafwing_input_manager::orientation::Direction;
pub struct Camera3DPlugin;

impl Plugin for Camera3DPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(Update, handle_input);
    }
}

#[derive(Component)]
pub struct DungeonCamera;

#[derive(Component, Default)]
pub enum CameraDirection {
    #[default]
    North,
    East,
    South,
    West,
}

impl CameraDirection {
    // TODO: We should probably not use `leafwing_input_manager` here and just get rid of the dependency
    pub fn uvec(&self) -> Vec3 {
        match self {
            CameraDirection::North => Direction::NORTH.unit_vector().extend(0.).xzy(),
            CameraDirection::East => Direction::EAST.unit_vector().extend(0.).xzy(),
            CameraDirection::South => Direction::SOUTH.unit_vector().extend(0.).xzy(),
            CameraDirection::West => Direction::WEST.unit_vector().extend(0.).xzy(),
        }
    }
}

#[derive(Component)]
pub struct Player;

pub fn setup(mut commands: Commands, ui_config: Res<UIConfig>) {
    commands
        .spawn((
            TransformBundle {
                local: Transform::from_xyz(0., 0., -1.),
                ..default()
            },
            VisibilityBundle::default(),
            CameraDirection::default(),
            Player,
        ))
        .with_children(|builder| {
            builder.spawn((
                Camera3dBundle {
                    transform: Transform::from_xyz(0., 0., 3.).looking_at(Vec3::ZERO, Vec3::Y),
                    projection: Projection::Perspective(PerspectiveProjection {
                        fov: 0.7853981634,
                        ..default()
                    }),
                    camera: Camera {
                        viewport: Some(Viewport {
                            physical_size: UVec2::new(
                                ui_config.dungeon_window_size.x as u32,
                                ui_config.dungeon_window_size.y as u32,
                            ),
                            physical_position: UVec2::new(
                                ui_config.margins.x as u32,
                                ui_config.margins.y as u32,
                            ),
                            ..default()
                        }),
                        ..default()
                    },
                    camera_3d: Camera3d {
                        clear_color: ClearColorConfig::Custom(Color::BLACK),
                        ..default()
                    },
                    ..default()
                },
                DungeonCamera,
            ));
        });
}

pub fn handle_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera: Query<(&mut Transform, &mut CameraDirection), With<Player>>,
    config: Res<DungeonConfig>,
) {
    if let Some((mut transform, mut direction)) = camera.iter_mut().next() {
        if keyboard_input.just_pressed(KeyCode::W) {
            transform.translation = match *direction {
                CameraDirection::North | CameraDirection::South => {
                    transform.translation - direction.uvec() * config.size
                }
                CameraDirection::East | CameraDirection::West => {
                    transform.translation + direction.uvec() * config.size
                }
            }
        } else if keyboard_input.just_pressed(KeyCode::S) {
            transform.translation = match *direction {
                CameraDirection::North | CameraDirection::South => {
                    transform.translation + direction.uvec() * config.size
                }
                CameraDirection::East | CameraDirection::West => {
                    transform.translation - direction.uvec() * config.size
                }
            }
        } else if keyboard_input.just_pressed(KeyCode::A) {
            //transform.translation.x -= config.size;
            match *direction {
                CameraDirection::North => *direction = CameraDirection::West,
                CameraDirection::South => *direction = CameraDirection::East,
                CameraDirection::East => *direction = CameraDirection::North,
                CameraDirection::West => *direction = CameraDirection::South,
            }
            transform.rotate_y(FRAC_PI_2);
        } else if keyboard_input.just_pressed(KeyCode::D) {
            match *direction {
                CameraDirection::North => *direction = CameraDirection::East,
                CameraDirection::South => *direction = CameraDirection::West,
                CameraDirection::East => *direction = CameraDirection::South,
                CameraDirection::West => *direction = CameraDirection::North,
            }
            transform.rotate_y(-FRAC_PI_2);
        }
    }
}
