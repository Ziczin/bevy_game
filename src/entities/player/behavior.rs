use bevy::prelude::*;
use avian2d::prelude::*;

use crate::{components::markers::Player, entities::player::state::PlayerLogicFlags};
use super::state::{MOVING_SPEED, MovingDirection};

pub fn behavior(
    mut player: Query<(
        &mut LinearVelocity,
        &PlayerLogicFlags,
        &MovingDirection,
    ), With<Player>>,
) {
    let speed = *MOVING_SPEED;

    for (
        mut velocity,
        logic_flags,
        direction
    ) in &mut player {
        if logic_flags.contains(PlayerLogicFlags::CanMove) {
            velocity.x = direction.x * speed;
            velocity.y = direction.y * speed;
        } else {
            velocity.x = 0.0;
            velocity.y = 0.0;
        }
    }
}