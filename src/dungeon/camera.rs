use bevy::prelude::*;
use leafwing_input_manager::{
    action_state::ActionState, input_map::InputMap, plugin::InputManagerPlugin, Actionlike,
    InputManagerBundle,
};

use crate::GameState;

use super::config::DungeonConfig;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<CameraMoves>::default())
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update,
                update_camera_moves.run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct DungeonCamera;

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

pub fn setup(mut commands: Commands, config: Res<DungeonConfig>) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 0., config.distance_field * 2.0)
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
}

pub fn update_camera_moves(
    mut query: Query<
        (&ActionState<CameraMoves>, &mut Transform),
        (With<DungeonCamera>, Changed<ActionState<CameraMoves>>),
    >,
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
