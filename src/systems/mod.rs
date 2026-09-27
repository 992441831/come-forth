use bevy::prelude::*;

use crate::components::{CommunicationEvent, DeathEvent, SpawnCreature, SpawnFood, SpawnPredator};
use crate::spatial::update_spatial_grid;

pub mod behavior;
pub mod charts;
pub mod food;
pub mod input;
pub mod life;
pub mod movement;
pub mod predators;
pub mod render;
pub mod setup;

use behavior::*;
use charts::*;
use food::*;
use input::*;
use life::*;
use movement::*;
use predators::*;
use render::*;
use setup::*;

/// Plugin that wires all simulation systems together with correct ordering.
pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnCreature>()
            .add_event::<SpawnPredator>()
            .add_event::<SpawnFood>()
            .add_event::<DeathEvent>()
            .add_event::<behavior::BehaviorIntent>()
            .add_event::<CommunicationEvent>()
            .add_systems(Startup, (
                setup_camera,
                setup_config,
                setup_history,
                spawn_initial_creatures,
                spawn_initial_food,
                spawn_initial_predators,
                setup_ui,
            ).chain())
            .add_systems(FixedUpdate, (
                update_spatial_grid,
                gather_behavior_data,
                apply_behavior,
                resolve_movement,
                predator_behavior,
                consume_food,
                energy_metabolism,
                age_creatures,
                reproduce,
                handle_deaths,
                handle_creature_spawns,
                handle_predator_spawns,
                handle_food_spawns,
                spawn_food_randomly,
                increment_sim_tick,
            ).chain())
            .add_systems(Update, (
                camera_zoom_pan,
                sample_population,
                render_creatures,
                render_food,
                render_predators,
                render_population_chart,
                update_ui_stats,
            ));
    }
}
