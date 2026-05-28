use bevy::prelude::*;
use crate::components::core::Velocity;
use crate::components::markers::Player;
pub fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    const SPEED: f32 = 200.0;

    let mut direction = Vec2::ZERO;
    if keyboard.pressed(KeyCode::ArrowLeft)  { direction.x -= 1.0; }
    if keyboard.pressed(KeyCode::ArrowRight) { direction.x += 1.0; }
    if keyboard.pressed(KeyCode::ArrowUp)    { direction.y += 1.0; }
    if keyboard.pressed(KeyCode::ArrowDown)  { direction.y -= 1.0; }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
    }

    for mut vel in query.iter_mut() {
        vel.x = direction.x * SPEED;
        vel.y = direction.y * SPEED;
    }
}