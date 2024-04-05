mod camera2d;

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
};

use crate::{
    loading::{TextureAssets, UIConfig},
    GameState,
};
use camera2d::Camera2DPlugin;

use self::camera2d::UI_LAYER;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Camera2DPlugin)
            .add_systems(OnEnter(GameState::Playing), setup);
    }
}

const ASPECT_RATIO_4_3: f32 = 1.3574498567;

#[derive(Component)]
pub struct HudImage;

#[derive(Component)]
pub struct MinimapImage;

#[derive(Component)]
pub struct DungeonViewImage;

pub fn setup(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    config: Res<Assets<UIConfig>>,
    mut images: ResMut<Assets<Image>>,
) {
    let config = config.get(textures.hud_config.id()).unwrap();
    // Create an empty image to hold the dungeon view
    let mut dungeon_render_target = Image::new_fill(
        Extent3d {
            width: config.dungeon_view_size.0 as u32,
            height: config.dungeon_view_size.1 as u32,
            ..default()
        },
        TextureDimension::D2,
        &[255; 4],
        TextureFormat::Rgba8UnormSrgb,
    );

    // Create an empty image to hold the minimap
    let mut minimap_render_target = Image::new_fill(
        Extent3d {
            width: config.minimap_size.0 as u32,
            height: config.minimap_size.1 as u32,
            ..default()
        },
        TextureDimension::D2,
        &[255; 4],
        TextureFormat::Rgba8UnormSrgb,
    );

    dungeon_render_target.texture_descriptor.usage =
        TextureUsages::RENDER_ATTACHMENT | dungeon_render_target.texture_descriptor.usage;
    let dungeon_handle = images.add(dungeon_render_target);

    minimap_render_target.texture_descriptor.usage =
        TextureUsages::RENDER_ATTACHMENT | minimap_render_target.texture_descriptor.usage;
    let minimap_handle = images.add(minimap_render_target);

    let ui_config = HUDRenderViews {
        dungeon_handle: dungeon_handle.clone(),
        minimap_handle: minimap_handle.clone(),
    };
    commands.insert_resource(ui_config);

    info!("ui node setup");
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    height: Val::Vh(100.0),
                    // flex_direction: FlexDirection::Column,
                    width: Val::Vw(100.0),
                    // align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Name::new("ui_root_node"),
            UI_LAYER,
        ))
        // Add the HUD withing the UI node using Image Bundle
        .with_children(|ui_root_node| {
            ui_root_node
                .spawn((
                    ImageBundle {
                        style: Style {
                            min_height: Val::Percent(100.0),
                            max_width: Val::Percent(100.0),
                            aspect_ratio: Some(ASPECT_RATIO_4_3),
                            // align_items: AlignItems::Center,
                            // justify_content: JustifyContent::Center,
                            ..default()
                        },
                        image: UiImage::new(textures.hud.clone()),
                        ..default()
                    },
                    UI_LAYER,
                    HudImage,
                    Name::new("hud_image_node"),
                ))
                .with_children(|hud_image_node| {
                    // Add the dungeon view image
                    let left = config.dongeon_view_margin.0 * 100. / config.size.0;
                    let top =
                        config.dongeon_view_margin.1 / ASPECT_RATIO_4_3 * 100. / config.size.1;

                    hud_image_node.spawn((
                        ImageBundle {
                            style: Style {
                                width: Val::Percent(
                                    (config.dungeon_view_size.0 * 100.) / config.size.0,
                                ),
                                height: Val::Percent(
                                    (config.dungeon_view_size.1 * 100.) / config.size.1,
                                ),
                                margin: UiRect::percent(left, 0., top, 0.),
                                position_type: PositionType::Absolute,
                                ..default()
                            },
                            image: UiImage::new(dungeon_handle.clone()),
                            ..default()
                        },
                        UI_LAYER,
                        DungeonViewImage,
                        Name::new("dungeon_view_node"),
                    ));
                })
                .with_children(|hud_image_node| {
                    // Add the minimap image
                    let left = (config.dongeon_view_margin.0
                        + config.dungeon_view_size.0
                        + config.minimap_margin.0)
                        * 100.
                        / config.size.0;
                    let top = config.minimap_margin.1 / ASPECT_RATIO_4_3 * 100. / config.size.1;

                    hud_image_node.spawn((
                        ImageBundle {
                            style: Style {
                                width: Val::Percent((config.minimap_size.0 * 100.) / config.size.0),
                                height: Val::Percent(
                                    (config.minimap_size.1 * 100.) / config.size.1,
                                ),
                                margin: UiRect::percent(left, 0., top, 0.),
                                position_type: PositionType::Absolute,
                                ..default()
                            },
                            image: UiImage::new(minimap_handle.clone()),
                            ..default()
                        },
                        UI_LAYER,
                        MinimapImage,
                        Name::new("minimap_node"),
                    ));
                });
        });
}

/// Listen to the creation of this resource to wait for UI initialization
#[derive(Resource)]
pub struct HUDRenderViews {
    pub dungeon_handle: Handle<Image>,
    pub minimap_handle: Handle<Image>,
}
