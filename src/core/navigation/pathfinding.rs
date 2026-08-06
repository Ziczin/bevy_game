use bevy::prelude::*;
use avian2d::prelude::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::components::core::GameLayer;
use crate::components::markers::Player;
use crate::components::pathfinding::Pathfinder;
use crate::core::debug_log::{DebugLogBuffer, debug_log};
use crate::core::profiling::{ProfilingBuffer, profile_scope};

use super::state::{
    ASTAR_AGENT_CHECK_RADIUS_MULTIPLIER, ASTAR_AGENT_OCCUPIED_MULTIPLIER,
    ASTAR_DIAGONAL_COST, ASTAR_MAX_AGENT_CHECKS_PER_CELL,
    ASTAR_MAX_EXPANSIONS, ASTAR_MAX_SEARCHES_PER_FRAME, ASTAR_MAX_STEP,
    ASTAR_MIN_STEP, ASTAR_ORTHOGONAL_COST, ASTAR_STEP_DISTANCE_DIVISOR,
    COLLIDER_MIN_SIZE, NO_ROTATION, RAYCAST_AGENT_COLLISION_MULTIPLIER,
    RAYCAST_COLLIDER_RADIUS_SCALE, RAYCAST_MIN_DISTANCE, RAYCAST_MIN_STEP,
    RAYCAST_STEP_RADIUS_SCALE,
};

fn get_collider_world_position(
    transform: &Transform,
    children: &Children,
    child_query: &Query<(&Transform, Option<&Collider>)>,
) -> Vec2 {
    for child in children.iter() {
        if let Ok((child_transform, Some(_collider))) =
            child_query.get(child)
        {
            return transform.translation.xy()
                + child_transform.translation.xy();
        }
    }

    transform.translation.xy()
}

fn raycast_clear(
    start: Vec2,
    end: Vec2,
    spatial_query: &SpatialQuery,
    agent_half_size: Vec2,
    other_agent_positions: &[Vec2],
) -> bool {
    let filter = SpatialQueryFilter::from_mask([GameLayer::World]);

    let radius = agent_half_size.x.min(agent_half_size.y)
        * *RAYCAST_COLLIDER_RADIUS_SCALE;

    let collider = Collider::circle(radius);

    let direction = end - start;
    let distance = direction.length();

    if distance < *RAYCAST_MIN_DISTANCE {
        return true;
    }

    let collision_distance =
        radius * *RAYCAST_AGENT_COLLISION_MULTIPLIER;

    let collision_distance_sq = collision_distance * collision_distance;
    let direction_sq = direction.length_squared();

    // Изменено: другие агенты учитываются как препятствия.
    for &other_position in other_agent_positions {
        let to_other = other_position - start;
        let projection = to_other.dot(direction) / direction_sq;
        let clamped_projection = projection.clamp(0.0, 1.0);

        let closest_position =
            start + direction * clamped_projection;

        let delta = other_position - closest_position;

        if delta.length_squared() < collision_distance_sq {
            return false;
        }
    }

    let direction_normalized = direction / distance;

    let step = (radius * *RAYCAST_STEP_RADIUS_SCALE)
        .max(*RAYCAST_MIN_STEP);

    let steps = (distance / step).ceil() as usize;

    for i in 0..=steps {
        let t = if steps == 0 {
            1.0
        } else {
            (i as f32) / (steps as f32)
        };

        let point = start + direction_normalized * t * distance;

        let is_blocked = !spatial_query
            .shape_intersections(
                &collider,
                point,
                *NO_ROTATION,
                &filter,
            )
            .is_empty();

        if is_blocked {
            return false;
        }
    }

    true
}

#[derive(Copy, Clone)]
struct AStarNode {
    f_cost: i32,
    g_cost: i32,
    position: (i32, i32),
}

impl PartialEq for AStarNode {
    fn eq(&self, other: &Self) -> bool {
        self.f_cost == other.f_cost
    }
}

