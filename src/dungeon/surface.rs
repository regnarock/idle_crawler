use bevy::ecs::system::Command;
use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2, PI};

use super::camera3d::DUNGEON_CAMERA_LAYER;
use super::config::DungeonConfig;
use super::vec_utils::{MoveBy, MoveDirection};
use super::{Layout, Position};
use crate::loading::TextureAssets;
use crate::GameState;

pub struct SurfacePlugin;

impl Plugin for SurfacePlugin {
    fn build(&self, app: &mut App) {
        // TODO: do it during the loading state instead
        app.add_systems(OnExit(GameState::Loading), setup);
    }
}

#[derive(Resource)]
struct RoomResources {
    pub surface_mesh: Handle<Mesh>,
    pub wall_material: Handle<StandardMaterial>,
    pub ceilling_material: Handle<StandardMaterial>,
    pub floor_material: Handle<StandardMaterial>,
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<TextureAssets>,
    config: Res<DungeonConfig>,
) {
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

    let surface_mesh = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        config.size,
        config.size,
    ))));

    commands.insert_resource(RoomResources {
        surface_mesh,
        wall_material,
        ceilling_material,
        floor_material,
    });
}

pub struct SpawnSurfaceCommand {
    pub position: Vec3,
    pub direction: Option<MoveDirection>,
    pub position_id: Position,
}

impl Command for SpawnSurfaceCommand {
    fn apply(self, world: &mut World) {
        let resources = world.resource::<RoomResources>();
        let config = world.resource::<DungeonConfig>();

        let mut transform = Transform::from_translation(
            self.direction
                .map(|d| self.position.move_by(d, config.size))
                .unwrap_or(self.position),
        );

        if let Some(direction) = self.direction {
            let rotation = match direction {
                MoveDirection::Right => Quat::from_rotation_y(-FRAC_PI_2),
                MoveDirection::Left => Quat::from_rotation_y(FRAC_PI_2),
                MoveDirection::Up => Quat::from_rotation_x(FRAC_PI_2),
                MoveDirection::Down => Quat::from_rotation_x(-FRAC_PI_2),
                MoveDirection::Backward => Quat::from_rotation_y(PI),
                MoveDirection::ShiftRight | MoveDirection::ShiftLeft | MoveDirection::Forward => {
                    Quat::IDENTITY
                }
            };
            transform = transform.with_rotation(rotation);
        }

        let material = match self.direction {
            Some(MoveDirection::Up) => resources.ceilling_material.clone(),
            Some(MoveDirection::Down) => resources.floor_material.clone(),
            _ => resources.wall_material.clone(),
        };

        let surface_name: &'static str = self.position_id.into();
        world.spawn((
            PbrBundle {
                mesh: resources.surface_mesh.clone(),
                transform,
                material,
                ..default()
            },
            Layout {
                depth: (-self.position.z / config.size) as usize,
                position: self.position_id,
            },
            Name::new(format!("{} Surface", surface_name)),
            DUNGEON_CAMERA_LAYER,
        ));
    }
}
