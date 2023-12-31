mod camera;
mod config;
mod debug;
mod vec_utils;

use std::f32::consts::FRAC_PI_2;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{bevy_egui::EguiContext, egui};

use crate::{loading::TextureAssets, GameState};

use self::{
    camera::CameraPlugin,
    config::{ConfigPlugin, DungeonConfig},
    debug::DebugPlugin,
    vec_utils::{MoveBy, MoveDirection},
};

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CameraPlugin, ConfigPlugin, DebugPlugin))
            .add_systems(OnEnter(GameState::Playing), setup)
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 0.1,
            })
            .add_systems(
                Update,
                update_walls_visibility.run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, ui_example.run_if(in_state(GameState::Playing)));
    }
}

/// Helper marker to identify walls in the scene.
///
/// This struct is used as a component to mark entities that represent walls
/// within the dungeon. Each wall can have a unique ID to differentiate it
/// from others.
///
/// # Example Visualization
///
/// Imagine a cube where each face is a wall with a unique `WallMarker<ID>`:
///
/// ```plaintext
///     + -----+----------+----- +
/// 10 -> \    | \   5  / |    / <- 11
///         +--|--+----+--|--+
///    9 -> |  |4 |  1 | 2|  | <- 6
///         +--|--+----+--|--+
///  8 -> /    | /   3  \ |    \ <- 7
///     +------+----------+------+
/// ```

#[derive(Component)]
pub struct Layout {
    depth: usize,
    position: Position,
}

pub enum Position {
    Center,       // 1
    FrontRight,   // 2
    Floor,        // 3
    FrontLeft,    // 4
    Ceiling,      // 5
    Right,        // 6
    FloorRight,   // 7
    FloorLeft,    // 8
    Left,         // 9
    CeilingLeft,  // 10
    CeilingRight, // 11
}

pub fn setup(
    mut commands: Commands,
    config: Res<DungeonConfig>,
    assets: Res<TextureAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0., 0., config.distance_field),
        ..Default::default()
    });

    let wall_material = materials.add(StandardMaterial {
        base_color_texture: Some(assets.wall.clone()),
        normal_map_texture: Some(assets.wall_normal.clone()),
        perceptual_roughness: 1.,
        ..Default::default()
    });
    let floor_material = materials.add(StandardMaterial {
        base_color_texture: Some(assets.floor.clone()),
        perceptual_roughness: 0.9,
        ..Default::default()
    });
    let ceilling_material = materials.add(StandardMaterial {
        base_color_texture: Some(assets.ceilling.clone()),
        perceptual_roughness: 0.9,
        ..Default::default()
    });
    let mesh = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        config.size,
        config.size,
    ))));

    // Depth 0

    spawn_center(
        &mut commands,
        config.distance_field / 2.,
        config.size,
        mesh.clone(),
        wall_material.clone(),
        floor_material.clone(),
        ceilling_material.clone(),
        1,
    );

    spawn_center(
        &mut commands,
        config.distance_field / 2.,
        config.size,
        mesh.clone(),
        wall_material.clone(),
        floor_material.clone(),
        ceilling_material.clone(),
        2,
    );

    spawn_center(
        &mut commands,
        config.distance_field / 2.,
        config.size,
        mesh.clone(),
        wall_material.clone(),
        floor_material.clone(),
        ceilling_material.clone(),
        3,
    );
    let center = Vec3::new(0.0, 0.0, 0.0);
    let square = Vec2::splat(config.size);
    let flattened_square = square * Vec2::new(1.0, config.distance_field / 2.);

    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            transform: Transform::from_translation(
                center.move_by(MoveDirection::ShiftLeft, flattened_square),
            ),
            material: wall_material.clone(),
            ..default()
        },
        Layout {
            depth: 0,
            position: Position::Left,
        },
    ));
    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            transform: Transform::from_translation(
                center.move_by(MoveDirection::ShiftRight, flattened_square),
            ),
            material: wall_material.clone(),
            ..default()
        },
        Layout {
            depth: 0,
            position: Position::Right,
        },
    ));
    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            transform: Transform::from_translation(
                center
                    .move_by(MoveDirection::ShiftLeft, flattened_square)
                    .move_by(MoveDirection::Down, flattened_square),
            )
            .with_rotation(Quat::from_rotation_x(-FRAC_PI_2))
            .with_scale(Vec3::new(1.0, flattened_square.y / 2., 1.0)),
            material: floor_material.clone(),
            ..default()
        },
        Layout {
            depth: 0,
            position: Position::FloorLeft,
        },
    ));
    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            transform: Transform::from_translation(
                center
                    .move_by(MoveDirection::ShiftRight, flattened_square)
                    .move_by(MoveDirection::Down, flattened_square),
            )
            .with_rotation(Quat::from_rotation_x(-FRAC_PI_2))
            .with_scale(Vec3::new(1.0, flattened_square.y / 2., 1.0)),
            material: floor_material.clone(),
            ..default()
        },
        Layout {
            depth: 0,
            position: Position::FloorRight,
        },
    ));
    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            transform: Transform::from_translation(
                center
                    .move_by(MoveDirection::ShiftLeft, flattened_square)
                    .move_by(MoveDirection::Up, flattened_square),
            )
            .with_rotation(Quat::from_rotation_x(FRAC_PI_2))
            .with_scale(Vec3::new(1.0, flattened_square.y / 2., 1.0)),
            material: ceilling_material.clone(),
            ..default()
        },
        Layout {
            depth: 0,
            position: Position::CeilingLeft,
        },
    ));
    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            transform: Transform::from_translation(
                center
                    .move_by(MoveDirection::ShiftRight, flattened_square)
                    .move_by(MoveDirection::Up, flattened_square),
            )
            .with_rotation(Quat::from_rotation_x(FRAC_PI_2))
            .with_scale(Vec3::new(1.0, flattened_square.y / 2., 1.0)),
            material: ceilling_material.clone(),
            ..default()
        },
        Layout {
            depth: 0,
            position: Position::CeilingRight,
        },
    ));
}

