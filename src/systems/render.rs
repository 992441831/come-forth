use bevy::prelude::*;

use crate::components::{CommunicationBuffer, Creature, Energy, Food, Position, Predator, SimTick, Species};

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
    creatures: Query<(&Position, &Species, &Energy, &CommunicationBuffer), With<Creature>>,
) {
    for (pos, species, energy, buffer) in creatures.iter() {
        let radius = 2.5 + (energy.current / energy.max) * 2.5;
        let alpha = if buffer.last_message.is_some() { 1.0 } else { 0.65 };
        let base = species.color.to_srgba();
        let color = Color::srgba(base.red, base.green, base.blue, alpha);
        gizmos.circle_2d(pos.0, radius, color);
    }
}

pub fn render_food(
    mut gizmos: Gizmos,
    food: Query<&Position, With<Food>>,
) {
    for pos in food.iter() {
        gizmos.rect_2d(pos.0, 0.0, Vec2::splat(4.0), Color::srgb(0.2, 0.85, 0.25));
    }
}

pub fn render_predators(
    mut gizmos: Gizmos,
    predators: Query<&Position, With<Predator>>,
) {
    for pos in predators.iter() {
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
