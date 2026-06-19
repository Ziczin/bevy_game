// src/entities/player/handle_input.rs
use bevy::prelude::*;

use crate::{components::markers::Player, core::debug_log::DebugLogBuffer};
use super::state::{MovingDirection, PlayerLogicFlags};

pub fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut debug_log: ResMut<DebugLogBuffer>,
    mut player: Query<(&mut MovingDirection, &PlayerLogicFlags), With<Player>>,
) {
    let mut direction = Vec2::ZERO;
    if keyboard.pressed(KeyCode::ArrowLeft)  { direction.x -= 1.0; }
    if keyboard.pressed(KeyCode::ArrowRight) { direction.x += 1.0; }
    if keyboard.pressed(KeyCode::ArrowUp)    { direction.y += 1.0; }
    if keyboard.pressed(KeyCode::ArrowDown)  { direction.y -= 1.0; }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
    }

    for (mut player_direction, logic_flags) in &mut player {
        if !logic_flags.contains(PlayerLogicFlags::CanMove) {
            if player_direction.0 != direction {
                debug_log.add(format!("Player input: direction changed to ({:.2}, {:.2})", direction.x, direction.y));
            }
            *player_direction = direction.into();
        }
    }
}