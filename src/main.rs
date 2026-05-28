mod components;
mod systems;
mod macros;
mod resources;

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
        .add_systems(Startup, (
            systems::setup::summon::player,
            systems::setup::summon::red_slime,
            systems::setup::ground::spawn_tiles,
        ))
        .add_systems(Update, (
            systems::movement::player::handle_input,
            systems::base::apply_velocity,
            systems::movement::follow::lerp_follow_to_player,
            systems::behavior::red_slime::brain,
            systems::behavior::red_slime::behavior,
        ))
        .insert_resource(
            resources::behavior::RedSlimeConfig {
                chase_speed: 0.0,
                chase_distance_start: 1000.0,
                chase_distance_end: 100.0,
            }
        )
        .run();
}