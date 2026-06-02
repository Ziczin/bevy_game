use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::SpritesheetAnimation;

use crate::components::markers::Player;
use crate::components::pathfinding::Pathfinder;
use super::state::{RedSlimeState, RedSlimeAnimation, RedSlimeStateHandler, WALK_DISTANCE_END, WALK_DISTANCE_START};

pub fn brain(
    player_query: Query<&Transform, With<Player>>,
    mut slime_query: Query<(
        &mut RedSlimeStateHandler,
        &mut SpritesheetAnimation,
        &RedSlimeAnimation,
        &mut Pathfinder,
        &Transform,
    )>,
) {
    let Ok(player_transform) = player_query.single() else { return; };

    for (
        mut state_handler, 
        mut sprite_sheet,
        animation,
        mut pathfinder,
        transform,
    ) in &mut slime_query {
        let enemy_pos = transform.translation.xy();
        let player_pos = player_transform.translation.xy();
        let distance = enemy_pos.distance(player_pos);

        pathfinder.is_active = distance >= WALK_DISTANCE_END && distance <= WALK_DISTANCE_START;

        if pathfinder.current_target.is_some() {
            if state_handler.set(RedSlimeState::Walk) {
                sprite_sheet.switch(animation.walk.clone());
            }
        } else {
            if sprite_sheet.progress.frame == 0 {
                if state_handler.set(RedSlimeState::Idle) {
                    sprite_sheet.switch(animation.idle.clone());
                }
            }
        }
    }
}