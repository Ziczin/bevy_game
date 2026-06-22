// src/entities/player/mod.rs
use bevy::prelude::*;

mod state;
mod summon;
mod brain;
mod handle_input;
mod behavior;
mod health_input;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, summon::summon)
            .add_systems(Update, (
                handle_input::handle_input,
                health_input::handle_health_input,
                brain::brain,
                behavior::behavior,
            ).chain()
        );
    }
}