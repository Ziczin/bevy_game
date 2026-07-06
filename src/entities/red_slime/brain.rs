use bevy::prelude::*;
use avian2d::prelude::*;
use bevy_spritesheet_animation::prelude::SpritesheetAnimation;

use crate::components::markers::Player;
use crate::components::pathfinding::Pathfinder;
use crate::core::debug_log::DebugLogBuffer;
use crate::core::profiling::{ProfilingBuffer, ProfileScope};
use crate::entities::red_slime::state::MovingDirection;
use super::state::{
    RedSlimeState, RedSlimeAnimation, RedSlimeStateHandler, RedSlimeLogicFlags,
    WALK_DISTANCE_END, WALK_DISTANCE_START, EXPECTED_IDLE_FRAME, WAYPOINT_ARRIVAL_THRESHOLD,
};
use super::utils::get_collider_world_position;

#[allow(clippy::type_complexity)]
pub fn brain(
    player_query: Query<&Transform, With<Player>>,
    child_query: Query<(&Transform, Option<&Collider>)>,
    profiling: Res<ProfilingBuffer>,
    mut slime_query: Query<(
        Entity,
        &RedSlimeAnimation,
        &Transform,
        &Children,
        &mut RedSlimeStateHandler,
        &mut SpritesheetAnimation,
        &mut Pathfinder,
        &mut RedSlimeLogicFlags,
        &mut MovingDirection,
    )>,
    mut debug_log: ResMut<DebugLogBuffer>,
) {
    let _scope = ProfileScope::new(&profiling, "entities::red_slime::brain::brain", &["red_slime", "brain", "state", "animation", "ai"]);
    
    let Ok(player_transform) = player_query.single() else { return; };

    let walk_distance_end = *WALK_DISTANCE_END;
    let walk_distance_start = *WALK_DISTANCE_START;
    let expected_idle_frame = *EXPECTED_IDLE_FRAME;
    let waypoint_arrival_threshold = *WAYPOINT_ARRIVAL_THRESHOLD;

    for (
        entity,
        animation,
        transform,
        children,
        mut state_handler,
        mut sprite_sheet,
        mut pathfinder,
        mut logic_flags,
        mut velocity,
    ) in &mut slime_query {
        let enemy_pos = transform.translation.xy();
        let player_pos = player_transform.translation.xy();
        let distance = enemy_pos.distance(player_pos);

        let was_active = pathfinder.is_active;
        pathfinder.is_active = distance >= walk_distance_end && distance <= walk_distance_start;
        
        if pathfinder.is_active != was_active {
            debug_log.add(&["red_slime", "pathfinding"], format!("RedSlime {:?}: Pathfinder active: {} (distance: {:.1})", entity, pathfinder.is_active, distance));
        }

        let collider_pos = get_collider_world_position(transform, children, &child_query);

        let can_maneuver = if matches!(sprite_sheet.progress.frame, 3..=9) {
            logic_flags.insert(RedSlimeLogicFlags::CanMove); false
        }
        else {
            logic_flags.remove(RedSlimeLogicFlags::CanMove); true
        };

        if pathfinder.current_target.is_some() && pathfinder.is_active {
            if state_handler.set(RedSlimeState::Walk) {
                debug_log.add(&["red_slime"], format!("RedSlime {:?}: State -> Walk", entity));
                sprite_sheet.switch(animation.walk.clone());
            }
        } else if sprite_sheet.progress.frame == expected_idle_frame && state_handler.set(RedSlimeState::Idle) {
            debug_log.add(&["red_slime"], format!("RedSlime {:?}: State -> Idle", entity));
            sprite_sheet.switch(animation.idle.clone());
        }
        
        if can_maneuver && let Some(target) = pathfinder.current_target {
            if (target - collider_pos).length() < waypoint_arrival_threshold {
                debug_log.add(&["red_slime", "pathfinding"], format!("RedSlime {:?}: Reached waypoint at ({:.1}, {:.1})", entity, target.x, target.y));
                pathfinder.current_waypoint += 1;
                if pathfinder.current_waypoint < pathfinder.path.len() {
                    *velocity = if let Some(current_target) = Some(pathfinder.path[pathfinder.current_waypoint]) {
                        pathfinder.current_target = Some(current_target);
                        (current_target - collider_pos).normalize_or_zero()
                    }
                    else {
                        Vec2::ZERO
                    }.into();
                } else {
                    pathfinder.current_target = None;
                }
            }
        }
    }
}