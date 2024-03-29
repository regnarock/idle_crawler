use bevy::prelude::*;
use bevy_inspector_egui::{
    inspector_options::{std_options::NumberDisplay, ReflectInspectorOptions},
    InspectorOptions,
};

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DungeonConfig {
            size: 2.0,
            brightness: 30.0,
            light_x: 0.0,
            light_y: 0.0,
            light_z: -1.0,
            debug: 0,
        })
        .register_type::<DungeonConfig>();
    }
}

#[derive(Resource, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct DungeonConfig {
    pub size: f32,
    #[inspector(min = 5.0, max = 50.0, display = NumberDisplay::Slider)]
    pub brightness: f32,
    #[inspector(min = -10.0, max = 10.0, display = NumberDisplay::Slider)]
    pub light_x: f32,
    #[inspector(min = -10.0, max = 10.0, display = NumberDisplay::Slider)]
    pub light_y: f32,
    #[inspector(min = -20.0, max = 10.0, display = NumberDisplay::Slider)]
    pub light_z: f32,

    #[inspector(min = 0, max = 1, display = NumberDisplay::Slider)]
    pub debug: usize,
}
