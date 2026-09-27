use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::components::*;
use crate::config::SimConfig;

pub fn energy_metabolism(
    config: Res<SimConfig>,
    mut creatures: Query<(Entity, &mut Energy), With<Creature>>,
    mut deaths: EventWriter<DeathEvent>,
) {
    for (entity, mut energy) in creatures.iter_mut() {
        energy.current -= config.metabolism_rate;
        if energy.current <= 0.0 {
            deaths.send(DeathEvent {
                entity,
                reason: DeathReason::Starvation,
            });
        }
    }
}

pub fn age_creatures(
    mut creatures: Query<(Entity, &mut Age), With<Creature>>,
    mut deaths: EventWriter<DeathEvent>,
) {
    for (entity, mut age) in creatures.iter_mut() {
        age.ticks += 1;
        if age.ticks >= age.max_lifetime {
            deaths.send(DeathEvent {
                entity,
                reason: DeathReason::OldAge,
            });
        }
    }
}

pub fn reproduce(
    config: Res<SimConfig>,
    mut creatures: Query<(Entity, &Position, &mut Energy, &Species, &mut ReproductionCooldown), With<Creature>>,
    mut spawns: EventWriter<SpawnCreature>,
) {
    let mut rng = thread_rng();

    for (_, pos, mut energy, species, mut cooldown) in creatures.iter_mut() {
        cooldown.timer -= 1.0;

        if energy.current >= config.reproduction_threshold && cooldown.timer <= 0.0 {
            energy.current -= config.reproduction_cost;
            cooldown.timer = config.reproduction_cooldown;

            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let offset = Vec2::new(angle.cos(), angle.sin()) * 8.0;

            spawns.send(SpawnCreature {
                pos: pos.0 + offset,
                species_id: species.id,
                energy: config.reproduction_cost * 0.5,
            });
        }
    }
}

pub fn handle_deaths(
    mut commands: Commands,
    mut deaths: EventReader<DeathEvent>,
) {
    for event in deaths.read() {
        if let Some(mut entity) = commands.get_entity(event.entity) {
            entity.despawn();
        }
    }
}
