use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{camera::Viewport, view::RenderLayers},
};
use leafwing_input_manager::orientation::Direction;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};

use super::config::DungeonConfig;
use crate::{
    loading::{TextureAssets, UIConfig},
    ui::ScaledUiConfig,
    GameState,
};

pub struct Camera3DPlugin;

impl Plugin for Camera3DPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update,
                (handle_input, update_viewport).run_if(in_state(GameState::Playing)),
            );
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

pub fn setup(mut commands: Commands, textures: Res<TextureAssets>, config: Res<Assets<UIConfig>>) {
    let config = config.get(textures.hud_config.id()).unwrap();

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
                        viewport: Some(Viewport {
                            physical_size: UVec2::new(
                                config.dungeon_view_size.0 as u32,
                                config.dungeon_view_size.1 as u32,
                            ),
                            physical_position: UVec2::new(
                                config.dongeon_view_margin.0 as u32,
                                config.dongeon_view_margin.1 as u32,
                            ),
                            ..default()
                        }),
                        ..default()
                    },
                    camera_3d: Camera3d {
                        clear_color: ClearColorConfig::None,
                        ..default()
                    },
                    ..default()
                },
                DUNGEON_CAMERA_LAYER,
                DungeonCamera,
            ));
        });
}

pub fn update_viewport(
    mut camera3d: Query<&mut Camera, With<DungeonCamera>>,
    config: Res<ScaledUiConfig>,
    window: Query<&Window>,
) {
    // if !config.is_changed() {
    //     return;
    // }
    if let (Ok(mut camera), Ok(window)) = (camera3d.get_single_mut(), window.get_single()) {
        match camera.viewport.as_mut() {
            Some(viewport) => {
                viewport.physical_size = UVec2::new(
                    config.dungeon_view_size.0 as u32,
                    config.dungeon_view_size.1 as u32,
                );
                viewport.physical_position = UVec2::new(
                    (config.padding + config.dongeon_view_margin.0) as u32,
                    config.dongeon_view_margin.1 as u32,
                );
            }
            None => (),
        }
    }
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
