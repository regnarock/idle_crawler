use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{camera::Viewport, view::RenderLayers},
};
use bevy_ecs_ldtk::{LdtkPlugin, LdtkWorldBundle, LevelSelection};

use crate::{
    loading::{TextureAssets, UIConfig},
    GameState,
};

pub struct DungeonLabyrinthPlugin;

impl Plugin for DungeonLabyrinthPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::index(0))
            .add_systems(OnEnter(GameState::Playing), setup_minimap)
            .add_systems(Update, update_viewport.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct LabyrinthCamera2D;

// FIXME: ldtk scenes are not rendering when value > 0
pub const LABYRINTH_LAYER: RenderLayers = RenderLayers::layer(0);

pub fn setup_minimap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    textures: Res<TextureAssets>,
    config: Res<Assets<UIConfig>>,
) {
    let config = config.get(textures.hud_config.id()).unwrap();

    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
                ..default()
            },
            camera: Camera {
                // renders on top of the other cameras
                order: 2,
                viewport: Some(Viewport {
                    physical_size: UVec2::new(
                        config.minimap_size.0 as u32,
                        config.minimap_size.1 as u32,
                    ),
                    physical_position: UVec2::new(
                        config.dungeon_view_size.0 as u32
                            + config.dongeon_view_margin.0 as u32
                            + config.minimap_margin.0 as u32,
                        config.dungeon_view_size.1 as u32
                            + config.dongeon_view_margin.1 as u32
                            + config.minimap_margin.1 as u32,
                    ),
                    ..default()
                }),
                ..default()
            },
            ..default()
        },
        Name::new("minimap"),
        LabyrinthCamera2D,
        UiCameraConfig { show_ui: false },
        //LABYRINTH_LAYER,
    ));

    commands.spawn((
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("insectivore.ldtk"),
            ..Default::default()
        },
        //LABYRINTH_LAYER,
    ));
}

pub fn update_viewport(
    mut camera: Query<&mut Camera, With<LabyrinthCamera2D>>,
    textures: Res<TextureAssets>,
    config: Res<Assets<UIConfig>>,
    window: Query<&Window>,
) {
    if !config.is_changed() {
        return;
    }
    if let (Ok(mut camera), Ok(window)) = (camera.get_single_mut(), window.get_single()) {
        let config: &UIConfig = config.get(textures.hud_config.id()).unwrap();

        match camera.viewport.as_mut() {
            Some(viewport) => {
                viewport.physical_size =
                    UVec2::new(config.minimap_size.0 as u32, config.minimap_size.1 as u32);
                viewport.physical_position = UVec2::new(
                    (window.width() / 2. - config.size.0 / 2.
                        + config.dungeon_view_size.0
                        + config.dongeon_view_margin.0
                        + config.minimap_margin.0) as u32,
                    config.dungeon_view_size.1 as u32
                        + config.dongeon_view_margin.1 as u32
                        + config.minimap_margin.1 as u32,
                );
                viewport.physical_size = UVec2::new(
                    config.dungeon_view_size.0 as u32,
                    config.dungeon_view_size.1 as u32,
                );
            }
            None => (),
        }
    }
}
