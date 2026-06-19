use bevy::prelude::*;

mod animation;
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