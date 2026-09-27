use bevy::prelude::*;

use crate::components::{CommunicationBuffer, Creature, Energy, Fleeing, Food, InLake, Lake, Position, Predator, SimTick, Species};
use crate::config::SimConfig;

pub fn setup_ui(mut commands: Commands) {
    commands.spawn(
        TextBundle::from_section(
            "Creatures: 0 | Food: 0",
            TextStyle {
                font: Handle::default(),
                font_size: 18.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
}

pub fn render_creatures(
    mut gizmos: Gizmos,
    creatures: Query<(&Position, &Species, &Energy, &CommunicationBuffer, Option<&Fleeing>, Option<&InLake>), With<Creature>>,
    camera_query: Query<(&Camera, &GlobalTransform, &OrthographicProjection)>,
) {
    let Ok((_camera, camera_transform, projection)) = camera_query.get_single() else {
        return;
    };

    // 计算当前相机视野的包围盒，额外留出一点边距避免物体在边缘突然消失。
    let camera_pos = camera_transform.translation().xy();
    let half_size = Vec2::new(projection.area.width(), projection.area.height()) * 0.5;
    let margin = 10.0;
    let min = camera_pos - half_size - Vec2::splat(margin);
    let max = camera_pos + half_size + Vec2::splat(margin);

    for (pos, species, energy, buffer, fleeing, in_lake) in creatures.iter() {
        // 跳过屏幕外的生物，大幅减少 GPU 绘制压力。
        if pos.0.x < min.x || pos.0.x > max.x || pos.0.y < min.y || pos.0.y > max.y {
            continue;
        }

        let radius = 2.5 + (energy.current / energy.max) * 2.5;
        let base = species.color.to_srgba();

        // 逃跑中的生物用白色高亮显示，让“恐慌波”肉眼可见。
        // 在湖泊中的生物用蓝色调显示，便于观察避难行为。
        let color = if fleeing.is_some() {
            Color::srgba(1.0, 1.0, 1.0, 0.95)
        } else if in_lake.is_some() {
            Color::srgba(base.red * 0.5, base.green * 0.5 + 0.3, base.blue * 0.5 + 0.5, 0.9)
        } else {
            let alpha = if buffer.last_message.is_some() { 1.0 } else { 0.65 };
            Color::srgba(base.red, base.green, base.blue, alpha)
        };
        gizmos.circle_2d(pos.0, radius, color);
    }
}

pub fn render_lakes(
    mut gizmos: Gizmos,
    lakes: Query<&Position, With<Lake>>,
    camera_query: Query<(&Camera, &GlobalTransform, &OrthographicProjection)>,
    config: Res<SimConfig>,
) {
    let Ok((_camera, camera_transform, projection)) = camera_query.get_single() else {
        return;
    };

    let camera_pos = camera_transform.translation().xy();
    let half_size = Vec2::new(projection.area.width(), projection.area.height()) * 0.5;
    let margin = config.lake_radius;
    let min = camera_pos - half_size - Vec2::splat(margin);
    let max = camera_pos + half_size + Vec2::splat(margin);

    for pos in lakes.iter() {
        // 跳过屏幕外的湖泊。
        if pos.0.x < min.x || pos.0.x > max.x || pos.0.y < min.y || pos.0.y > max.y {
            continue;
        }

        // 湖泊：半透明蓝色圆形区域 + 边界线。
        gizmos.circle_2d(pos.0, config.lake_radius, Color::srgba(0.15, 0.45, 0.85, 0.25));
        gizmos.circle_2d(pos.0, config.lake_radius, Color::srgba(0.3, 0.6, 0.95, 0.4));
    }
}

fn compute_view_bounds(
    camera_transform: &GlobalTransform,
    projection: &OrthographicProjection,
    margin: f32,
) -> (Vec2, Vec2) {
    let camera_pos = camera_transform.translation().xy();
    let half_size = Vec2::new(projection.area.width(), projection.area.height()) * 0.5;
    let min = camera_pos - half_size - Vec2::splat(margin);
    let max = camera_pos + half_size + Vec2::splat(margin);
    (min, max)
}

pub fn render_food(
    mut gizmos: Gizmos,
    food: Query<&Position, With<Food>>,
    camera_query: Query<(&Camera, &GlobalTransform, &OrthographicProjection)>,
) {
    let Ok((_, camera_transform, projection)) = camera_query.get_single() else {
        return;
    };
    let (min, max) = compute_view_bounds(camera_transform, projection, 10.0);

    for pos in food.iter() {
        if pos.0.x < min.x || pos.0.x > max.x || pos.0.y < min.y || pos.0.y > max.y {
            continue;
        }
        gizmos.rect_2d(pos.0, 0.0, Vec2::splat(4.0), Color::srgb(0.2, 0.85, 0.25));
    }
}

pub fn render_predators(
    mut gizmos: Gizmos,
    predators: Query<&Position, With<Predator>>,
    camera_query: Query<(&Camera, &GlobalTransform, &OrthographicProjection)>,
) {
    let Ok((_, camera_transform, projection)) = camera_query.get_single() else {
        return;
    };
    let (min, max) = compute_view_bounds(camera_transform, projection, 10.0);

    for pos in predators.iter() {
        if pos.0.x < min.x || pos.0.x > max.x || pos.0.y < min.y || pos.0.y > max.y {
            continue;
        }

        // 捕食者用红色三角形象征，比生物大一些，便于识别。
        let size = 7.0;
        let p1 = pos.0 + Vec2::new(0.0, size);
        let p2 = pos.0 + Vec2::new(-size * 0.85, -size * 0.6);
        let p3 = pos.0 + Vec2::new(size * 0.85, -size * 0.6);
        gizmos.line_2d(p1, p2, Color::srgb(0.95, 0.2, 0.2));
        gizmos.line_2d(p2, p3, Color::srgb(0.95, 0.2, 0.2));
        gizmos.line_2d(p3, p1, Color::srgb(0.95, 0.2, 0.2));
    }
}

pub fn update_ui_stats(
    creatures: Query<(), With<Creature>>,
    food: Query<(), With<Food>>,
    predators: Query<(), With<Predator>>,
    tick: Res<SimTick>,
    mut text_query: Query<&mut Text>,
) {
    let creature_count = creatures.iter().count();
    let food_count = food.iter().count();
    let predator_count = predators.iter().count();

    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!(
            "Tick: {} | Creatures: {} | Food: {} | Predators: {} | WASD pan, wheel zoom | Chart: blue=creatures, green=food",
            tick.count, creature_count, food_count, predator_count
        );
    }
}
