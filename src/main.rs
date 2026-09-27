use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod components;
mod config;
mod spatial;
mod systems;

use config::SimConfig;
use systems::SimPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Emergent Behavior Simulation".into(),
                resolution: (1600.0_f32, 1000.0_f32).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .insert_resource(Time::<Fixed>::from_hz(SimConfig::default().ticks_per_second as f64))
        .add_plugins(SimPlugin)
        .run();
}
