use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{camera::Viewport, view::RenderLayers},
    window::WindowResized,
};
use bevy_ecs_ldtk::{LdtkPlugin, LdtkWorldBundle, LevelSelection};

use crate::{ui::ScaledUiConfig, GameState};

pub struct DungeonLabyrinthPlugin;

impl Plugin for DungeonLabyrinthPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::index(0))
            .add_systems(
                Update,
                (
                    // Setup
                    setup_minimap.run_if(resource_added::<ScaledUiConfig>()),
                    // Update
                    center_camera_on_window_resize.run_if(on_event::<WindowResized>()),
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (update_viewport.run_if(
                    resource_exists_and_changed::<ScaledUiConfig>()
                        .and_then(not(resource_added::<ScaledUiConfig>())),
                ))
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
    config: Res<ScaledUiConfig>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let mut camera = Camera2dBundle::default();
        camera.projection.scale = 2.;
        camera.transform.translation.x += window.resolution.height() / 4.0;
        camera.transform.translation.y += window.resolution.width() / 4.0;
        camera.camera.order = 2;
        camera.camera_2d.clear_color = ClearColorConfig::None;
        camera.camera.viewport = Some(Viewport {
            physical_size: UVec2::new(config.minimap_size.0 as u32, config.minimap_size.1 as u32),
            physical_position: UVec2::new(
                (config.padding
                    + config.dungeon_view_size.0
                    + config.dongeon_view_margin.0
                    + config.minimap_margin.0) as u32,
                config.minimap_margin.1 as u32,
            ),
            ..default()
        });
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

pub fn center_camera_on_window_resize(
    mut camera: Query<&mut Transform, With<LabyrinthCamera2D>>,
    window: Query<&Window>,
) {
    if let (Ok(window), Ok(mut camera)) = (window.get_single(), camera.get_single_mut()) {
        camera.translation.x = window.resolution.height() / 4.0;
        camera.translation.y = window.resolution.width() / 4.0;
    }
}

pub fn update_viewport(
    mut camera: Query<&mut Camera, With<LabyrinthCamera2D>>,
    config: Res<ScaledUiConfig>,
) {
    if let Ok(mut camera) = camera.get_single_mut() {
        match camera.viewport.as_mut() {
            Some(viewport) => {
                viewport.physical_size =
                    UVec2::new(config.minimap_size.0 as u32, config.minimap_size.1 as u32);
                viewport.physical_position = UVec2::new(
                    (config.padding
                        + config.dungeon_view_size.0
                        + config.dongeon_view_margin.0
                        + config.minimap_margin.0) as u32,
                    config.minimap_margin.1 as u32,
                );
            }
            None => (),
        }
    }
}
