use bevy::prelude::*;

use crate::components::*;
use crate::config::SimConfig;

/// 检测生物是否进入或离开湖泊，并处理湖中的生存逻辑。
///
/// 生物在湖中时可以躲避捕食者追踪，但：
/// - 能量消耗是陆地的 2 倍
/// - 最多停留 `lake_max_time` 秒，否则会淹死
pub fn update_lake_state(
    config: Res<SimConfig>,
    lakes: Query<&Position, With<Lake>>,
    mut creatures: Query<(Entity, &Position, &mut Energy, Option<&mut InLake>), With<Creature>>,
    mut deaths: EventWriter<DeathEvent>,
) {
    let lake_radius_sq = config.lake_radius * config.lake_radius;
    let tick_dt = 1.0 / config.ticks_per_second;

    for (entity, pos, mut energy, in_lake) in creatures.iter_mut() {
        let inside_any_lake = lakes.iter().any(|lake_pos| {
            pos.0.distance_squared(lake_pos.0) <= lake_radius_sq
        });

        match (inside_any_lake, in_lake) {
            (true, None) => {
                // 刚进入湖泊：附加 InLake 组件。
                // 注意：这里不能直接插入组件，因为 commands 会延迟执行。
                // 为了避免同一系统内读写冲突，我们用 Event 或延迟处理。
                // 简化方案：在 apply_lake_transitions 中统一处理。
            }
            (true, Some(mut in_lake)) => {
                // 已在湖中：消耗双倍能量，倒计时减少。
                let cost = config.metabolism_rate * config.lake_energy_cost_multiplier;
                energy.current -= cost;
                in_lake.time_remaining -= tick_dt;

                if in_lake.time_remaining <= 0.0 {
                    deaths.send(DeathEvent {
                        entity,
                        reason: DeathReason::Drowning,
                    });
                } else if energy.current <= 0.0 {
                    deaths.send(DeathEvent {
                        entity,
                        reason: DeathReason::Starvation,
                    });
                }
            }
            (false, Some(_)) => {
                // 离开湖泊：移除 InLake 组件，同样延迟处理。
            }
            (false, None) => {}
        }
    }
}

/// 处理进入/离开湖泊的组件增删。
///
/// 由于 Bevy ECS 在同一系统内不能安全地边查询边插入/删除组件，
/// 所以将状态检测和组件变更分成两个系统。
pub fn apply_lake_transitions(
    mut commands: Commands,
    config: Res<SimConfig>,
    lakes: Query<&Position, With<Lake>>,
    creatures: Query<(Entity, &Position, Option<&InLake>), With<Creature>>,
) {
    let lake_radius_sq = config.lake_radius * config.lake_radius;

    for (entity, pos, in_lake) in creatures.iter() {
        let inside_any_lake = lakes.iter().any(|lake_pos| {
            pos.0.distance_squared(lake_pos.0) <= lake_radius_sq
        });

        match (inside_any_lake, in_lake.is_some()) {
            (true, false) => {
                commands.entity(entity).insert(InLake {
                    time_remaining: config.lake_max_time,
                });
            }
            (false, true) => {
                commands.entity(entity).remove::<InLake>();
            }
            _ => {}
        }
    }
}
