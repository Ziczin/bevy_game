mod nav_grid;
mod grid_builder;
mod grid_updater;
mod astar;
mod pathfinding;
mod visualize;
pub mod state;

use bevy::prelude::*;
use grid_builder::build_initial_nav_grid;
use grid_updater::update_nav_grid_position;
use pathfinding::update_paths;
use visualize::{visualize_nav_grid, visualize_nav_path, visualize_agent_centers};
pub use visualize::NavigationVisualSettings;

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<NavigationVisualSettings>()
            .add_systems(PostStartup, build_initial_nav_grid)
            .add_systems(Update, (
                update_nav_grid_position,
                update_paths,
                visualize_nav_grid,
                visualize_nav_path,
                visualize_agent_centers,
            ));
    }
}