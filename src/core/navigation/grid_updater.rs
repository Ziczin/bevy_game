// src/core/navigation/grid_updater.rs
use bevy::prelude::*;
use avian2d::spatial_query::SpatialQuery;
use crate::components::markers::Player;
use crate::core::debug_log::DebugLogBuffer;
use super::nav_grid::NavGrid;
use super::grid_builder::rebuild_nav_grid;

pub fn update_nav_grid_position(
    grid: Option<ResMut<NavGrid>>,
    player_query: Query<&Transform, With<Player>>,
    spatial_query: SpatialQuery,
    mut debug_log: ResMut<DebugLogBuffer>,
) {
    let Some(mut grid) = grid else { 
        debug_log.add("⚠️ update_nav_grid_position: NavGrid not ready");
        return; 
    };
    
    let Ok(player_transform) = player_query.single() else { 
        debug_log.add("⚠️ update_nav_grid_position: Player not found");
        return; 
    };
    let player_pos = player_transform.translation.xy();
    
    let threshold = grid.cell_size; 
    let distance = player_pos - grid.origin;
    
    if distance.x.abs() > threshold || distance.y.abs() > threshold {
        let new_origin_x = (player_pos.x / grid.cell_size).round() * grid.cell_size;
        let new_origin_y = (player_pos.y / grid.cell_size).round() * grid.cell_size;
        
        grid.origin = Vec2::new(new_origin_x, new_origin_y);
        
        let (walkable, blocked) = rebuild_nav_grid(&mut grid, &spatial_query);
        debug_log.add(format!("🔄 NavGrid snapped to ({:.1}, {:.1}). Walkable: {}, Blocked: {}", grid.origin.x, grid.origin.y, walkable, blocked));
    }
}