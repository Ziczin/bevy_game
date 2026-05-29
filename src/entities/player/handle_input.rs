use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::components::core::Velocity;
use crate::components::markers::Player;

use super::state::{MOVING_SPEED, PlayerStateHandler, PlayerState, PlayerAnimation};

pub fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player: Query<(
        &mut Velocity,
        &mut SpritesheetAnimation,
        &mut PlayerStateHandler,
        &PlayerAnimation,
    ), With<Player>>,
) {

    let mut direction = Vec2::ZERO;
    if keyboard.pressed(KeyCode::ArrowLeft)  { direction.x -= 1.0; }
    if keyboard.pressed(KeyCode::ArrowRight) { direction.x += 1.0; }
    if keyboard.pressed(KeyCode::ArrowUp)    { direction.y += 1.0; }
    if keyboard.pressed(KeyCode::ArrowDown)  { direction.y -= 1.0; }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
    }

    for (mut velocity, mut sprite_sheet, mut state_handler, animation) in &mut player {
        velocity.x = direction.x * MOVING_SPEED;
        velocity.y = direction.y * MOVING_SPEED;
        if velocity.length() > 0.0 {
            if state_handler.set(PlayerState::Walk) {
                sprite_sheet.switch(animation.walk.clone());
            }
        }
        else if sprite_sheet.progress.frame == 0 {
            if state_handler.set(PlayerState::Idle) {
                sprite_sheet.switch(animation.idle.clone());
            }
        }
    }
}