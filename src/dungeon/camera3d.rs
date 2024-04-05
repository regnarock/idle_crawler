use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{camera::RenderTarget, view::RenderLayers},
};
use leafwing_input_manager::orientation::Direction;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};

use super::config::DungeonConfig;
use crate::{ui::HUDRenderViews, GameState};

pub struct Camera3DPlugin;

impl Plugin for Camera3DPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup.run_if(resource_added::<HUDRenderViews>()))
            .add_systems(Update, (handle_input,).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct DungeonCamera;

pub const DUNGEON_CAMERA_LAYER: RenderLayers = RenderLayers::layer(2);

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

pub fn setup(mut commands: Commands, config: Res<HUDRenderViews>) {
    commands
        .spawn((
            TransformBundle {
                local: Transform::from_xyz(0., 0., -1.),
                ..default()
            },
            VisibilityBundle::default(),
            CameraDirection::default(),
            Player,
            DUNGEON_CAMERA_LAYER,
        ))
        .with_children(|builder| {
            builder.spawn((
                UiCameraConfig { show_ui: false },
                Camera3dBundle {
                    transform: Transform::from_xyz(0., 0., 3.).looking_at(Vec3::ZERO, Vec3::Y),
                    projection: Projection::Perspective(PerspectiveProjection {
                        fov: FRAC_PI_4,
                        ..default()
                    }),
                    camera: Camera {
                        target: RenderTarget::Image(config.dungeon_handle.clone()),
                        ..default()
                    },
                    camera_3d: Camera3d {
                        clear_color: ClearColorConfig::Custom(Color::BLACK),
                        ..default()
                    },
                    ..default()
                },
                DUNGEON_CAMERA_LAYER,
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