impl Eq for AStarNode {}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_cost.cmp(&self.f_cost)
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_path_astar(
    start: Vec2,
    goal: Vec2,
    spatial_query: &SpatialQuery,
    agent_half_size: Vec2,
    occupied_positions: &[Vec2],
) -> Option<Vec<Vec2>> {
    let collider_min_size = *COLLIDER_MIN_SIZE;
    let no_rotation = *NO_ROTATION;
    let max_expansions = *ASTAR_MAX_EXPANSIONS;

    let agent_collider = Collider::ellipse(
        agent_half_size.x.max(collider_min_size),
        agent_half_size.y.max(collider_min_size),
    );

    let filter = SpatialQueryFilter::from_mask([GameLayer::World]);

    let distance = start.distance(goal);
    let raw_step = distance / *ASTAR_STEP_DISTANCE_DIVISOR;

    // Изменено: динамический шаг A* берётся из конфига.
    let step = raw_step.clamp(*ASTAR_MIN_STEP, *ASTAR_MAX_STEP);

    let start_cell = (
        (start.x / step).round() as i32,
        (start.y / step).round() as i32,
    );

    let goal_cell = (
        (goal.x / step).round() as i32,
        (goal.y / step).round() as i32,
    );

    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut g_score: HashMap<(i32, i32), i32> = HashMap::new();

    // Изменено: closed_cells защищает от повторного раскрытия узлов.
    let mut closed_cells: HashSet<(i32, i32)> = HashSet::new();

    g_score.insert(start_cell, 0);

    open_set.push(AStarNode {
        f_cost: 0,
        g_cost: 0,
        position: start_cell,
    });

    let directions = [
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ];

    let min_distance =
        agent_half_size.length() * *ASTAR_AGENT_OCCUPIED_MULTIPLIER;

    let min_distance_sq = min_distance * min_distance;

    let agent_check_radius =
        min_distance * *ASTAR_AGENT_CHECK_RADIUS_MULTIPLIER;

    let agent_check_radius_sq = agent_check_radius * agent_check_radius;

    let max_agent_checks_per_cell = *ASTAR_MAX_AGENT_CHECKS_PER_CELL;

    let mut expanded_cells = 0usize;

    while let Some(current) = open_set.pop() {
        let best_g_cost = g_score
            .get(&current.position)
            .copied()
            .unwrap_or(i32::MAX);

        if current.g_cost != best_g_cost {
            continue;
        }

        if closed_cells.contains(&current.position) {
            continue;
        }

        closed_cells.insert(current.position);

        if current.position == goal_cell {
            let mut path = Vec::new();
            let mut current_position = current.position;

            while let Some(&previous_position) =
                came_from.get(&current_position)
            {
                let position = Vec2::new(
                    current_position.0 as f32 * step,
                    current_position.1 as f32 * step,
                );

                path.push(position);
                current_position = previous_position;
            }

            path.reverse();

            return Some(path);
        }

        if expanded_cells >= max_expansions {
            return None;
        }

        expanded_cells += 1;

        for &(dx, dy) in &directions {
            let next_cell = (
                current.position.0 + dx,
                current.position.1 + dy,
            );

            if closed_cells.contains(&next_cell) {
                continue;
            }

            let next_position = Vec2::new(
                next_cell.0 as f32 * step,
                next_cell.1 as f32 * step,
            );

            // Изменено: блокировка определяется наличием пересечений.
            let is_blocked_by_static = !spatial_query
                .shape_intersections(
                    &agent_collider,
                    next_position,
                    no_rotation,
                    &filter,
                )
                .is_empty();

            if is_blocked_by_static {
                continue;
            }

            let mut occupied_by_agent = false;
            let mut checked_agents = 0usize;

            for &agent_position in occupied_positions {
                if checked_agents >= max_agent_checks_per_cell {
                    break;
                }

                let delta = next_position - agent_position;
                let distance_sq = delta.length_squared();

                if distance_sq < agent_check_radius_sq
                    && distance_sq < min_distance_sq
                {
                    occupied_by_agent = true;
                    break;
                }

                checked_agents += 1;
            }

            if occupied_by_agent {
                continue;
            }

            let move_cost = if dx != 0 && dy != 0 {
                *ASTAR_DIAGONAL_COST
            } else {
                *ASTAR_ORTHOGONAL_COST
            };

            let tentative_g_cost = current.g_cost + move_cost;

            let best_next_g_cost = g_score
                .get(&next_cell)
                .copied()
                .unwrap_or(i32::MAX);

            if tentative_g_cost < best_next_g_cost {
                came_from.insert(next_cell, current.position);
                g_score.insert(next_cell, tentative_g_cost);

                let delta_x = (next_cell.0 - goal_cell.0).abs();
                let delta_y = (next_cell.1 - goal_cell.1).abs();

                let min_delta = delta_x.min(delta_y);
                let max_delta = delta_x.max(delta_y);

                let heuristic_cost = *ASTAR_DIAGONAL_COST * min_delta
                    + *ASTAR_ORTHOGONAL_COST * (max_delta - min_delta);

                let f_cost = tentative_g_cost + heuristic_cost;

                open_set.push(AStarNode {
                    f_cost,
                    g_cost: tentative_g_cost,
                    position: next_cell,
                });
            }
        }
    }

    None
}

