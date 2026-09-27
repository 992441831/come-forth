use bevy::prelude::*;

use crate::components::{Creature, Position, Velocity};
use crate::config::SimConfig;

pub fn resolve_movement(
    config: Res<SimConfig>,
    mut creatures: Query<(&mut Position, &mut Velocity), With<Creature>>,
) {
    let half = config.world_half_size;

    for (mut pos, mut vel) in creatures.iter_mut() {
        pos.0 += vel.0;

        // Clamp to world bounds and kill the velocity component that would escape.
        if pos.0.x < -half.x {
            pos.0.x = -half.x;
            vel.0.x = vel.0.x.abs();
        } else if pos.0.x > half.x {
            pos.0.x = half.x;
            vel.0.x = -vel.0.x.abs();
        }

        if pos.0.y < -half.y {
            pos.0.y = -half.y;
            vel.0.y = vel.0.y.abs();
        } else if pos.0.y > half.y {
            pos.0.y = half.y;
            vel.0.y = -vel.0.y.abs();
        }
    }
}
