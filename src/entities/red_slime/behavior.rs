use bevy::prelude::*;
use avian2d::prelude::*;

use crate::components::pathfinding::Pathfinder;
use super::state::{RedSlimeStateHandler, RedSlimeState, WALK_SPEED};

pub fn behavior(
    mut slime_query: Query<(
        &mut LinearVelocity,
        &RedSlimeStateHandler,
        &mut Pathfinder,
        &Transform,
    )>,
) {
    for (mut velocity, state_handler, mut pathfinder, transform) in &mut slime_query {
        let enemy_pos = transform.translation.xy();

        match state_handler.get() {
            RedSlimeState::Idle => {
                velocity.x = 0.0;
                velocity.y = 0.0;
            }
            RedSlimeState::Walk => {
                if let Some(target) = pathfinder.current_target {
                    let to_target = target - enemy_pos;
                    let distance_to_target = to_target.length();

                    if distance_to_target < 16.0 {
                        pathfinder.current_waypoint += 1;
                        if pathfinder.current_waypoint < pathfinder.path.len() {
                            pathfinder.current_target = Some(pathfinder.path[pathfinder.current_waypoint]);
                        } else {
                            pathfinder.current_target = None;
                            pathfinder.path.clear();
                        }
                    }

                    if let Some(current_target) = pathfinder.current_target {
                        let direction = (current_target - enemy_pos).normalize_or_zero();
                        velocity.x = direction.x * WALK_SPEED;
                        velocity.y = direction.y * WALK_SPEED;
                    } else {
                        velocity.x = 0.0;
                        velocity.y = 0.0;
                    }
                } else {
                    velocity.x = 0.0;
                    velocity.y = 0.0;
                }
            }
        }
    }
}