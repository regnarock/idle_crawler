use bevy::{prelude::*, utils::HashMap};

use crate::GameState;

use super::Position;

pub struct LabyrinthPlugin;

impl Plugin for LabyrinthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Loading), setup)
            .register_type::<Labyrinth>();
    }
}

pub fn setup(mut commands: Commands) {
    // (x,z)
    // + -  - + -  - + -  - +
    // | -1,2    0,2    1,2 |
    // +      +      +      +
    // |         0,1        |
    // +      +      +      +
    // | -1,0    0,0    1,0 |
    // + -  - + -  - + -  - +

    let mut cells = HashMap::new();
    for z in 0..=2 {
        for x in -1..=1 {
            let left_wall = x == -1;
            let right_wall = x == 1;
            let center_wall = z == 2;
            let back_wall = z == 0;

            cells.insert(
                (x, z),
                Cell {
                    walls: [
                        (Position::Center, center_wall),
                        (Position::Left, left_wall),
                        (Position::Right, right_wall),
                        (Position::Back, back_wall),
                        (Position::Ceiling, true),
                        (Position::Floor, true),
                    ],
                },
            );
        }
    }

    commands.insert_resource(Labyrinth { cells })
}

#[derive(Resource, Reflect)]
pub struct Labyrinth {
    pub cells: HashMap<(i32, i32), Cell>,
}

#[derive(Reflect)]
pub struct Cell {
    pub walls: [(Position, bool); 6],
}
