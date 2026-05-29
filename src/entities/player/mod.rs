use bevy::prelude::*;

mod state;
mod summon;
mod handle_input;
mod animation;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, summon::summon)
            .add_systems(Update, (
                handle_input::handle_input,
            ).chain());
    }
}