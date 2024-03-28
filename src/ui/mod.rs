mod camera2d;

use bevy::{prelude::*, ui::widget::UiImageSize, window::WindowResized};

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
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update,
                (update_ui_config_on_window_resize.run_if(on_event::<WindowResized>()))
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct HudImage;

pub fn setup(mut commands: Commands, textures: Res<TextureAssets>, config: Res<Assets<UIConfig>>) {
    let config = config.get(textures.hud_config.id()).unwrap();

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
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    style: Style {
                        min_height: Val::Percent(100.0),
                        max_width: Val::Percent(100.0),
                        aspect_ratio: Some(1.3574498567),
                        ..default()
                    },
                    image: UiImage::new(textures.hud.clone()),
                    ..default()
                },
                UI_LAYER,
                HudImage,
                Name::new("hud_node"),
            ));
        });

    commands.insert_resource(ScaledUiConfig {
        size: config.size,
        dongeon_view_margin: config.dongeon_view_margin,
        dungeon_view_size: config.dungeon_view_size,
        minimap_margin: config.minimap_margin,
        minimap_size: config.minimap_size,
        padding: 0.,
    });
}

/// Listen to the creation of this resource to wait for UI initialization
#[derive(Resource)]
pub struct ScaledUiConfig {
    pub size: (f32, f32),
    pub dongeon_view_margin: (f32, f32),
    pub dungeon_view_size: (f32, f32),
    pub minimap_margin: (f32, f32),
    pub minimap_size: (f32, f32),
    pub padding: f32,
}

pub fn update_ui_config_on_window_resize(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    config: Res<Assets<UIConfig>>,
    mut er: EventReader<WindowResized>,
    hud: Query<(&UiImageSize, &Node), With<HudImage>>,
    window: Query<&Window>,
) {
    for _ in er.read() {
        if let (Ok((ui_image_size, node)), Ok(window)) = (hud.get_single(), window.get_single()) {
            // Calculate the UI scale ratio by dividing the real HUD image width by the dynamically calculated width
            // FIXME: the scale ratio is not correct, we need to find a way to calculate it correctly and remove the * 2.0
            let new_scale_ratio = node.size().x / ui_image_size.size().x * 2.;

            let new_padding: f32 = window.width() - node.size().x;
            let config = config.get(textures.hud_config.id()).unwrap();

            commands.insert_resource(ScaledUiConfig {
                size: (
                    config.size.0 * new_scale_ratio,
                    config.size.1 * new_scale_ratio,
                ),
                dongeon_view_margin: (
                    config.dongeon_view_margin.0 * new_scale_ratio,
                    config.dongeon_view_margin.1 * new_scale_ratio,
                ),
                dungeon_view_size: (
                    config.dungeon_view_size.0 * new_scale_ratio,
                    config.dungeon_view_size.1 * new_scale_ratio,
                ),
                minimap_margin: (
                    config.minimap_margin.0 * new_scale_ratio,
                    config.minimap_margin.1 * new_scale_ratio,
                ),
                minimap_size: (
                    config.minimap_size.0 * new_scale_ratio,
                    config.minimap_size.1 * new_scale_ratio,
                ),
                padding: new_padding,
            });
        }
    }
}
