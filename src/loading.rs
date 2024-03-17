use bevy::{ecs::query::WorldQuery, prelude::*};
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
            .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading)
            .add_systems(OnExit(GameState::Loading), setup_game_scene)
            .add_systems(
                Update,
                generate_graphical_constants.run_if(resource_exists::<TextureAssets>()),
            );
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

#[derive(Resource)]
pub struct GraphicalConstants {
    pub ratio: f32,
    pub half_width: f32,
}

pub fn generate_graphical_constants(
    mut commands: Commands,
    window: Query<&Window>,
    textures: Res<TextureAssets>,
    config: Res<Assets<UIConfig>>,
) {
    if let Ok(window) = window.get_single() {
        let config = config.get(textures.hud_config.id()).unwrap();
        let ratio = config.size.1 as f32 / window.physical_height() as f32;
        let half_width = window.width() / 2.0;

        commands.insert_resource(GraphicalConstants { ratio, half_width });
    }
}

#[derive(Component)]
pub struct GameSceneAnchor;

pub fn setup_game_scene(mut commands: Commands, constants: Res<GraphicalConstants>) {
    commands.spawn((
        TransformBundle {
            local: Transform::from_scale(Vec3::new(constants.ratio, constants.ratio, 1.)),
            ..default()
        },
        GameSceneAnchor,
        Name::new("GameSceneAnchor"),
    ));
}

#[derive(WorldQuery)]
pub struct GameScene {
    pub anchor: Entity,
    _filter: With<GameSceneAnchor>,
}

pub fn spawn_in_game_scene(
    In(result): In<Result<Vec<Entity>, ()>>,
    mut commands: Commands,
    scene: Query<GameScene>,
) {
    info!("spawn_in_game_scene");
    match result {
        Ok(entities) => {
            if let Ok(scene) = scene.get_single() {
                info!("pushing entity as children");
                commands.entity(scene.anchor).push_children(&entities);
            }
        }
        Err(_) => error!("Failed to spawn entity in game scene"),
    }
}
