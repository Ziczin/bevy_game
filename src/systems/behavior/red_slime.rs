use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::resources::behavior::RedSlimeConfig;

use crate::components::behavior::{FollowPlayer, RedSlimeState, RedSlimeStateHandler};
use crate::components::markers::Player;
use crate::components::core::Velocity;
use crate::components::animations::RedSlimeAnimations;

pub fn brain(
    mut slime: Query<(
        &mut RedSlimeStateHandler,
        &mut SpritesheetAnimation,
        &RedSlimeAnimations,
        &Transform
    )>,
    config: Res<RedSlimeConfig>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player) = player_query.single() else { return; };

    for (mut state_handler, mut sprite_sheet, animations, transform) in &mut slime {
        let to_player = player.translation - transform.translation;
        let distance = to_player.length();
        let new_state: RedSlimeState;

        if config.chase_distance_end < distance && distance < config.chase_distance_start {
            new_state = RedSlimeState::Chase;
        } else {
            new_state = RedSlimeState::Idle;
        };
        
        if state_handler.get() != new_state {
            state_handler.set(new_state);
            match new_state {
                RedSlimeState::Chase => sprite_sheet.switch(animations.walk.clone()),
                RedSlimeState::Idle => sprite_sheet.switch(animations.idle.clone())
            }
        }
    }
}

pub fn behavior(
    config: Res<RedSlimeConfig>,
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
            RedSlimeState::Chase => {
                let to_player = player.translation - transform.translation;
                let direction = to_player.normalize_or_zero();
                let step = direction * config.chase_speed;
                velocity.x = step.x;
                velocity.y = step.y;
            }
        }
    }
}