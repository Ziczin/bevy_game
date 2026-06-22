// src/entities/red_slime/behavior.rs
use bevy::prelude::*;
use avian2d::prelude::*;

use crate::{entities::red_slime::state::MovingDirection};
use super::state::{
    RedSlimeStateHandler, RedSlimeState, RedSlimeLogicFlags,
    WALK_SPEED
};

pub fn behavior(
    mut slime_query: Query<(
        &mut LinearVelocity,
        &RedSlimeStateHandler,
        &RedSlimeLogicFlags,
        &MovingDirection,
    )>,
) {
    let speed = *WALK_SPEED;

    for (
        mut velocity,
        state_handler,
        logic_flags,
        direction,
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