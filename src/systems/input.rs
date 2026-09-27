use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;

pub fn camera_zoom_pan(
    mut projection_query: Query<&mut OrthographicProjection, With<Camera2d>>,
    mut transform_query: Query<&mut Transform, With<Camera2d>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut scroll_events: EventReader<MouseWheel>,
    time: Res<Time>,
) {
    let mut projection = projection_query.single_mut();
    for event in scroll_events.read() {
        projection.scale *= 1.0 - event.y * 0.1;
        projection.scale = projection.scale.clamp(0.05, 10.0);
    }

    let mut transform = transform_query.single_mut();
    let speed = 600.0 * projection.scale * time.delta_seconds();

    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        transform.translation.y += speed;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        transform.translation.y -= speed;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= speed;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        transform.translation.x += speed;
    }
}
