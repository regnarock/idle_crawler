#![allow(clippy::type_complexity)]

mod audio;
mod dungeon;
mod labyrinth;
mod loading;
mod menu;
mod ui;

use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;

use bevy::app::App;
use bevy::prelude::*;
use dungeon::DungeonPlugin;
use labyrinth::DungeonLabyrinthPlugin;
use ui::UIPlugin;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins((
            LoadingPlugin,
            MenuPlugin,
            InternalAudioPlugin,
            DungeonPlugin,
            DungeonLabyrinthPlugin,
            UIPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            //app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}
