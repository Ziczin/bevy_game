use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::{components::markers::Player, core::debug_log::DebugLogBuffer, entities::player::state::PlayerState};
use super::state::{PlayerStateHandler, PlayerAnimation, MovingDirection, PlayerLogicFlags};

pub fn brain(
    mut debug_log: ResMut<DebugLogBuffer>,
    mut player: Query<(
        &mut SpritesheetAnimation,
        &mut PlayerStateHandler,
        &mut PlayerLogicFlags,
        &PlayerAnimation,
        &MovingDirection,
    ), With<Player>>,
) {
    for (
        mut sprite_sheet,
        mut state_handler,
        mut logic_flags,
        animation,
        velocity
    ) in &mut player {
        logic_flags.set(
            PlayerLogicFlags::CanMove,
            matches!(sprite_sheet.progress.frame, 4|5|6|7)
        );
        debug_log.add(format!("Can move: {}", logic_flags.contains(PlayerLogicFlags::CanMove)));
        if velocity.length() > 0.0 {
            logic_flags.remove(PlayerLogicFlags::CanStop);
            if state_handler.set(PlayerState::Walk) {
                sprite_sheet.switch(animation.walk.clone());
            }
        } else if sprite_sheet.progress.frame == 0 {
            logic_flags.insert(PlayerLogicFlags::CanStop);
            if state_handler.set(PlayerState::Idle) {
                sprite_sheet.switch(animation.idle.clone());
            }
        }
    }
}