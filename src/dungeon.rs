use std::f32::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_8, PI};

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{
    bevy_egui::{self, EguiContext},
    egui,
    inspector_options::{std_options::NumberDisplay, ReflectInspectorOptions},
    InspectorOptions,
};

use crate::GameState;

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .insert_resource(DungeonConfig {
                size: 2.0,
                distance_field: 6.0,
                brightness: 10.0,
                left: 0,
                right: 0,
                center: 1,
                light_x: 0.0,
                light_y: 0.0,
                light_z: 2.0,
            })
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 0.1,
            })
            .register_type::<DungeonConfig>()
            .add_systems(
                Update,
                update_walls_visibility.run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, draw_debug.run_if(in_state(GameState::Playing)))
            .add_systems(Update, ui_example.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Resource, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct DungeonConfig {
    pub size: f32,
    pub distance_field: f32,
    #[inspector(min = 5.0, max = 50.0, display = NumberDisplay::Slider)]
    pub brightness: f32,
    #[inspector(min = -10.0, max = 10.0, display = NumberDisplay::Slider)]
    pub light_x: f32,
    #[inspector(min = -10.0, max = 10.0, display = NumberDisplay::Slider)]
    pub light_y: f32,
    #[inspector(min = -10.0, max = 10.0, display = NumberDisplay::Slider)]
    pub light_z: f32,

    #[inspector(min = 0, max = 1, display = NumberDisplay::Slider)]
    pub left: usize,
    #[inspector(min = 0, max = 1, display = NumberDisplay::Slider)]
    pub right: usize,
    #[inspector(min = 0, max = 1, display = NumberDisplay::Slider)]
    pub center: usize,
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
///    +----------+
///    | \   5  / |
///    |  +----+  |
/// 4->|  |  1 |  |<-2
///    |  +----+  |
///    | /   3  \ |
///    +----------+
/// ```
/// In this cube, each face could be a separate entity with its own `WallMarker<ID>`.

#[derive(Component)]
pub struct Layout {
    depth: usize,
    position: Position,
}

pub enum Position {
    Center,  // 1
    Right,   // 2
    Floor,   // 3
    Left,    // 4
    Ceiling, // 5
}

pub fn setup(
    mut commands: Commands,
    config: Res<DungeonConfig>,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut gizmo_config: ResMut<GizmoConfig>,
    //mut ambient_light: ResMut<AmbientLight>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 0., config.distance_field)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    //    ambient_light.brightness = config.brightness;
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0., 0., config.distance_field),
        ..Default::default()
    });

    //gizmo_config.depth_bias = -1.0;

    let wall: Handle<Image> = assets.load("textures/Wall_2.png");
    let wall_normal: Handle<Image> = assets.load("textures/Wall_2_normal.png");
    let floor: Handle<Image> = assets.load("textures/floor.png");
    let wall_material = materials.add(StandardMaterial {
        base_color_texture: Some(wall),
        normal_map_texture: Some(wall_normal),
        perceptual_roughness: 1.,
        ..Default::default()
    });
    let floor_material = materials.add(StandardMaterial {
        base_color_texture: Some(floor),
        perceptual_roughness: 0.9,
        ..Default::default()
    });
    let mesh = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        config.size,
        config.size,
    ))));

    // Depth 0

    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            transform: Transform::from_xyz(0., 0., 0.),
            material: wall_material.clone(),
            ..default()
        },
        Layout {
            depth: 0,
            position: Position::Center,
        },
    ));
    let center = Vec3::new(0.0, 0.0, 0.0);
    let square = Vec2::splat(config.size);
    let flattened_square = square * Vec2::new(1.0, config.distance_field / 4.);

    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            transform: Transform::from_translation(Vec3::new(
                flattened_square.x * 0.5,
                0.0,
                flattened_square.y * 0.5,
            ))
            .with_scale(Vec3::new(config.distance_field / 4., 1.0, 1.0))
            .with_rotation(Quat::from_rotation_y(-FRAC_PI_2)),
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
            transform: Transform::from_translation(Vec3::new(
                flattened_square.x * -0.5,
                0.0,
                flattened_square.y * 0.5,
            ))
            .with_scale(Vec3::new(config.distance_field / 4., 1.0, 1.0))
            .with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
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
                center.move_by(Direction::Down, flattened_square),
            )
            .with_scale(Vec3::new(1.0, config.distance_field / 4., 1.0))
            .with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
            material: floor_material.clone(),
            ..default()
        },
        Layout {
            depth: 0,
            position: Position::Floor,
        },
    ));

    // Depth 1
    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            transform: Transform::from_translation(
                center.move_by(Direction::ShiftLeft, flattened_square),
            ),
            material: wall_material.clone(),
            ..default()
        },
        Layout {
            depth: 1,
            position: Position::Left,
        },
    ));
    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            transform: Transform::from_translation(
                center.move_by(Direction::ShiftRight, flattened_square),
            ),
            material: wall_material.clone(),
            ..default()
        },
        Layout {
            depth: 1,
            position: Position::Right,
        },
    ));
    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            transform: Transform::from_translation(
                center
                    .move_by(Direction::ShiftLeft, flattened_square)
                    .move_by(Direction::Down, flattened_square),
            )
            .with_rotation(Quat::from_rotation_x(-FRAC_PI_2))
            .with_scale(Vec3::new(1.0, config.distance_field / 4., 1.0)),
            material: floor_material.clone(),
            ..default()
        },
        Layout {
            depth: 1,
            position: Position::Floor, // + Position::Left
        },
    ));
    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            transform: Transform::from_translation(
                center
                    .move_by(Direction::ShiftRight, flattened_square)
                    .move_by(Direction::Down, flattened_square),
            )
            .with_rotation(Quat::from_rotation_x(-FRAC_PI_2))
            .with_scale(Vec3::new(1.0, config.distance_field / 4., 1.0)),
            material: floor_material.clone(),
            ..default()
        },
        Layout {
            depth: 1,
            position: Position::Floor, // + Position::Right
        },
    ));
}

