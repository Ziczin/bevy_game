use bevy::prelude::*;
use crate::{components::markers::Player, core::debug_log::{DebugLogBuffer, debug_log}, core::profiling::{ProfilingBuffer, profile_scope}};
use super::state::{MovingDirection, PlayerLogicFlags};

pub fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    profiling: Res<ProfilingBuffer>,
    mut player: Query<(&PlayerLogicFlags, &mut MovingDirection), With<Player>>,
    mut debug_log: ResMut<DebugLogBuffer>,
) {
    profile_scope!(&profiling, "entities::player::handle_input::handle_input", &["player", "input", "keyboard"]);
    let mut direction = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyW) { direction.y += 1.0; }
    if keyboard.pressed(KeyCode::KeyS) { direction.y -= 1.0; }
    if keyboard.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
    if keyboard.pressed(KeyCode::KeyD) { direction.x += 1.0; }
    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
    }
    for (logic_flags, mut player_direction) in &mut player {
        if !logic_flags.contains(PlayerLogicFlags::CanMove) {
            if player_direction.0 != direction {
                debug_log!(&mut debug_log, &["player"], "Player input: direction changed to ({:.2}, {:.2})", direction.x, direction.y);
            }
            *player_direction = direction.into();
        }
    }
}
