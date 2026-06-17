use crate::core::macros::bevy_alias::component_from_type;
use crate::core::macros::bevy_custom::animation_states;
use crate::core::macros::state_machine::behavior_states;
use bitflags;
use bevy::prelude::*;

pub const MOVING_SPEED: f32 = 80.0;
pub const CAMERA_FOLLOW_SMOOTHNESS: f32 = 0.99;

pub const PLAYER_COLLIDER_HALF_WIDTH: i32 = 4;
pub const PLAYER_COLLIDER_HALF_HEIGHT: i32 = 4;
pub const PLAYER_COLLIDER_OFFSET_X: i32 = 0;
pub const PLAYER_COLLIDER_OFFSET_Y: i32 = -4;

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