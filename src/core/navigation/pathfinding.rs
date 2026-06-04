use bevy::prelude::*;
use avian2d::prelude::*;
use std::collections::HashSet;
use crate::components::markers::Player;
use crate::components::pathfinding::Pathfinder;
use crate::core::debug_log::DebugLogBuffer;
use super::nav_grid::NavGrid;
use super::astar::find_path;
use super::state::{COLLIDER_MIN_SIZE, ELLIPSE_THRESHOLD};

fn get_collider_world_position(
    transform: &Transform,
    children: &Children,
    child_query: &Query<(&Transform, Option<&Collider>)>,
) -> Vec2 {
    for child in children.iter() {
        if let Ok((child_transform, Some(_collider))) = child_query.get(child) {
            return transform.translation.xy() + child_transform.translation.xy();
        }
    }
    transform.translation.xy()
}

fn get_occupied_cells(
    grid: &NavGrid,
    other_pos: Vec2,
    other_half_size: Vec2,
    current_half_size: Vec2,
) -> HashSet<(usize, usize)> {
    let mut cells = HashSet::new();

    let rx = (other_half_size.x + current_half_size.x).max(COLLIDER_MIN_SIZE);
    let ry = (other_half_size.y + current_half_size.y).max(COLLIDER_MIN_SIZE);

    let Some((agent_gx, agent_gy)) = grid.world_to_grid(other_pos) else {
        return cells;
    };

    let cells_rx = (rx / grid.cell_size).ceil() as isize + 1;
    let cells_ry = (ry / grid.cell_size).ceil() as isize + 1;

    for dx in -cells_rx..=cells_rx {
        for dy in -cells_ry..=cells_ry {
            let gx = (agent_gx as isize + dx) as usize;
            let gy = (agent_gy as isize + dy) as usize;

            if gx >= grid.width || gy >= grid.height {
                continue;
            }

            let cell_center = grid.grid_to_world(gx, gy);

            let norm_x = (cell_center.x - other_pos.x) / rx;
            let norm_y = (cell_center.y - other_pos.y) / ry;

            if norm_x * norm_x + norm_y * norm_y <= ELLIPSE_THRESHOLD {
                cells.insert((gx, gy));
            }
        }
    }

    cells
}

pub fn update_paths(
    grid: Option<Res<NavGrid>>,
    player_query: Query<&Transform, With<Player>>,
    spatial_query: SpatialQuery,
    mut pathfinder_query: Query<(Entity, &Transform, &Children, &mut Pathfinder)>,
    child_query: Query<(&Transform, Option<&Collider>)>,
    time: Res<Time>,
    mut debug_log: ResMut<DebugLogBuffer>,
) {
    let Some(grid) = grid else { 
        debug_log.add("⚠️ update_paths: NavGrid not ready");
        return; 
    };
    let Ok(player_transform) = player_query.single() else { 
        debug_log.add("⚠️ update_paths: Player not found");
        return; 
    };
    let player_pos = player_transform.translation.xy();

    let mut agent_data: Vec<(Entity, Vec2, Vec2)> = Vec::new();
    
    for (entity, transform, children, pathfinder) in &pathfinder_query {
        let collider_pos = get_collider_world_position(transform, children, &child_query);
        agent_data.push((entity, collider_pos, pathfinder.agent_half_size));
    }

    let mut count = 0;
    for (entity, transform, children, mut pathfinder) in &mut pathfinder_query {
        count += 1;
        
        if !pathfinder.is_active {
            continue;
        }

        let collider_pos = get_collider_world_position(transform, children, &child_query);
        
        let distance_to_player = collider_pos.distance(player_pos);
        if distance_to_player <= pathfinder.arrival_threshold {
            if !pathfinder.path.is_empty() {
                debug_log.add(format!("🎯 Slime {:?}: Arrived at target (distance: {:.1})", entity, distance_to_player));
                pathfinder.path.clear();
                pathfinder.current_waypoint = 0;
                pathfinder.current_target = None;
            }
            continue;
        }

        pathfinder.update_timer += time.delta_secs();
        
        if pathfinder.update_timer >= pathfinder.update_interval {
            pathfinder.update_timer = 0.0;
            
            debug_log.add(format!("🧠 Slime {:?}: Attempting to find path", entity));
            
            let mut occupied_without_self = HashSet::new();
            
            for (other_entity, other_pos, other_half_size) in &agent_data {
                if *other_entity == entity {
                    continue;
                }
                
                let occupied = get_occupied_cells(
                    &grid, 
                    *other_pos, 
                    *other_half_size, 
                    pathfinder.agent_half_size
                );
                occupied_without_self.extend(occupied);
            }
            
            if let Some(new_path) = find_path(&grid, collider_pos, player_pos, &spatial_query, pathfinder.agent_half_size, pathfinder.arrival_threshold, &occupied_without_self) {
                debug_log.add(format!("✅ Slime {:?}: Path found with {} waypoints", entity, new_path.len()));
                pathfinder.path = new_path;
                pathfinder.current_waypoint = 0;
                pathfinder.current_target = pathfinder.path.first().copied();
            } else {
                debug_log.add(format!("❌ Slime {:?}: Path NOT found", entity));
                pathfinder.path.clear();
                pathfinder.current_target = None;
            }
        }
    }

    if count == 0 {
        debug_log.add("⚠️ update_paths: No pathfinders found");
    }
}