use bevy::prelude::*;
use avian2d::prelude::*;
use crate::{core::profiling::{ProfilingBuffer, profile_scope}, entities::red_slime::state::MovingDirection};
use super::state::{
    RedSlimeStateHandler, RedSlimeState, RedSlimeLogicFlags,
    WALK_SPEED
};

pub fn behavior(
    profiling: Res<ProfilingBuffer>,
    mut slime_query: Query<(
        &RedSlimeStateHandler,
        &RedSlimeLogicFlags,
        &MovingDirection,
        &mut LinearVelocity,
    )>,
) {
    profile_scope!(&profiling, "entities::red_slime::behavior::behavior", &["red_slime", "behavior", "movement", "physics"]);
    let speed = *WALK_SPEED;
    for (
        state_handler,
        logic_flags,
        direction,
        mut velocity,
    ) in &mut slime_query {
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