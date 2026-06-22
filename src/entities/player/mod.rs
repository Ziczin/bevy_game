// src/entities/player/mod.rs
use bevy::prelude::*;

mod state;
mod summon;
mod brain;
mod handle_input;
mod behavior;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, summon::summon)
            .add_systems(Update, (
                handle_input::handle_input,
                brain::brain,
                behavior::behavior,
            ).chain()
        );
    }
}