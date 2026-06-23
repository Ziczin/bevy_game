// FILE: src/modules/health/mod.rs
mod components;
mod systems;

pub use components::*;
pub use systems::*;

use bevy::prelude::*;

pub struct HealthModulePlugin;

impl Plugin for HealthModulePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<DamageEvent>()
            .add_systems(Update, apply_damage_events);
    }
}