use bevy::prelude::*;
use avian2d::prelude::*;
use crate::components::core::GameLayer;
use crate::components::markers::Player;
use crate::core::debug_log::DebugLogBuffer;
use super::nav_grid::NavGrid;

pub fn rebuild_nav_grid(grid: &mut NavGrid, spatial_query: &SpatialQuery) {
    let movement_filter = SpatialQueryFilter::from_mask([GameLayer::World]);
    let vision_filter = SpatialQueryFilter::from_mask([GameLayer::VisionBlock]);

    let cell_collider = Collider::rectangle(grid.cell_size, grid.cell_size);

    for y in 0..grid.height {
        for x in 0..grid.width {
            let center = grid.grid_to_world(x, y);
            
            let is_blocked_movement = !spatial_query.shape_intersections(
                &cell_collider,
                center,
                0.0,
                &movement_filter,
            ).is_empty();

            let is_blocked_vision = !spatial_query.shape_intersections(
                &cell_collider,
                center,
                0.0,
                &vision_filter,
            ).is_empty();
            
            grid.set_cell(x, y, !is_blocked_movement, is_blocked_vision);
        }
    }
}

pub fn build_initial_nav_grid(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    spatial_query: SpatialQuery,
    mut debug_log: ResMut<DebugLogBuffer>,
) {
    debug_log.add("🔨 build_initial_nav_grid: Starting...");
    
    let Ok(player_transform) = player_query.single() else { 
        debug_log.add("❌ build_initial_nav_grid: Player not found in PostStartup!");
        return; 
    };
    let player_pos = player_transform.translation.xy();
    
    debug_log.add(format!("🔨 build_initial_nav_grid: Player found at ({:.1}, {:.1})", player_pos.x, player_pos.y));
    
    let cell_size = 16.0;
    let width = 96;
    let height = 72;
    
    let initial_origin_x = (player_pos.x / cell_size).round() * cell_size;
    let initial_origin_y = (player_pos.y / cell_size).round() * cell_size;
    let aligned_player_pos = Vec2::new(initial_origin_x, initial_origin_y);
    
    let mut grid = NavGrid::new(cell_size, width, height, aligned_player_pos);
    rebuild_nav_grid(&mut grid, &spatial_query);
    
    commands.insert_resource(grid);
    debug_log.add(format!("✅ NavGrid построен: {}x{} клеток с выровненным центром", width, height));
}