fn spawn_center(
    commands: &mut Commands<'_, '_>,
    distance_field: f32,
    size: f32,
    mesh: Handle<Mesh>,
    wall_material: Handle<StandardMaterial>,
    floor_material: Handle<StandardMaterial>,
    ceilling_material: Handle<StandardMaterial>,
    depth: usize,
) {
    let mut z = 0.;
    for d in 1..depth {
        z += distance_field.powi(d as i32) * 4.;
    }
    z = -z;
    let center = Vec3::new(0.0, 0.0, z);
    let square = Vec2::splat(size);
    let y = if depth > 1 {
        distance_field.powi(depth as i32 - 1) * 2.
    } else {
        distance_field
    };
    let flattened_square = square * Vec2::new(1.0, y);
    let scale_factor = 2.;

    info!("Distance {:?} / square_length {:?}", z, y,);

    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            transform: Transform::from_translation(center),
            material: wall_material.clone(),
            ..default()
        },
        Layout {
            depth,
            position: Position::Center,
        },
    ));

    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            transform: Transform::from_translation(
                center.move_by(MoveDirection::Right, flattened_square),
            )
            .with_scale(Vec3::new(flattened_square.y / scale_factor, 1.0, 1.0))
            .with_rotation(Quat::from_rotation_y(-FRAC_PI_2)),
            material: wall_material.clone(),
            ..default()
        },
        Layout {
            depth,
            position: Position::FrontRight,
        },
    ));

    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            transform: Transform::from_translation(
                center.move_by(MoveDirection::Left, flattened_square),
            )
            .with_scale(Vec3::new(flattened_square.y / scale_factor, 1.0, 1.0))
            .with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
            material: wall_material.clone(),
            ..default()
        },
        Layout {
            depth,
            position: Position::FrontLeft,
        },
    ));

    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            transform: Transform::from_translation(
                center.move_by(MoveDirection::Down, flattened_square),
            )
            .with_scale(Vec3::new(1.0, flattened_square.y / scale_factor, 1.0))
            .with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
            material: floor_material.clone(),
            ..default()
        },
        Layout {
            depth,
            position: Position::Floor,
        },
    ));
    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            transform: Transform::from_translation(
                center.move_by(MoveDirection::Up, flattened_square),
            )
            .with_scale(Vec3::new(1.0, flattened_square.y / scale_factor, 1.0))
            .with_rotation(Quat::from_rotation_x(FRAC_PI_2)),
            material: ceilling_material.clone(),
            ..default()
        },
        Layout {
            depth,
            position: Position::Ceiling,
        },
    ));
}

pub fn update_walls_visibility(
    config: Res<DungeonConfig>,
    mut ambiant_light: ResMut<AmbientLight>,
    mut q_light: Query<(&mut Transform, &mut PointLight)>,
    mut q_walls: Query<(Entity, &Layout, &mut Visibility)>,
) {
    ambiant_light.brightness = 0.0;
    if let Ok((mut transform, mut light)) = q_light.get_single_mut() {
        transform.translation.x = config.light_x;
        transform.translation.y = config.light_y;
        transform.translation.z = config.light_z;
        light.intensity = config.brightness;
    }

    let update_visibility = |mut visibility: Mut<Visibility>, conf_value: bool| {
        *visibility = if conf_value {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        };
    };

    for (wall, layout, visibility) in q_walls.iter_mut() {
        if layout.depth == 1 {
            // take care of depth 0
            match layout.position {
                Position::Center => update_visibility(visibility, config.center == 0),
                Position::FrontRight => update_visibility(visibility, config.right == 0),
                Position::FrontLeft => update_visibility(visibility, config.left == 0),
                _ => {}
            }
        } else if layout.depth == 2 {
            // take care of depth 1
            match layout.position {
                Position::Center => update_visibility(visibility, config.center_center == 0),
                _ => {}
            }
        }
    }
}

pub fn ui_example(world: &mut World) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

    egui::Window::new("Dungeon").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            bevy_inspector_egui::bevy_inspector::ui_for_resource::<DungeonConfig>(world, ui);
        });
    });
}
