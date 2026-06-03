use bevy::prelude::*;
use avian2d::prelude::*;

use crate::components::pathfinding::Pathfinder;
use crate::core::debug_log::DebugLogBuffer;
use super::state::{RedSlimeStateHandler, RedSlimeState, WALK_SPEED};

/// Получает мировую позицию коллайдера агента
fn get_collider_world_position(
    transform: &Transform,
    children: &Children,
    child_query: &Query<(&Transform, Option<&Collider>)>,
) -> Vec2 {
    for child in children.iter() {
        if let Ok((child_transform, Some(_collider))) = child_query.get(child) {
            // Складываем локальную позицию ребёнка с мировой позицией родителя
            return transform.translation.xy() + child_transform.translation.xy();
        }
    }
    // Если коллайдер не найден, возвращаем позицию родителя
    transform.translation.xy()
}

pub fn behavior(
    mut slime_query: Query<(
        Entity,
        &mut LinearVelocity,
        &RedSlimeStateHandler,
        &mut Pathfinder,
        &Transform,
        &Children,
    )>,
    child_query: Query<(&Transform, Option<&Collider>)>,
    mut debug_log: ResMut<DebugLogBuffer>,
) {
    let mut count = 0;
    
    for (entity, mut velocity, state_handler, mut pathfinder, transform, children) in &mut slime_query {
        count += 1;
        
        // Получаем позицию коллайдера (физический центр агента)
        let collider_pos = get_collider_world_position(transform, children, &child_query);

        match state_handler.get() {
            RedSlimeState::Idle => {
                velocity.x = 0.0;
                velocity.y = 0.0;
            }
            RedSlimeState::Walk => {
                if let Some(target) = pathfinder.current_target {
                    let to_target = target - collider_pos;
                    let distance_to_target = to_target.length();

                    if distance_to_target < 16.0 {
                        pathfinder.current_waypoint += 1;
                        if pathfinder.current_waypoint < pathfinder.path.len() {
                            pathfinder.current_target = Some(pathfinder.path[pathfinder.current_waypoint]);
                            debug_log.add(format!("🎯 Slime {:?}: Moving to waypoint {}", entity, pathfinder.current_waypoint));
                        } else {
                            debug_log.add(format!("🏁 Slime {:?}: Reached final destination", entity));
                            pathfinder.current_target = None;
                            pathfinder.path.clear();
                        }
                    }

                    if let Some(current_target) = pathfinder.current_target {
                        let direction = (current_target - collider_pos).normalize_or_zero();
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

    if count == 0 {
        debug_log.add("⚠️ behavior: No slimes found in query");
    }
}