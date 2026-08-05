use bevy::prelude::*;

mod behavior;
mod brain;
mod state;
mod summon;
mod utils;

pub struct RedSlimePlugin;

impl Plugin for RedSlimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, summon::summon)
            .add_systems(Update, (brain::brain, behavior::behavior).chain());
    }
}
