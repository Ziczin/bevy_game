use bevy::prelude::*;
use avian2d::prelude::*;
use crate::{components::markers::Player, core::profiling::{ProfilingBuffer, ProfileScope}, entities::player::state::PlayerLogicFlags};
use super::state::{MOVING_SPEED, MovingDirection};

pub fn behavior(
    profiling: Res<ProfilingBuffer>,
    mut player: Query<(
        &mut LinearVelocity,
        &PlayerLogicFlags,
        &MovingDirection,
    ), With<Player>>,
) {
    let _scope = ProfileScope::new(&profiling, "entities::player::behavior::behavior", &["player", "behavior", "movement", "physics"]);
    
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