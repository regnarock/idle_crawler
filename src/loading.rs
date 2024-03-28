use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_kira_audio::AudioSource;

use crate::GameState;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<UIConfig>::new(&["ron"]))
            .add_loading_state(
                LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu),
            )
            .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
            .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/IMG_3359.png")]
    pub wall: Handle<Image>,
    #[asset(path = "textures/IMG_3359.png")]
    pub wall_normal: Handle<Image>,
    #[asset(path = "textures/floor.png")]
    pub floor: Handle<Image>,
    #[asset(path = "textures/Brick_1.png")]
    pub ceilling: Handle<Image>,
    #[asset(path = "textures/HUD.png")]
    pub hud: Handle<Image>,
    #[asset(path = "textures/HUD_config.ron")]
    pub hud_config: Handle<UIConfig>,
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct UIConfig {
    pub size: (f32, f32),
    pub dongeon_view_margin: (f32, f32),
    pub dungeon_view_size: (f32, f32),
    pub minimap_margin: (f32, f32),
    pub minimap_size: (f32, f32),
}
