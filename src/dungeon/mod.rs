mod camera3d;
mod config;
mod labyrinth;
mod surface;
mod vec_utils;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{bevy_egui::EguiContext, egui};

use strum_macros::IntoStaticStr;

use crate::GameState;

use self::{
    camera3d::Camera3DPlugin,
    config::{ConfigPlugin, DungeonConfig},
    labyrinth::{Labyrinth, LabyrinthPlugin},
    surface::{SpawnSurfaceCommand, SurfacePlugin},
    vec_utils::{MoveBy, MoveDirection},
};

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((Camera3DPlugin, ConfigPlugin, SurfacePlugin, LabyrinthPlugin))
            .add_systems(OnEnter(GameState::Playing), setup)
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 0.1,
            })
            .add_systems(
                Update,
                debug_update_position.run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, ui_example.run_if(in_state(GameState::Playing)))
            .register_type::<Layout>();
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
///     +----------+ <- 6
///     | \   5  / |
///     |  +----+  |
///     |4 |  1 | 2|
///     |  +----+  |
///     | /   3  \ |
///     +----------+
/// ```

#[derive(Component, Reflect)]
pub struct Layout {
    depth: usize,
    position: Position,
}

#[derive(Clone, Copy, Reflect, IntoStaticStr, Hash, PartialEq, Eq)]
pub enum Position {
    Center,  // 1
    Right,   // 2
    Floor,   // 3
    Left,    // 4
    Ceiling, // 5
    Back,    // 6
}

pub fn setup(mut commands: Commands, config: Res<DungeonConfig>, labyrinth: Res<Labyrinth>) {
    commands.spawn((
        PointLightBundle {
            transform: Transform::from_xyz(0., 0., config.size / 2.0 * -1.),
            ..Default::default()
        },
        Name::new("Point Light"),
    ));

    for (&(x, y), cell) in labyrinth.cells.iter() {
        let center = Vec3::new(0.0, 0.0, -config.size);
        let center = center.move_by(MoveDirection::Forward, config.size * y as f32);
        let center = center.move_by(MoveDirection::ShiftRight, config.size * x as f32);
        spawn_room(&mut commands, center, &cell.walls);
    }
}

fn spawn_room(commands: &mut Commands, center: Vec3, side_walls: &[(Position, bool); 6]) {
    for (position, exists) in side_walls.iter() {
        if *exists {
            let direction = match position {
                Position::Center => None,
                Position::Right => Some(MoveDirection::Right),
                Position::Left => Some(MoveDirection::Left),
                Position::Back => Some(MoveDirection::Backward),
                Position::Floor => Some(MoveDirection::Down),
                Position::Ceiling => Some(MoveDirection::Up),
            };
            commands.add(SpawnSurfaceCommand {
                position: center,
                direction,
                position_id: *position,
            })
        }
    }
}

pub fn debug_update_position(
    config: Res<DungeonConfig>,
    mut ambiant_light: ResMut<AmbientLight>,
    mut q_light: Query<(&mut Transform, &mut PointLight)>,
) {
    ambiant_light.brightness = 0.0;
    if let Ok((mut transform, mut light)) = q_light.get_single_mut() {
        transform.translation.x = config.light_x;
        transform.translation.y = config.light_y;
        transform.translation.z = config.light_z;
        light.intensity = config.brightness;
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