pub fn update_paths(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    spatial_query: SpatialQuery,
    child_query: Query<(&Transform, Option<&Collider>)>,
    profiling: Res<ProfilingBuffer>,
    mut pathfinder_query: Query<(
        Entity,
        &Transform,
        &Children,
        &mut Pathfinder,
    )>,
    mut debug_log: ResMut<DebugLogBuffer>,
) {
    profile_scope!(
        &profiling,
        "core::navigation::pathfinding::update_paths",
        &["pathfinding", "astar", "update", "ai"]
    );

    let Ok(player_transform) = player_query.single() else {
        debug_log!(
            &mut debug_log,
            &["pathfinding"],
            "update_paths: Player not found"
        );

        return;
    };

    let player_pos = player_transform.translation.xy();
    let current_time = time.elapsed_secs();

    // Изменено: позиции агентов берутся из реальных Children.
    let all_agent_positions: Vec<(Entity, Vec2)> = pathfinder_query
        .iter()
        .map(|(entity, transform, children, _)| {
            (
                entity,
                get_collider_world_position(
                    transform,
                    children,
                    &child_query,
                ),
            )
        })
        .collect();

    // Изменено: бюджет A* поисков берётся из конфига.
    let mut remaining_path_searches = *ASTAR_MAX_SEARCHES_PER_FRAME;

    for (entity, transform, children, mut pathfinder) in
        &mut pathfinder_query
    {
        if !pathfinder.is_active {
            continue;
        }

        if pathfinder.next_update_time > current_time {
            continue;
        }

        let collider_pos = get_collider_world_position(
            transform,
            children,
            &child_query,
        );

        if collider_pos.distance(player_pos)
            <= pathfinder.arrival_threshold
        {
            pathfinder.path.clear();
            pathfinder.current_target = None;
            pathfinder.use_raycast = false;

            pathfinder.next_update_time = current_time
                + rand::random::<f32>() * pathfinder.update_interval;

            continue;
        }

        let occupied_positions: Vec<Vec2> = all_agent_positions
            .iter()
            .filter(|(other_entity, _)| *other_entity != entity)
            .map(|(_, position)| *position)
            .collect();

        let raycast_ok = raycast_clear(
            collider_pos,
            player_pos,
            &spatial_query,
            pathfinder.agent_half_size,
            &occupied_positions,
        );

        if raycast_ok {
            pathfinder.path = vec![player_pos];
            pathfinder.current_waypoint = 0;
            pathfinder.current_target = Some(player_pos);
            pathfinder.use_raycast = true;

            debug_log!(
                &mut debug_log,
                &["pathfinding"],
                "Slime {:?}: Using raycast",
                entity
            );

            let delay =
                rand::random::<f32>() * pathfinder.update_interval;

            pathfinder.next_update_time = current_time + delay;

            continue;
        }

        pathfinder.use_raycast = false;

        if remaining_path_searches == 0 {
            pathfinder.next_update_time = current_time
                + rand::random::<f32>() * pathfinder.update_interval;

            continue;
        }

        remaining_path_searches -= 1;

        if let Some(path) = find_path_astar(
            collider_pos,
            player_pos,
            &spatial_query,
            pathfinder.agent_half_size,
            &occupied_positions,
        ) {
            pathfinder.path = path;
            pathfinder.current_waypoint = 0;

            if let Some(&first) = pathfinder.path.first() {
                pathfinder.current_target = Some(first);
            }

            debug_log!(
                &mut debug_log,
                &["pathfinding"],
                "Slime {:?}: A* found path",
                entity
            );
        } else {
            pathfinder.path.clear();
            pathfinder.current_target = None;

            debug_log!(
                &mut debug_log,
                &["pathfinding"],
                "Slime {:?}: A* path NOT found",
                entity
            );
        }

        let delay = rand::random::<f32>() * pathfinder.update_interval;

        pathfinder.next_update_time = current_time + delay;
    }
}
