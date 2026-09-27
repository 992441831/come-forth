use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::collections::HashSet;

use crate::components::{Creature, Energy, Food, Position, SpawnFood};
use crate::config::SimConfig;
use crate::spatial::{nearby_entities, SpatialGrid};

const EAT_RADIUS: f32 = 8.0;

pub fn consume_food(
    config: Res<SimConfig>,
    grid: Res<SpatialGrid>,
    mut commands: Commands,
    mut creatures: Query<(Entity, &Position, &mut Energy), With<Creature>>,
    foods: Query<&Position, With<Food>>,
) {
    let mut scratch = Vec::with_capacity(32);
    let mut eaten_this_tick = HashSet::new();

    for (_creature_entity, pos, mut energy) in creatures.iter_mut() {
        scratch.clear();
        nearby_entities(&grid, pos.0, EAT_RADIUS, &mut scratch);

        for &food_entity in scratch.iter() {
            if eaten_this_tick.contains(&food_entity) {
                continue;
            }
            if let Ok(food_pos) = foods.get(food_entity) {
                if pos.0.distance(food_pos.0) <= EAT_RADIUS {
                    energy.current = (energy.current + config.food_energy_value).min(energy.max);
                    eaten_this_tick.insert(food_entity);
                    if let Some(mut food) = commands.get_entity(food_entity) {
                        food.despawn();
                    }
                }
            }
        }
    }
}

pub fn spawn_food_randomly(
    config: Res<SimConfig>,
    mut events: EventWriter<SpawnFood>,
) {
    let mut rng = thread_rng();
    let half = config.world_half_size;

    for _ in 0..config.food_spawn_per_tick {
        events.send(SpawnFood {
            pos: Vec2::new(
                rng.gen_range(-half.x..half.x),
                rng.gen_range(-half.y..half.y),
            ),
        });
    }
}

pub fn handle_food_spawns(
    mut commands: Commands,
    mut events: EventReader<SpawnFood>,
) {
    for event in events.read() {
        commands.spawn((Food, Position(event.pos)));
    }
}
