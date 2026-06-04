use crate::macros::{animation_states, behavior_states};

pub const WALK_SPEED: f32 = 100.0;
pub const WALK_DISTANCE_START: f32 = 1000.0;
pub const WALK_DISTANCE_END: f32 = 150.0;
pub const WAYPOINT_ARRIVAL_THRESHOLD: f32 = 16.0;

pub const PATHFINDER_UPDATE_INTERVAL: f32 = 0.5;
pub const SPRITE_SIZE_MULTIPLIER_X: i32 = 8;
pub const SPRITE_SIZE_MULTIPLIER_Y: i32 = 7;
pub const COLLIDER_PADDING: f32 = 2.0;
pub const COLLIDER_OFFSET_X: i32 = 0;
pub const COLLIDER_OFFSET_Y: i32 = -8;

pub const EXPECTED_IDLE_FRAME: usize = 0;

behavior_states!(RedSlime { Idle, Walk });
animation_states!(RedSlime { idle, walk });