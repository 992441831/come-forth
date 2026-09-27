use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::components::*;
use crate::config::SimConfig;
use crate::spatial::SpatialGrid;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_config(mut commands: Commands) {
    commands.insert_resource(SimConfig::default());
    commands.insert_resource(SpatialGrid::default());
    commands.insert_resource(SimTick::default());
}

pub fn increment_sim_tick(mut tick: ResMut<SimTick>) {
    tick.count += 1;
}

pub fn spawn_initial_creatures(
    mut commands: Commands,
    config: Res<SimConfig>,
) {
    let mut rng = thread_rng();
    let half = config.world_half_size;

    let creatures: Vec<_> = (0..config.creature_count)
        .map(|_| {
            let species_id = rng.gen_range(0..config.species_count);
            (
                Creature,
                Position(random_pos(&mut rng, half)),
                Velocity(random_direction(&mut rng) * config.max_speed),
                Energy {
                    current: rng.gen_range(30.0..80.0),
                    max: 100.0,
                },
                Age {
                    ticks: 0,
                    max_lifetime: rng.gen_range(1500..2500),
                },
                Species {
                    id: species_id,
                    color: species_color(species_id),
                },
                CommunicationBuffer::default(),
                ReproductionCooldown {
                    timer: rng.gen_range(0.0..config.reproduction_cooldown),
                },
            )
        })
        .collect();

    commands.spawn_batch(creatures);
}

pub fn spawn_initial_food(
    mut commands: Commands,
    config: Res<SimConfig>,
) {
    let mut rng = thread_rng();
    let half = config.world_half_size;

    let food: Vec<_> = (0..config.food_count)
        .map(|_| (Food, Position(random_pos(&mut rng, half))))
        .collect();

    commands.spawn_batch(food);
}

pub fn spawn_initial_predators(
    mut commands: Commands,
    config: Res<SimConfig>,
) {
    let mut rng = thread_rng();
    let half = config.world_half_size;

    let predators: Vec<_> = (0..config.predator_count)
        .map(|_| (
            Predator,
            Position(random_pos(&mut rng, half)),
            Velocity(random_direction(&mut rng) * config.predator_speed),
            Energy {
                current: rng.gen_range(60.0..100.0),
                max: 100.0,
            },
            Age {
                ticks: 0,
                max_lifetime: rng.gen_range(2000..3500),
            },
        ))
        .collect();

    commands.spawn_batch(predators);
}

fn random_pos(rng: &mut impl Rng, half: Vec2) -> Vec2 {
    Vec2::new(
        rng.gen_range(-half.x..half.x),
        rng.gen_range(-half.y..half.y),
    )
}

fn random_direction(rng: &mut impl Rng) -> Vec2 {
    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
    Vec2::new(angle.cos(), angle.sin())
}

pub fn species_color(id: u8) -> Color {
    match id % 4 {
        0 => Color::srgb(0.95, 0.35, 0.25), // red
        1 => Color::srgb(0.25, 0.55, 0.95), // blue
        2 => Color::srgb(0.95, 0.85, 0.25), // yellow
        _ => Color::srgb(0.45, 0.85, 0.35), // green
    }
}

pub fn handle_creature_spawns(
    mut commands: Commands,
    mut events: EventReader<SpawnCreature>,
) {
    let mut rng = thread_rng();

    for event in events.read() {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let velocity = Vec2::new(angle.cos(), angle.sin());

        commands.spawn((
            Creature,
            Position(event.pos),
            Velocity(velocity),
            Energy {
                current: event.energy,
                max: 100.0,
            },
            Age {
                ticks: 0,
                max_lifetime: rng.gen_range(1500..2500),
            },
            Species {
                id: event.species_id,
                color: species_color(event.species_id),
            },
            CommunicationBuffer::default(),
            ReproductionCooldown {
                timer: 10.0,
            },
        ));
    }
}
