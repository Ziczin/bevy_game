// src/entities/player/state.rs
use crate::core::config::from_toml;
use crate::core::macros::bevy_alias::component_from_type;
use crate::core::macros::bevy_custom::animation_states;
use crate::core::macros::state_machine::behavior_states;
use bitflags;
use bevy::prelude::*;

from_toml!("config/player.toml", [
    MOVING_SPEED: f32,
    CAMERA_FOLLOW_SMOOTHNESS: f32,
    PLAYER_COLLIDER_HALF_WIDTH: i32,
    PLAYER_COLLIDER_HALF_HEIGHT: i32,
    PLAYER_COLLIDER_OFFSET_X: i32,
    PLAYER_COLLIDER_OFFSET_Y: i32,
]);

behavior_states!(Player { Idle, Walk });
animation_states!(Player { idle, walk });

component_from_type!(MovingDirection, Vec2);

bitflags::bitflags! {
    #[derive(Component, Clone, Copy, PartialEq, Default)]
    pub struct PlayerLogicFlags: u8 {
        const CanMove = 1 << 0;
        const CanStop = 1 << 1;
    }
}