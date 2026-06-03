use bevy::prelude::*;
use avian2d::prelude::*;
use crate::components::markers::Player;
use crate::components::pathfinding::Pathfinder;
use crate::core::debug_log::DebugLogBuffer;
use super::nav_grid::NavGrid;
use super::astar::find_path;

/// Получает мировую позицию коллайдера агента
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

    let mut count = 0;
    for (entity, transform, children, mut pathfinder) in &mut pathfinder_query {
        count += 1;
        
        if !pathfinder.is_active {
            continue;
        }

        // Получаем позицию коллайдера (физический центр агента)
        let collider_pos = get_collider_world_position(transform, children, &child_query);
        
        // Проверяем, достиг ли слайм расстояния до цели
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
            
            if let Some(new_path) = find_path(&grid, collider_pos, player_pos, &spatial_query, pathfinder.agent_half_size, pathfinder.arrival_threshold) {
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