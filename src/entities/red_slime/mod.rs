// src/entities/red_slime/mod.rs
use bevy::prelude::*;

mod behavior;
mod state;
mod brain;
mod summon;
mod utils;

pub struct RedSlimePlugin;

impl Plugin for RedSlimePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, summon::summon)
            .add_systems(Update, (
                brain::brain,
                behavior::behavior,
            ).chain());
    }
}