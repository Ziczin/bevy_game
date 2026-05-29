use bevy::prelude::*;

use crate::components::markers::Player;
use crate::components::core::Velocity;

use super::state::{RedSlimeStateHandler, RedSlimeState, WALK_SPEED};

pub fn behavior(
    player_query: Query<&Transform, With<Player>>,
    mut slime: Query<(&mut Velocity, &mut RedSlimeStateHandler, &Transform)>,
) {
    let Ok(player) = player_query.single() else { return; };

    for (mut velocity, state_handler, transform) in &mut slime {
        match state_handler.get() {
            RedSlimeState::Idle => {
                velocity.x = 0.0;
                velocity.y = 0.0;
            }
            RedSlimeState::Walk => {
                let to_player = player.translation - transform.translation;
                let direction = to_player.normalize_or_zero();
                let step = direction * WALK_SPEED;
                velocity.x = step.x;
                velocity.y = step.y;
            }
        }
    }
}