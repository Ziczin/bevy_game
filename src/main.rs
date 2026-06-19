mod components;
mod systems;
mod entities;
mod core;

use std::collections::HashSet;

use bevy::prelude::*;
use bevy::window::{WindowResolution, WindowMode, MonitorSelection};
use avian2d::prelude::*;
use bevy_spritesheet_animation::prelude::*;

const DEBUG_MESSAGE: bool = true;
const DEBUG_NAVMESH: bool = false;
const DEBUG_HITBOXS: bool = false;

fn main() {
    let mut binding = App::new();
    let app = binding
        .add_plugins((
            systems::base::BaseSystemsPlugin,
            PhysicsPlugins::default(),
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1920, 1080)
                            .with_scale_factor_override(4.0),
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        ..default()
                    }),
                    ..default()
                }),
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
            exit_on_escape,
        ));
    if DEBUG_HITBOXS {
        app.add_plugins(PhysicsDebugPlugin::default());
    }
    app.run();
}

fn exit_on_escape(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut exit: MessageWriter<AppExit>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}