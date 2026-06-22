// src/modules/health/mod.rs
mod components;
mod systems;
mod bundles;

pub use components::*;
pub use systems::*;
pub use bundles::*;

use bevy::prelude::*;

pub struct HealthModulePlugin;

impl Plugin for HealthModulePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<DamageEvent>()
            .add_systems(Update, (
                update_delayed_health,
                update_health_bar_visuals,
                update_health_bar_visibility,
                apply_damage_events,
            ));
    }
}