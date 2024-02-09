mod camera2d;
pub mod config;

use bevy::{prelude::*, render::view::RenderLayers};

use crate::{loading::TextureAssets, GameState};
use camera2d::Camera2DPlugin;

use self::config::UIConfigPlugin;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((Camera2DPlugin, UIConfigPlugin))
            .add_systems(OnEnter(GameState::Playing), setup);
    }
}

pub fn setup(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: textures.hud.clone(),
            ..default()
        },
        RenderLayers::layer(1),
        Name::new("hud"),
    ));
}
