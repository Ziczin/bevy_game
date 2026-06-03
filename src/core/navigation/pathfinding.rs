use bevy::prelude::*;
use avian2d::prelude::*;
use std::collections::HashSet;
use crate::components::markers::Player;
use crate::components::pathfinding::Pathfinder;
use crate::core::debug_log::DebugLogBuffer;
use super::nav_grid::NavGrid;
use super::astar::find_path;

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

/// Вычисляет все клетки, которые занимает агент с учётом его размера
fn get_occupied_cells(
    grid: &NavGrid,
    collider_pos: Vec2,
    agent_half_size: Vec2,
) -> HashSet<(usize, usize)> {
    let mut cells = HashSet::new();
    
    // Вычисляем границы прямоугольника вокруг агента
    let min_x = collider_pos.x - agent_half_size.x;
    let max_x = collider_pos.x + agent_half_size.x;
    let min_y = collider_pos.y - agent_half_size.y;
    let max_y = collider_pos.y + agent_half_size.y;
    
    // Преобразуем в координаты сетки
    if let Some((min_grid_x, _)) = grid.world_to_grid(Vec2::new(min_x, collider_pos.y)) {
        if let Some((max_grid_x, _)) = grid.world_to_grid(Vec2::new(max_x, collider_pos.y)) {
            if let Some((_, min_grid_y)) = grid.world_to_grid(Vec2::new(collider_pos.x, min_y)) {
                if let Some((_, max_grid_y)) = grid.world_to_grid(Vec2::new(collider_pos.x, max_y)) {
                    // Добавляем все клетки в прямоугольнике
                    for x in min_grid_x..=max_grid_x {
                        for y in min_grid_y..=max_grid_y {
                            cells.insert((x, y));
                        }
                    }
                }
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

    // Собираем занятые клетки от всех агентов с учётом их размеров
    let mut all_occupied_cells = HashSet::new();
    let mut agent_positions: Vec<(Entity, Vec2)> = Vec::new();
    
    for (entity, transform, children, pathfinder) in &pathfinder_query {
        let collider_pos = get_collider_world_position(transform, children, &child_query);
        agent_positions.push((entity, collider_pos));
        
        // Получаем все клетки, которые занимает этот агент
        let occupied = get_occupied_cells(&grid, collider_pos, pathfinder.agent_half_size);
        all_occupied_cells.extend(occupied);
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
            
            // Создаём копию occupied_cells без клеток текущего агента
            let mut occupied_without_self = all_occupied_cells.clone();
            let self_occupied = get_occupied_cells(&grid, collider_pos, pathfinder.agent_half_size);
            for cell in self_occupied {
                occupied_without_self.remove(&cell);
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