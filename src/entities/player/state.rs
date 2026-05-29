use crate::macros::{animation_states, behavior_states};

behavior_states!(Player { Idle, Walk });
animation_states!(Player { idle, walk } );

pub const MOVING_SPEED: f32 = 200.0;