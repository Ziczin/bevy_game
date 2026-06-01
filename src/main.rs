mod components;
mod systems;
mod macros;
mod entities;
mod core;

use bevy::prelude::*;
use avian2d::prelude::*;
use bevy_spritesheet_animation::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            PhysicsPlugins::default().with_length_unit(32.0),
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            SpritesheetAnimationPlugin,
        ))
        .insert_resource(Gravity::ZERO)
        .add_plugins(( // Entities
            entities::RedSlime,
            entities::Player,
        ))
        .add_systems(Startup, (
            systems::setup::ground::spawn_tiles,
            systems::setup::ground::spawn_fences,
        ))
        .add_systems(Update, (
            systems::movement::lerp_follow::lerp_follow_to_player,
        ))
        .run();
}