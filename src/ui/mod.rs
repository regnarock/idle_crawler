mod camera2d;

use bevy::{prelude::*, render::view::RenderLayers};

use crate::{loading::TextureAssets, GameState};
use camera2d::Camera2DPlugin;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Camera2DPlugin)
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