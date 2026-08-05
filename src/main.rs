#![allow(dead_code)]
mod components;
mod systems;
mod entities;
mod core;
mod modules;

use bevy::prelude::*;
use avian2d::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use crate::core::debug::DebugToolsPlugin;
use crate::systems::exit::exit_on_escape;

fn main() {
    let mut app = App::new();

    core::camera::configure_window(&mut app);

    app.add_plugins((
        systems::base::BaseSystemsPlugin,
        PhysicsPlugins::default(),
        SpritesheetAnimationPlugin,
        core::navigation::NavigationPlugin,
        entities::Playground,
        modules::health::HealthModulePlugin,
        modules::value_bar::ValueBarPlugin,
        entities::RedSlime,
        entities::Player,
        DebugToolsPlugin,
    ));

    app.insert_resource(Gravity::ZERO)
        .insert_resource(core::camera::load_zoom_config())
        .insert_resource(core::camera::load_follow_config());

    #[cfg(not(debug_assertions))]
    {
        app.insert_resource(core::debug_log::DebugLogBuffer::default());
        app.insert_resource(core::profiling::ProfilingBuffer::default());
    }

    core::camera::configure_gizmos(&mut app);

    app.add_systems(Update, (
        core::camera::apply_camera_follow,
        core::camera::zoom_system,
        core::camera::apply_zoom_lerp,
        exit_on_escape,
    ));

    app.run();
}
