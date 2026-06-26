use super::state::{MovingDirection, PlayerLogicFlags};
use crate::{
    components::markers::Player,
    core::debug_log::DebugLogBuffer,
    core::profiling::{ProfileScope, ProfilingBuffer},
};
use bevy::prelude::*;

pub fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    profiling: Res<ProfilingBuffer>,
    mut player: Query<(&mut MovingDirection, &PlayerLogicFlags), With<Player>>,
    mut debug_log: ResMut<DebugLogBuffer>,
) {
    let _scope = ProfileScope::new(
        &*profiling,
        "entities::player::handle_input::handle_input",
        &["player", "input", "keyboard"],
    );

    let mut direction = Vec2::ZERO;
    if keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
    }

    for (mut player_direction, logic_flags) in &mut player {
        if !logic_flags.contains(PlayerLogicFlags::CanMove) {
            if player_direction.0 != direction {
                debug_log.add(
                    &["player"],
                    format!(
                        "Player input: direction changed to ({:.2}, {:.2})",
                        direction.x, direction.y
                    ),
                );
            }
            *player_direction = direction.into();
        }
    }
}
