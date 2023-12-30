use std::f32::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_8, PI};

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{
    bevy_egui::{self, EguiContext},
    egui,
    inspector_options::{std_options::NumberDisplay, ReflectInspectorOptions},
    InspectorOptions,
};
use leafwing_input_manager::{
    action_state::ActionState, input_map::InputMap, plugin::InputManagerPlugin, Actionlike,
    InputManagerBundle,
};

use crate::{loading::TextureAssets, GameState};

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<CameraMoves>::default())
            .add_systems(OnEnter(GameState::Playing), setup)
            .insert_resource(DungeonConfig {
                size: 2.0,
                distance_field: 6.0,
                brightness: 10.0,
                left: 0,
                right: 0,
                center: 1,
                center_center: 1,
                light_x: 0.0,
                light_y: 0.0,
                light_z: 2.0,
                debug: 0,
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
            .add_systems(Update, ui_example.run_if(in_state(GameState::Playing)))
            .add_systems(
                Update,
                update_camera_moves.run_if(in_state(GameState::Playing)),
            );
    }
}

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum CameraMoves {
    Left,
    Right,
    // Up,
    // Down,
    // Forward,
    // Backward,
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
    #[inspector(min = 0, max = 1, display = NumberDisplay::Slider)]
    pub center_center: usize,
    #[inspector(min = 0, max = 1, display = NumberDisplay::Slider)]
    pub debug: usize,
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
///         +--|  +----+  |--+
///    9 -> |  |4 |  1 | 2|  | <- 6
///         +--|  +----+  |--+
///  8 -> /    | /   3  \ |    \ <- 7
///     +------+----------+------+
/// ```
/// In this cube, each face could be a separate entity with its own `WallMarker<ID>`.

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

#[derive(Component)]
pub struct DungeonCamera;

pub fn update_camera_moves(
    mut query: Query<(&ActionState<CameraMoves>, &mut Transform), With<DungeonCamera>>,
    time: Res<Time>,
) {
    let (action_state, mut transform) = query.single_mut();
    // Each action has a button-like state of its own that you can check
    if action_state.pressed(CameraMoves::Left) {
        transform.translation.x -= time.delta_seconds();
    } else if action_state.pressed(CameraMoves::Right) {
        transform.translation.x += time.delta_seconds();
    }
}

pub fn setup(
    mut commands: Commands,
    config: Res<DungeonConfig>,
    assets: Res<TextureAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut gizmo_config: ResMut<GizmoConfig>,
    //mut ambient_light: ResMut<AmbientLight>,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 0., config.distance_field)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        InputManagerBundle::<CameraMoves> {
            // Stores "which actions are currently pressed"
            action_state: ActionState::default(),
            // Describes how to convert from player inputs into those actions
            input_map: InputMap::new([
                (KeyCode::A, CameraMoves::Left),
                (KeyCode::D, CameraMoves::Right),
            ]),
        },
        DungeonCamera,
    ));
    //    ambient_light.brightness = config.brightness;
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0., 0., config.distance_field),
        ..Default::default()
    });

    //gizmo_config.depth_bias = -1.0;

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
    let mesh = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        config.size,
        config.size,
    ))));

    // Depth 0

    spawn_center(
        &mut commands,
        config.distance_field / 4.0,
        config.size,
        mesh.clone(),
        wall_material.clone(),
        floor_material.clone(),
        0,
    );

    spawn_center(
        &mut commands,
        config.distance_field / 4.0,
        config.size,
        mesh.clone(),
        wall_material.clone(),
        floor_material.clone(),
        1,
    );

    spawn_center(
        &mut commands,
        config.distance_field / 4.0,
        config.size,
        mesh.clone(),
        wall_material.clone(),
        floor_material.clone(),
        2,
    );
    let center = Vec3::new(0.0, 0.0, 0.0);
    let square = Vec2::splat(config.size);
    let flattened_square = square * Vec2::new(1.0, config.distance_field / 4.0);

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
            depth: 0,
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
            depth: 0,
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
            depth: 0,
            position: Position::FloorLeft,
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
            depth: 0,
            position: Position::FloorRight,
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
    depth: usize,
) {
    let center = Vec3::new(0.0, 0.0, distance_field * depth as f32 * -2.);
    let square = Vec2::splat(size);
    let flattened_square = square * Vec2::new(1.0, distance_field);

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
            transform: Transform::from_translation(Vec3::new(
                flattened_square.x * 0.5,
                0.0,
                flattened_square.y * 0.5 + center.z,
            ))
            .with_scale(Vec3::new(distance_field, 1.0, 1.0))
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
            transform: Transform::from_translation(Vec3::new(
                flattened_square.x * -0.5,
                0.0,
                flattened_square.y * 0.5 + center.z,
            ))
            .with_scale(Vec3::new(distance_field, 1.0, 1.0))
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
                center.move_by(Direction::Down, flattened_square),
            )
            //.with_translation(Vec3::new(0.0, 0.0, z))
            .with_scale(Vec3::new(1.0, distance_field, 1.0))
            .with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
            material: floor_material.clone(),
            ..default()
        },
        Layout {
            depth,
            position: Position::Floor,
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
            match layout.position {
                Position::Center => update_visibility(visibility, config.center == 0),
                Position::FrontRight => update_visibility(visibility, config.right == 0),
                Position::FrontLeft => update_visibility(visibility, config.left == 0),
                _ => {}
            }
        } else if layout.depth == 1 {
            // take care of depth 1
            match layout.position {
                Position::Center => update_visibility(visibility, config.center_center == 0),
                _ => {}
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
    if config.debug == 0 {
        return;
    }
    let size = config.size;
    let distance_field = config.distance_field;
    let left = config.left == 1;
    let right = config.right == 1;
    let center = config.center == 1;
    let center_center = config.center_center == 1;

    display_center(
        size,
        distance_field,
        &mut gizmos,
        left,
        right,
        center,
        center_center,
        0,
    );
}

fn display_center(
    size: f32,
    distance_field: f32,
    gizmos: &mut Gizmos<'_>,
    has_left: bool,
    has_right: bool,
    has_center: bool,
    has_center_center: bool,
    depth: usize,
) {
    let mut center = Vec3::new(0.0, 0.0, distance_field * depth as f32 / -2.);

    let square = Vec2::splat(size);
    let half = |s: Vec2| s * Vec2::new(0.5, 1.0);
    let flattened_square = square * Vec2::new(1.0, distance_field / 4.);

    // Layer 1
    // middle of the screen
    if has_center {
        gizmos.rect(center, Quat::IDENTITY, square, Color::BLUE);
    } else if depth < 2 {
        display_center(
            size,
            distance_field,
            gizmos,
            true,
            true,
            has_center_center,
            false,
            depth + 1,
        );
    }

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
        if !has_left {
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
        if !has_right {
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
