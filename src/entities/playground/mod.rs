mod state;
mod spawn_ground;
mod spawn_fences;
mod spawn_paths; // <-- добавить

use bevy::prelude::*;
use spawn_ground::spawn_ground;
use spawn_fences::spawn_fences;
use spawn_paths::spawn_paths; // <-- добавить

pub struct PlaygroundPlugin;

impl Plugin for PlaygroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_ground, spawn_fences, spawn_paths)); // <-- добавить
    }
}