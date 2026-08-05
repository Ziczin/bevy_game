use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use crate::{components::markers::Player, core::debug_log::{DebugLogBuffer, debug_log}, entities::player::state::PlayerState};
use crate::core::profiling::{ProfilingBuffer, profile_scope};
use super::state::{PlayerStateHandler, PlayerAnimation, MovingDirection, PlayerLogicFlags};

pub fn brain(
    profiling: Res<ProfilingBuffer>,
    mut player: Query<(
        &PlayerAnimation,
        &MovingDirection,
        &mut SpritesheetAnimation,
        &mut PlayerStateHandler,
        &mut PlayerLogicFlags,
    ), With<Player>>,
    mut debug_log: ResMut<DebugLogBuffer>,
) {
    profile_scope!(&profiling, "entities::player::brain::brain", &["player", "brain", "state", "animation"]);
    for (
        animation,
        velocity,
        mut sprite_sheet,
        mut state_handler,
        mut logic_flags,
    ) in &mut player {
        let was_can_move = logic_flags.contains(PlayerLogicFlags::CanMove);
        logic_flags.set(
            PlayerLogicFlags::CanMove,
            matches!(sprite_sheet.progress.frame, 3..=9)
        );
        let is_can_move = logic_flags.contains(PlayerLogicFlags::CanMove);
        if was_can_move != is_can_move {
            debug_log!(&mut debug_log, &["player"], "Can move: {}", is_can_move);
        }
        let current_state = state_handler.get();
        let vel_length = velocity.length();
        if vel_length > 0.0 {
            if state_handler.set(PlayerState::Walk) {
                debug_log!(&mut debug_log, &["player"], "Player state: {:?} -> Walk (velocity: {:.2})", current_state, vel_length);
                sprite_sheet.switch(animation.walk.clone());
            }
        } else if sprite_sheet.progress.frame == 0 && state_handler.set(PlayerState::Idle) {
            debug_log!(&mut debug_log, &["player"], "Player state: {:?} -> Idle (velocity: {:.2})", current_state, vel_length);
            sprite_sheet.switch(animation.idle.clone());
        }
    }
}