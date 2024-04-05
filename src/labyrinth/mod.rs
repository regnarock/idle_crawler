use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{camera::RenderTarget, view::RenderLayers},
};
use bevy_ecs_ldtk::{LdtkPlugin, LdtkWorldBundle, LevelSelection};

use crate::{ui::HUDRenderViews, GameState};

pub struct DungeonLabyrinthPlugin;

impl Plugin for DungeonLabyrinthPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::index(0))
            .add_systems(
                Update,
                (
                    // Setup
                    setup_minimap.run_if(resource_added::<HUDRenderViews>()),
                    // Update
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct LabyrinthCamera2D;

// FIXME: ldtk scenes are not rendering when value > 0
pub const LABYRINTH_LAYER: RenderLayers = RenderLayers::layer(0);

#[derive(Component)]
pub struct Minimap;

pub fn setup_minimap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<HUDRenderViews>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let mut camera = Camera2dBundle::default();
        camera.projection.scale = 2.;
        camera.transform.translation.x += window.resolution.height() / 4.0;
        camera.transform.translation.y += window.resolution.width() / 4.0;
        camera.camera.order = 2;
        camera.camera.target = RenderTarget::Image(config.minimap_handle.clone());
        camera.camera_2d.clear_color = ClearColorConfig::Custom(Color::BLACK);
        commands.spawn((camera, LabyrinthCamera2D, UiCameraConfig { show_ui: false }));

        commands.spawn((
            LdtkWorldBundle {
                ldtk_handle: asset_server.load("insectivore.ldtk"),
                ..Default::default()
            },
            Minimap,
            //LABYRINTH_LAYER,
        ));
    }
}
