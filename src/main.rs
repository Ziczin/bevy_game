mod components;
mod systems;
mod macros;
mod entities;
mod core;

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            FrameTimeDiagnosticsPlugin::default(),
            SpritesheetAnimationPlugin,
        ))
        .add_plugins(( // Entities
            entities::RedSlime,
            entities::Player,
        ))
        .add_systems(Startup, (
            systems::setup::ground::spawn_tiles,
        ))
        .add_systems(Update, (
            systems::base::apply_velocity,
            systems::movement::lerp_follow::lerp_follow_to_player,
        ))
        .run();
}