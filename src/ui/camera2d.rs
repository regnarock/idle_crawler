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

pub const UI_LAYER: RenderLayers = RenderLayers::layer(1);

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                // no "background color", we need to see the main camera's output
                clear_color: ClearColorConfig::Custom(Color::GRAY),
                ..default()
            },
            camera: Camera {
                // renders on top of the other cameras
                order: 10,
                ..default()
            },
            ..default()
        },
        Name::new("hud_camera"),
        UI_LAYER,
        UiCamera,
    ));
}
