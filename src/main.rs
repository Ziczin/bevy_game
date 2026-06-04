mod components;
mod systems;
mod macros;
mod entities;
mod core;

use std::collections::HashSet;

use bevy::prelude::*;
use avian2d::prelude::*;
use bevy_spritesheet_animation::prelude::*;

const DEBUG_MESSAGE: bool = true;
const DEBUG_NAVMESH: bool = true;
const DEBUG_HITBOXS: bool = true;

fn main() {
    let mut binding = App::new();
    let mut app = binding
        .add_plugins((
            systems::base::BaseSystemsPlugin,
            PhysicsPlugins::default(),
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            SpritesheetAnimationPlugin,
            core::navigation::NavigationPlugin,
            entities::Playground,
        ))
        .insert_resource(Gravity::ZERO)
        .insert_resource(core::navigation::NavigationVisualSettings { enabled: DEBUG_NAVMESH })
        .insert_resource(core::debug_log::DebugLogBuffer {
            messages: HashSet::new(),
            timer: 0.0,
            enabled: DEBUG_MESSAGE,
        })
        .insert_gizmo_config(
            PhysicsGizmos {
                aabb_color: Some(Color::srgb(1.0, 1.0, 1.0)),
                collider_color: Some(Color::srgb(1.0, 1.0, 0.0)),
                ..default()
            },
            GizmoConfig::default(),
        )
        .add_plugins((
            entities::RedSlime,
            entities::Player,
        ))
        .add_systems(Update, (
            systems::movement::lerp_follow::lerp_follow_to_player,
            core::debug_log::flush_debug_logs,
        ));
    if DEBUG_HITBOXS {
        app.add_plugins(PhysicsDebugPlugin::default());
    }
    app.run();
}