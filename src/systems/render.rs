use bevy::prelude::*;

use crate::components::{CommunicationBuffer, Creature, Energy, Food, Position, Species};

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

pub fn update_ui_stats(
    creatures: Query<(), With<Creature>>,
    food: Query<(), With<Food>>,
    mut text_query: Query<&mut Text>,
) {
    let creature_count = creatures.iter().count();
    let food_count = food.iter().count();

    for mut text in text_query.iter_mut() {
        text.sections[0].value =
            format!("Creatures: {creature_count} | Food: {food_count} | WASD pan, wheel zoom");
    }
}