pub fn update_walls_visibility(
    config: Res<DungeonConfig>,
    mut q_light: Query<(&mut Transform, &mut PointLight)>,
    mut q_walls: Query<(Entity, &Layout, &mut Visibility)>,
) {
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
        if layout.depth == 0 {
            // take care of depth 0
            match *layout {
                Layout {
                    position: Position::Center,
                    ..
                } => update_visibility(visibility, config.center == 0),
                Layout {
                    position: Position::Right,
                    ..
                } => update_visibility(visibility, config.right == 0),
                Layout {
                    position: Position::Floor,
                    ..
                } => {}
                Layout {
                    position: Position::Left,
                    ..
                } => update_visibility(visibility, config.left == 0),
                Layout {
                    position: Position::Ceiling,
                    ..
                } => {}
            }
        }
    }
}

trait MoveBy {
    fn move_by(self, d: Direction, s: Vec2) -> Self;
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
    ShiftLeft,
    HalfShiftLeft,
    ShiftRight,
    HalfShiftRight,
}

impl MoveBy for Vec3 {
    fn move_by(self, d: Direction, s: Vec2) -> Self {
        match d {
            Direction::Up => self + Vec3::new(0.0, s.x / 2.0, s.y / 2.0),
            Direction::Down => self + Vec3::new(0.0, s.x / -2.0, s.y / 2.0),
            Direction::ShiftLeft => self + Vec3::new(s.x * -1., 0.0, 0.0),
            Direction::HalfShiftLeft => self + Vec3::new(s.x * -0.75, 0.0, 0.0),
            Direction::Left => self + Vec3::new(s.x * -0.5, 0.0, s.y * 0.5),
            Direction::ShiftRight => self + Vec3::new(s.x, 0.0, 0.0),
            Direction::HalfShiftRight => self + Vec3::new(s.x * 0.75, 0.0, 0.0),
            Direction::Right => self + Vec3::new(s.x * 0.5, 0.0, s.y * 0.5),
        }
    }
}

/// A rectangle triangle gizmo, with the right angle at the bottom left
pub trait TriangleGizmos {
    fn triangle_rect(&mut self, position: Vec3, rotation: Quat, a: f32, b: f32, color: Color);
}

