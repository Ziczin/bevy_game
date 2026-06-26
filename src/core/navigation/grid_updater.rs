use bevy::prelude::*;
use avian2d::spatial_query::SpatialQuery;
use crate::components::markers::Player;
use crate::core::debug_log::DebugLogBuffer;
use crate::core::profiling::{ProfilingBuffer, ProfileScope};
use super::nav_grid::NavGrid;
use super::grid_builder::rebuild_nav_grid;

pub fn update_nav_grid_position(
    player_query: Query<&Transform, With<Player>>,
    spatial_query: SpatialQuery,
    profiling: Res<ProfilingBuffer>,
    grid: Option<ResMut<NavGrid>>,
    mut debug_log: ResMut<DebugLogBuffer>,
) {
    let _scope = ProfileScope::new(&profiling, "core::navigation::grid_updater::update_nav_grid_position", &["pathfinding", "navgrid", "update", "spatial_query"]);
    
    let Some(mut grid) = grid else { 
        debug_log.add(&["pathfinding"], "update_nav_grid_position: NavGrid not ready");
        return; 
    };
    
    let Ok(player_transform) = player_query.single() else { 
        debug_log.add(&["pathfinding"], "update_nav_grid_position: Player not found");
        return; 
    };
    let player_pos = player_transform.translation.xy();
    
    let threshold = grid.cell_size; 
    let distance = player_pos - grid.origin;
    
    if distance.x.abs() > threshold || distance.y.abs() > threshold {
        let new_origin_x = (player_pos.x / grid.cell_size).round() * grid.cell_size;
        let new_origin_y = (player_pos.y / grid.cell_size).round() * grid.cell_size;
        
        grid.origin = Vec2::new(new_origin_x, new_origin_y);
        
        let (walkable, blocked) = rebuild_nav_grid(&spatial_query, &profiling, &mut grid);
        debug_log.add(&["pathfinding"], format!("NavGrid snapped to ({:.1}, {:.1}). Walkable: {}, Blocked: {}", grid.origin.x, grid.origin.y, walkable, blocked));
    }
}