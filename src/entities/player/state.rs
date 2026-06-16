use crate::{animation_states, behavior_states};

pub const MOVING_SPEED: f32 = 50.0;
pub const CAMERA_FOLLOW_SMOOTHNESS: f32 = 0.99;

pub const PLAYER_COLLIDER_HALF_WIDTH: i32 = 4;
pub const PLAYER_COLLIDER_HALF_HEIGHT: i32 = 4;
pub const PLAYER_COLLIDER_OFFSET_X: i32 = 0;
pub const PLAYER_COLLIDER_OFFSET_Y: i32 = -4;

behavior_states!(Player { Idle, Walk });
animation_states!(Player { idle, walk });