use crate::macros::{behavior_states, component};

//Create <Name>State Enum & <Name>StateHandler Component
behavior_states!(Player { Idle, Walk });
behavior_states!(RedSlime { Idle, Chase });

component!(FollowPlayer {
    smoothness: f32
});