impl<'s> TriangleGizmos for Gizmos<'s> {
    fn triangle_rect(&mut self, position: Vec3, rotation: Quat, a: f32, b: f32, color: Color) {
        let [tl, tr, bl] = rect_inner(a, b).map(|vec2| position + rotation * vec2.extend(0.));
        self.linestrip([tl, tr, bl, tl], color);
        fn rect_inner(a: f32, b: f32) -> [Vec2; 3] {
            let half_a = a / 2.;
            let half_b = b / 2.;
            let tl = Vec2::new(-half_a, half_b);
            let tr = Vec2::new(half_a, half_b);
            let bl = Vec2::new(half_a, -half_b);
            [tl, tr, bl]
        }
    }
}

pub fn draw_debug(
    mut gizmos: Gizmos,
    time: Res<Time>,
    config: Res<DungeonConfig>,
    mut q_walls_transform: Query<&mut Transform>,
) {
    let mut center = Vec3::new(0.0, 0.0, 0.0);

    let square = Vec2::splat(config.size);
    let half = |s: Vec2| s * Vec2::new(0.5, 1.0);
    let flattened_square = square * Vec2::new(1.0, config.distance_field / 4.);

    // Debug only
    // if time.elapsed_seconds() % 6.0 > 3.0 {
    //     center.x = (time.elapsed_seconds() * 0.5).sin() * 2.0;
    // }
    // Layer 1
    // middle of the screen
    gizmos.rect(center, Quat::IDENTITY, square, Color::BLUE);

    // ceiling
    gizmos.rect(
        center.move_by(Direction::Up, flattened_square),
        Quat::from_rotation_x(FRAC_PI_2),
        flattened_square,
        Color::RED,
    );

    // floor
    gizmos.rect(
        center.move_by(Direction::Down, flattened_square),
        Quat::from_rotation_x(FRAC_PI_2),
        flattened_square,
        Color::RED,
    );
    // might be hidden left/right
    {
        if config.left == 0 {
            // middle left
            gizmos.rect(
                center.move_by(Direction::HalfShiftLeft, square),
                Quat::IDENTITY,
                half(square),
                Color::BLUE,
            );

            // floor left
            gizmos.triangle_rect(
                center
                    .move_by(Direction::Down, flattened_square)
                    .move_by(Direction::HalfShiftLeft, flattened_square),
                Quat::from_rotation_x(-FRAC_PI_2),
                half(flattened_square).x,
                flattened_square.y,
                Color::RED,
            );

            // ceiling left
            gizmos.triangle_rect(
                center
                    .move_by(Direction::Up, flattened_square)
                    .move_by(Direction::HalfShiftLeft, flattened_square),
                Quat::from_rotation_x(-FRAC_PI_2),
                half(flattened_square).x,
                flattened_square.y,
                Color::RED,
            );
        } else {
            // left wall
            gizmos.rect(
                center + Vec3::new(flattened_square.x / -2.0, 0.0, flattened_square.y / 2.0),
                Quat::from_rotation_y(FRAC_PI_2).mul_quat(Quat::from_rotation_z(FRAC_PI_2)),
                flattened_square.xy(),
                Color::GREEN,
            );
        }
        if config.right == 0 {
            // middle right
            gizmos.rect(
                center.move_by(Direction::HalfShiftRight, square),
                Quat::IDENTITY,
                half(square),
                Color::BLUE,
            );
            // floor right
            gizmos.triangle_rect(
                center
                    .move_by(Direction::Down, flattened_square)
                    .move_by(Direction::HalfShiftRight, flattened_square),
                Quat::from_rotation_x(FRAC_PI_2).mul_quat(Quat::from_rotation_z(PI)),
                half(flattened_square).x,
                flattened_square.y,
                Color::RED,
            );

            // ceiling right
            gizmos.triangle_rect(
                center
                    .move_by(Direction::Up, flattened_square)
                    .move_by(Direction::HalfShiftRight, flattened_square),
                Quat::from_rotation_x(FRAC_PI_2).mul_quat(Quat::from_rotation_z(PI)),
                half(flattened_square).x,
                flattened_square.y,
                Color::RED,
            );
        } else {
            // right wall
            gizmos.rect(
                center.move_by(Direction::Right, flattened_square),
                Quat::from_rotation_y(FRAC_PI_2).mul_quat(Quat::from_rotation_z(FRAC_PI_2)),
                flattened_square.xy(),
                Color::GREEN,
            );
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
