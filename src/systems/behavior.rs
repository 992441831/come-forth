use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::components::*;
use crate::config::SimConfig;
use crate::spatial::{nearby_entities, SpatialGrid};

/// Event emitted by `gather_behavior_data`, consumed by `apply_behavior`.
#[derive(Event)]
pub struct BehaviorIntent {
    pub entity: Entity,
    pub new_velocity: Vec2,
    pub is_fleeing: bool,
}

/// Read-only pass over all creatures: find food, find same-species neighbors,
/// detect predators, decide steering, and emit both movement intents and communication events.
pub fn gather_behavior_data(
    config: Res<SimConfig>,
    grid: Res<SpatialGrid>,
    creatures: Query<
        (Entity, &Position, &Velocity, &Energy, &Species, &CommunicationBuffer),
        With<Creature>,
    >,
    foods: Query<&Position, With<Food>>,
    predators: Query<&Position, With<Predator>>,
    neighbors: Query<(&Position, &Species), With<Creature>>,
    mut intents: EventWriter<BehaviorIntent>,
    mut comm_events: EventWriter<CommunicationEvent>,
) {
    let mut rng = thread_rng();
    let mut scratch = Vec::with_capacity(64);

    for (entity, pos, velocity, energy, species, buffer) in creatures.iter() {
        let hunger_ratio = energy.current / energy.max;
        let is_hungry = hunger_ratio < config.hungry_threshold;

        let mut desired = Vec2::ZERO;
        let mut known_food_dir: Option<Vec2> = None;
        let mut is_fleeing = false;

        // Rule 0: flee from nearby predators.
        scratch.clear();
        nearby_entities(&grid, pos.0, config.predator_flee_radius, &mut scratch);
        let mut nearest_predator_dist_sq = f32::MAX;
        let mut flee_dir = Vec2::ZERO;

        for &other in scratch.iter() {
            if other == entity {
                continue;
            }
            if let Ok(predator_pos) = predators.get(other) {
                let offset = pos.0 - predator_pos.0;
                let dist_sq = offset.length_squared();
                if dist_sq <= config.predator_flee_radius * config.predator_flee_radius
                    && dist_sq < nearest_predator_dist_sq
                {
                    nearest_predator_dist_sq = dist_sq;
                    flee_dir = offset.normalize_or_zero();
                    is_fleeing = true;
                }
            }
        }

        if is_fleeing {
            desired += flee_dir * config.flee_strength;
        }

        // Rule 1: hungry creatures seek food.
        if is_hungry && !is_fleeing {
            if let Some(food_dir) = find_nearest_food(&grid, &foods, pos.0, config.perception_radius, &mut scratch) {
                desired += food_dir * config.seek_strength;
                known_food_dir = Some(food_dir);
            }

            // Blend in communicated food direction if available.
            if let Some(message) = buffer.last_message {
                if let Some(msg_dir) = message.food_direction {
                    desired += msg_dir * config.seek_strength * 0.5;
                    if known_food_dir.is_none() {
                        known_food_dir = Some(msg_dir);
                    }
                }
            }
        }

        // Rule 3: communicate with same-species neighbors.
        if is_hungry && known_food_dir.is_some() && !is_fleeing {
            let food_dir = known_food_dir.unwrap();
            scratch.clear();
            nearby_entities(&grid, pos.0, config.communication_radius, &mut scratch);

            for &other in scratch.iter() {
                if other == entity {
                    continue;
                }
                if let Ok((_other_pos, other_species)) = neighbors.get(other) {
                    if other_species.id == species.id {
                        // Only tell hungry-looking neighbors (they have less energy).
                        // We don't have their Energy here, so approximate by distance + random.
                        // A more precise version would add Energy to the neighbor query.
                        comm_events.send(CommunicationEvent {
                            receiver: other,
                            message: Message {
                                sender_pos: pos.0,
                                food_direction: Some(food_dir),
                                sender_energy: energy.current,
                            },
                        });
                    }
                }
            }
        }

        // Wander to avoid getting stuck.
        let wander_angle = rng.gen_range(0.0..std::f32::consts::TAU);
        desired += Vec2::new(wander_angle.cos(), wander_angle.sin()) * config.wander_strength;

        // Smooth steering + clamp to max speed.
        // Fleeing creatures get a slight speed boost so they can actually escape.
        let max_speed = if is_fleeing {
            config.max_speed * 1.15
        } else {
            config.max_speed
        };
        let new_velocity = (velocity.0 + desired).clamp_length_max(max_speed);

        intents.send(BehaviorIntent {
            entity,
            new_velocity,
            is_fleeing,
        });
    }
}

/// Apply the intents and the communication events to creature components.
pub fn apply_behavior(
    mut commands: Commands,
    mut creatures: Query<(Entity, &mut Velocity, &mut CommunicationBuffer), With<Creature>>,
    mut intents: EventReader<BehaviorIntent>,
    mut comm_events: EventReader<CommunicationEvent>,
) {
    // Clear last tick's messages and fleeing state.
    for (entity, _, mut buffer) in creatures.iter_mut() {
        buffer.last_message = None;
        commands.entity(entity).remove::<Fleeing>();
    }

    // Apply velocities.
    for intent in intents.read() {
        if let Ok((entity, mut velocity, _)) = creatures.get_mut(intent.entity) {
            velocity.0 = intent.new_velocity;
            if intent.is_fleeing {
                commands.entity(entity).insert(Fleeing);
            }
        }
    }

    // Apply received messages.
    for event in comm_events.read() {
        if let Ok((_, _, mut buffer)) = creatures.get_mut(event.receiver) {
            buffer.last_message = Some(event.message);
        }
    }
}

fn find_nearest_food(
    grid: &SpatialGrid,
    foods: &Query<&Position, With<Food>>,
    pos: Vec2,
    radius: f32,
    scratch: &mut Vec<Entity>,
) -> Option<Vec2> {
    scratch.clear();
    nearby_entities(grid, pos, radius, scratch);

    let mut nearest: Option<(Entity, f32)> = None;
    for &entity in scratch.iter() {
        if let Ok(food_pos) = foods.get(entity) {
            let dist_sq = pos.distance_squared(food_pos.0);
            if dist_sq <= radius * radius {
                if nearest.map(|(_, d)| dist_sq < d).unwrap_or(true) {
                    nearest = Some((entity, dist_sq));
                }
            }
        }
    }

    nearest.map(|(entity, _)| {
        let food_pos = foods.get(entity).unwrap().0;
        (food_pos - pos).normalize_or_zero()
    })
}
