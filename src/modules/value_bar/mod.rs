// FILE: src/modules/value_bar/mod.rs
mod components;
mod systems;
mod config;

pub use components::*;
pub use systems::*;
pub use config::*;

use bevy::prelude::*;

pub struct ValueBarPlugin;

impl Plugin for ValueBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_delayed_value,
            update_value_bar_visuals,
            update_value_bar_visibility,
        ));
    }
}