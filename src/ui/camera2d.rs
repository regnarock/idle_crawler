use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, render::view::RenderLayers};

use crate::GameState;

pub struct Camera2DPlugin;

impl Plugin for Camera2DPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup);
    }
}

#[derive(Component)]
pub struct UiCamera;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                // no "background color", we need to see the main camera's output
                clear_color: ClearColorConfig::None,
                ..default()
            },
            camera: Camera {
                // renders after / on top of the main camera
                order: 1,
                ..default()
            },
            ..default()
        },
        RenderLayers::layer(1),
        UiCamera,
    ));
}
