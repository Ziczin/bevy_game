// src/entities/player/brain.rs
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
        let was_can_move = logic_flags.contains(PlayerLogicFlags::CanMove);
        
        logic_flags.set(
            PlayerLogicFlags::CanMove,
            matches!(sprite_sheet.progress.frame, 3|4|5|6|7|8|9)
        );
        
        let is_can_move = logic_flags.contains(PlayerLogicFlags::CanMove);
        if was_can_move != is_can_move {
            debug_log.add(format!("Can move: {}", is_can_move));
        }
        
        let current_state = state_handler.get();
        let vel_length = velocity.length();
        
        if vel_length > 0.0 {
            if state_handler.set(PlayerState::Walk) {
                debug_log.add(format!("Player state: {:?} -> Walk (velocity: {:.2})", current_state, vel_length));
                sprite_sheet.switch(animation.walk.clone());
            }
        } else if sprite_sheet.progress.frame == 0 {
            if state_handler.set(PlayerState::Idle) {
                debug_log.add(format!("Player state: {:?} -> Idle (velocity: {:.2})", current_state, vel_length));
                sprite_sheet.switch(animation.idle.clone());
            }
        }
    }
}