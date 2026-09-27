use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::collections::HashSet;

use crate::components::*;
use crate::config::SimConfig;
use crate::spatial::{nearby_entities, SpatialGrid};

/// 捕食者追击猎物、进食、代谢与繁殖。
pub fn predator_behavior(
    config: Res<SimConfig>,
    grid: Res<SpatialGrid>,
    mut predators: Query<(Entity, &Position, &mut Velocity, &mut Energy, &mut Age), With<Predator>>,
    creatures: Query<(Entity, &Position, Option<&InLake>), With<Creature>>,
    mut deaths: EventWriter<DeathEvent>,
    mut spawns: EventWriter<SpawnPredator>,
) {
    let mut scratch = Vec::with_capacity(64);
    let mut killed_this_tick = HashSet::new();
    let mut rng = thread_rng();

    for (predator_entity, pos, mut velocity, mut energy, mut age) in predators.iter_mut() {
        age.ticks += 1;
        energy.current -= config.predator_metabolism_rate;

        // 饿死或老死。
        if energy.current <= 0.0 || age.ticks >= age.max_lifetime {
            deaths.send(DeathEvent {
                entity: predator_entity,
                reason: DeathReason::Starvation,
            });
            continue;
        }

        // 寻找最近猎物。
        let mut desired = Vec2::ZERO;
        scratch.clear();
        nearby_entities(&grid, pos.0, config.predator_perception_radius, &mut scratch);

        let mut nearest: Option<(Entity, f32)> = None;
        for &entity in scratch.iter() {
            if entity == predator_entity || killed_this_tick.contains(&entity) {
                continue;
            }
            if let Ok((creature_entity, creature_pos, in_lake)) = creatures.get(entity) {
                // 猎物在湖泊中时捕食者无法追踪/捕食。
                if in_lake.is_some() {
                    continue;
                }
                let dist = pos.0.distance(creature_pos.0);
                if dist <= config.predator_perception_radius {
                    match nearest {
                        None => nearest = Some((creature_entity, dist)),
                        Some((_, best_dist)) if dist < best_dist => {
                            nearest = Some((creature_entity, dist));
                        }
                        _ => {}
                    }
                }
            }
        }

        // 追击。
        let mut ate = false;
        if let Some((target_entity, dist)) = nearest {
            let target_pos = creatures.get(target_entity).unwrap().1;
            let direction = (target_pos.0 - pos.0).normalize_or_zero();
            desired += direction * config.seek_strength;

            // 足够近则吃掉猎物。
            const KILL_RADIUS: f32 = 10.0;
            if dist <= KILL_RADIUS {
                killed_this_tick.insert(target_entity);
                deaths.send(DeathEvent {
                    entity: target_entity,
                    reason: DeathReason::Predation,
                });
                energy.current = (energy.current + config.predator_energy_gain).min(energy.max);
                ate = true;
            }
        }

        // 没有猎物时随机游荡。
        if nearest.is_none() {
            let wander_angle = rng.gen_range(0.0..std::f32::consts::TAU);
            desired += Vec2::new(wander_angle.cos(), wander_angle.sin()) * config.wander_strength;
        }

        // 更新速度并限制最大速度。
        velocity.0 = (velocity.0 + desired).clamp_length_max(config.predator_speed);

        // 繁殖：能量充足时产生后代。
        if !ate
            && energy.current >= config.predator_reproduction_threshold
            && rng.gen_bool(0.02)
        {
            energy.current -= config.reproduction_cost;
            spawns.send(SpawnPredator {
                pos: pos.0,
                energy: config.reproduction_cost * 0.5,
            });
        }
    }
}

/// 处理捕食者繁殖事件。
pub fn handle_predator_spawns(
    mut commands: Commands,
    mut events: EventReader<SpawnPredator>,
    config: Res<SimConfig>,
) {
    let mut rng = thread_rng();
    for event in events.read() {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let offset = Vec2::new(angle.cos(), angle.sin()) * 8.0;
        commands.spawn((
            Predator,
            Position(event.pos + offset),
            Velocity(Vec2::new(angle.cos(), angle.sin()) * config.predator_speed),
            Energy {
                current: event.energy,
                max: 100.0,
            },
            Age {
                ticks: 0,
                max_lifetime: rng.gen_range(2000..3500),
            },
        ));
    }
}
