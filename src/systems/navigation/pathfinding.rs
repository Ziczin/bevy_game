use bevy::prelude::*;
use crate::components::markers::Player;
use crate::components::pathfinding::Pathfinder;
use super::nav_grid::NavGrid;
use super::astar::find_path;

pub fn update_paths(
    grid: Option<Res<NavGrid>>,
    player_query: Query<&Transform, With<Player>>,
    mut pathfinder_query: Query<(&Transform, &mut Pathfinder)>,
    time: Res<Time>,
) {
    let Some(grid) = grid else { return; };
    let Ok(player_transform) = player_query.single() else { return; };
    let player_pos = player_transform.translation.xy();

    for (transform, mut pathfinder) in &mut pathfinder_query {
        if !pathfinder.is_active {
            continue;
        }

        pathfinder.update_timer += time.delta_secs();
        
        if pathfinder.update_timer >= pathfinder.update_interval {
            pathfinder.update_timer = 0.0;
            let enemy_pos = transform.translation.xy();
            
            if let Some(new_path) = find_path(&grid, enemy_pos, player_pos) {
                pathfinder.path = new_path;
                pathfinder.current_waypoint = 0;
                pathfinder.current_target = pathfinder.path.first().copied();
            } else {
                pathfinder.path.clear();
                pathfinder.current_target = None;
            }
        }
    }
}