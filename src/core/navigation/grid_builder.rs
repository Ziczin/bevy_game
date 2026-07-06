use bevy::prelude::*;
use avian2d::prelude::*;
use crate::components::core::GameLayer;
use crate::components::markers::Player;
use crate::core::debug_log::DebugLogBuffer;
use crate::core::profiling::{ProfilingBuffer, ProfileScope};
use super::nav_grid::NavGrid;
use super::state::{NAV_GRID_CELL_SIZE, NAV_GRID_WIDTH, NAV_GRID_HEIGHT, NO_ROTATION};

pub fn rebuild_nav_grid(
    spatial_query: &SpatialQuery,
    profiling: &ProfilingBuffer,
    grid: &mut NavGrid,
) -> (usize, usize) {
    let _scope = ProfileScope::new(profiling, "core::navigation::grid_builder::rebuild_nav_grid", &["pathfinding", "navgrid", "setup", "spatial_query"]);
    
    let movement_filter = SpatialQueryFilter::from_mask([GameLayer::World]);
    let vision_filter = SpatialQueryFilter::from_mask([GameLayer::VisionBlock]);

    let cell_collider = Collider::rectangle(grid.cell_size, grid.cell_size);
    
    let mut walkable_count = 0;
    let mut blocked_count = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let center = grid.grid_to_world(x, y);
            
            let is_blocked_movement = !spatial_query.shape_intersections(
                &cell_collider, center, *NO_ROTATION, &movement_filter,
            ).is_empty();

            let is_blocked_vision = !spatial_query.shape_intersections(
                &cell_collider, center, *NO_ROTATION, &vision_filter,
            ).is_empty();
            
            let walkable = !is_blocked_movement;
            grid.set_cell(x, y, walkable, is_blocked_vision);
            
            if walkable { walkable_count += 1; } else { blocked_count += 1; }
        }
    }
    return (walkable_count, blocked_count);
}

pub fn build_initial_nav_grid(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    spatial_query: SpatialQuery,
    profiling: Res<ProfilingBuffer>,
    mut debug_log: ResMut<DebugLogBuffer>,
) {
    let _scope = ProfileScope::new(&profiling, "core::navigation::grid_builder::build_initial_nav_grid", &["pathfinding", "navgrid", "setup", "spawn"]);
    
    debug_log.add(&["setup", "pathfinding"], "build_initial_nav_grid: Starting...");
    
    let Ok(player_transform) = player_query.single() else { 
        debug_log.add(&["pathfinding"], "build_initial_nav_grid: Player not found in PostStartup!");
        return; 
    };
    let player_pos = player_transform.translation.xy();
    
    let cell_size = *NAV_GRID_CELL_SIZE;
    let grid_width = *NAV_GRID_WIDTH;
    let grid_height = *NAV_GRID_HEIGHT;
    
    debug_log.add(&["pathfinding"], format!("build_initial_nav_grid: Player found at ({:.1}, {:.1})", player_pos.x, player_pos.y));
    
    let initial_origin_x = (player_pos.x / cell_size).round() * cell_size;
    let initial_origin_y = (player_pos.y / cell_size).round() * cell_size;
    let aligned_player_pos = Vec2::new(initial_origin_x, initial_origin_y);
    
    let mut grid = NavGrid::new(cell_size, grid_width, grid_height, aligned_player_pos);
    let (walkable, blocked) = rebuild_nav_grid(&spatial_query, &profiling, &mut grid);
    
    commands.insert_resource(grid);
    debug_log.add(&["pathfinding"], format!("NavGrid built: {}x{} cells. Walkable: {}, Blocked: {}", grid_width, grid_height, walkable, blocked));
}