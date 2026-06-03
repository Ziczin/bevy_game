use crate::macros::{behavior_states, animation_states};

behavior_states!(RedSlime { Idle, Walk });
animation_states!(RedSlime { idle, walk } );

pub const WALK_SPEED: f32 = 100.0;
pub const WALK_DISTANCE_START: f32 = 1000.0;
pub const WALK_DISTANCE_END: f32 = 150.0;