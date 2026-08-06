mod pathfinding;
pub mod state;
mod visualize;

use bevy::prelude::*;
use pathfinding::update_paths;
pub use visualize::NavigationVisualSettings;
use visualize::{
    visualize_agent_centers, visualize_nav_grid, visualize_nav_path,
};

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NavigationVisualSettings>()
            .add_systems(Update, (
                update_paths,
                visualize_nav_grid,
                visualize_nav_path,
                visualize_agent_centers,
            ));
    }
}
