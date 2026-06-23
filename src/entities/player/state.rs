// FILE: src/entities/player/state.rs
use crate::core::config::from_toml;
use crate::core::dto::{AnimationConfig, SpriteSheetConfig};
use crate::core::macros::bevy_alias::component_from_type;
use crate::core::macros::bevy_custom::animation_states;
use crate::core::macros::state_machine::behavior_states;
use bitflags;
use bevy::prelude::*;

from_toml!("config/player.toml", [
    MOVING_SPEED: f32 = "moving.speed",
    PLAYER_COLLIDER_HALF_WIDTH: i32 = "collider.half_width",
    PLAYER_COLLIDER_HALF_HEIGHT: i32 = "collider.half_height",
    PLAYER_COLLIDER_OFFSET_X: i32 = "collider.offset_x",
    PLAYER_COLLIDER_OFFSET_Y: i32 = "collider.offset_y",
    SPRITESHEET: SpriteSheetConfig = "spritesheet",
    ANIMATIONS: Vec<AnimationConfig> = "animations",
    HEALTH_BAR_OFFSET_Y: f32 = "health.bar_offset_y",
    MANA_BAR_OFFSET_Y: f32 = "mana.bar_offset_y",
]);

from_toml!("config/global/display.toml", [
    CAMERA_FOLLOW_SMOOTHNESS: f32 = "camera.follow_smoothness",
    TILE_SIZE: f32 = "display.tile_size",
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