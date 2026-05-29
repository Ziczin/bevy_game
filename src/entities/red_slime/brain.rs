use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use super::state::{RedSlimeState, RedSlimeAnimation, RedSlimeStateHandler, WALK_DISTANCE_END, WALK_DISTANCE_START};
use crate::components::markers::Player;

pub fn brain(
    mut slime: Query<(
        &mut RedSlimeStateHandler,
        &mut SpritesheetAnimation,
        &RedSlimeAnimation,
        &Transform
    )>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player) = player_query.single() else { return; };

    for (
        mut state_handler, 
        mut sprite_sheet,
        animation,
        transform
    ) in &mut slime {
        let to_player = player.translation - transform.translation;
        let distance = to_player.length();

        if WALK_DISTANCE_END < distance && distance <WALK_DISTANCE_START {
            if state_handler.set(RedSlimeState::Walk) {
                sprite_sheet.switch(animation.walk.clone());
            }
        } else if sprite_sheet.progress.frame == 0 {
            if state_handler.set(RedSlimeState::Idle) {
                sprite_sheet.switch(animation.idle.clone());
            }
        };
    }
}