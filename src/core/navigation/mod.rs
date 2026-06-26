mod astar;
mod grid_builder;
mod grid_updater;
mod nav_grid;
mod pathfinding;
pub mod state;
mod visualize;

use bevy::prelude::*;
use grid_builder::build_initial_nav_grid;
use grid_updater::update_nav_grid_position;
use pathfinding::update_paths;
pub use visualize::NavigationVisualSettings;
use visualize::{
    visualize_agent_centers, visualize_nav_grid, visualize_nav_path,
};

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NavigationVisualSettings>()
            .add_systems(PostStartup, build_initial_nav_grid)
            .add_systems(
                Update,
                (
                    update_nav_grid_position,
                    update_paths,
                    visualize_nav_grid,
                    visualize_nav_path,
                    visualize_agent_centers,
                ),
            );
    }
}
