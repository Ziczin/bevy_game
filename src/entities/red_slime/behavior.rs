use super::state::{
    RedSlimeLogicFlags, RedSlimeState, RedSlimeStateHandler, WALK_SPEED,
};
use crate::{
    core::profiling::{ProfileScope, ProfilingBuffer},
    entities::red_slime::state::MovingDirection,
};
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn behavior(
    profiling: Res<ProfilingBuffer>,
    mut slime_query: Query<(
        &mut LinearVelocity,
        &RedSlimeStateHandler,
        &RedSlimeLogicFlags,
        &MovingDirection,
    )>,
) {
    let _scope = ProfileScope::new(
        &*profiling,
        "entities::red_slime::behavior::behavior",
        &["red_slime", "behavior", "movement", "physics"],
    );

    let speed = *WALK_SPEED;

    for (mut velocity, state_handler, logic_flags, direction) in
        &mut slime_query
    {
        match state_handler.get() {
            RedSlimeState::Idle => {
                velocity.x = 0.0;
                velocity.y = 0.0;
            }
            RedSlimeState::Walk => {
                if logic_flags.contains(RedSlimeLogicFlags::CanMove) {
                    velocity.x = direction.x * speed;
                    velocity.y = direction.y * speed;
                } else {
                    velocity.x = 0.0;
                    velocity.y = 0.0;
                }
            }
        }
    }
}
