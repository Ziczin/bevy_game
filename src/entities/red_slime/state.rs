// src/entities/red_slime/state.rs
use bevy::math::Vec2;
use bevy::prelude::Component;

use crate::core::config::from_toml;
use crate::core::dto::{AnimationConfig, SpriteSheetConfig};
use crate::core::macros::bevy_custom::animation_states;
use crate::core::macros::state_machine::behavior_states;
use crate::core::macros::bevy_alias::component_from_type;

from_toml!("config/red_slime.toml", [
    WALK_SPEED: f32 = "moving.walk_speed",
    WALK_DISTANCE_START: f32 = "moving.walk_distance_start",
    WALK_DISTANCE_END: f32 = "moving.walk_distance_end",
    WAYPOINT_ARRIVAL_THRESHOLD: f32 = "moving.waypoint_arrival_threshold",
    PATHFINDER_UPDATE_INTERVAL: f32 = "moving.pathfinder_update_interval",
    SPRITE_SIZE_MULTIPLIER_X: i32 = "sprite.size_multiplier_x",
    SPRITE_SIZE_MULTIPLIER_Y: i32 = "sprite.size_multiplier_y",
    COLLIDER_PADDING: f32 = "collider.padding",
    COLLIDER_OFFSET_X: i32 = "collider.offset_x",
    COLLIDER_OFFSET_Y: i32 = "collider.offset_y",
    EXPECTED_IDLE_FRAME: usize = "collider.expected_idle_frame",
    SPRITESHEET: SpriteSheetConfig = "spritesheet",
    ANIMATIONS: Vec<AnimationConfig> = "animations",
]);

behavior_states!(RedSlime { Idle, Walk });
animation_states!(RedSlime { idle, walk });

component_from_type!(MovingDirection, Vec2);

bitflags::bitflags! {
    #[derive(Component, Clone, Copy, PartialEq, Default)]
    pub struct RedSlimeLogicFlags: u8 {
        const CanMove = 1 << 0;
    }
}