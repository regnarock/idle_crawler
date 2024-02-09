use bevy::prelude::*;

pub struct UIConfigPlugin;

impl Plugin for UIConfigPlugin {
    fn build(&self, app: &mut App) {
        // inner square
        let width_dungeon_window = 800.0;
        let height_dungeon_window = 800.0;
        // including margin
        let width_dungeon_window_with_margin = 1250.0;
        let height_dungeon_window_with_margin = 1075.0;

        let width_margin = width_dungeon_window_with_margin - width_dungeon_window;
        let height_margin = height_dungeon_window_with_margin - height_dungeon_window;

        let center_x = width_dungeon_window / 2.0 + width_margin;
        let center_y = height_dungeon_window / 2.0 + height_margin;

        app.insert_resource(UIConfig {
            dungeon_window_size: Vec2::new(width_dungeon_window, height_dungeon_window),
            dungeon_window_pos: Vec2::new(center_x, center_y),
            margins: Vec2::new(width_margin, height_margin),
        });
    }
}

#[derive(Resource)]
pub struct UIConfig {
    pub dungeon_window_size: Vec2,
    pub dungeon_window_pos: Vec2,
    pub margins: Vec2,
}
