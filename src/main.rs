#![allow(dead_code)]

mod components;
mod systems;
mod entities;
mod core;
mod modules;

use std::collections::HashSet;

use bevy::prelude::*;
use bevy::window::{WindowResolution, WindowMode, MonitorSelection};
use avian2d::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::core::config::from_toml;

from_toml!("config/debug/settings.toml", [
    DEBUG_MESSAGE: bool = "debug.message",
    DEBUG_NAVMESH: bool = "debug.navmesh",
    DEBUG_HITBOXS: bool = "debug.hitboxes",
]);

from_toml!("config/global/display.toml", [
    WINDOW_WIDTH: u32 = "display.window_width",
    WINDOW_HEIGHT: u32 = "display.window_height",
    SCALE_FACTOR_OVERRIDE: f32 = "display.scale_factor_override",
    GIZMO_AABB_COLOR: Vec4 = "gizmos.aabb_color",
    GIZMO_COLLIDER_COLOR: Vec4 = "gizmos.collider_color",
]);

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
                        resolution: WindowResolution::new(*WINDOW_WIDTH, *WINDOW_HEIGHT)
                            .with_scale_factor_override(*SCALE_FACTOR_OVERRIDE),
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        ..default()
                    }),
                    ..default()
                }),
            SpritesheetAnimationPlugin,
            core::navigation::NavigationPlugin,
            entities::Playground,
        ))
        .add_plugins(modules::health::HealthModulePlugin)
        .add_plugins(modules::value_bar::ValueBarPlugin)
        .insert_resource(Gravity::ZERO)
        .insert_resource(core::navigation::NavigationVisualSettings { enabled: *DEBUG_NAVMESH })
        .insert_resource(core::debug_log::DebugLogBuffer {
            messages: HashSet::new(),
            timer: 0.0,
            enabled: *DEBUG_MESSAGE,
        })
        .insert_gizmo_config(
            PhysicsGizmos {
                aabb_color: Some(Color::srgba(
                    GIZMO_AABB_COLOR.x,
                    GIZMO_AABB_COLOR.y,
                    GIZMO_AABB_COLOR.z,
                    GIZMO_AABB_COLOR.w,
                )),
                collider_color: Some(Color::srgba(
                    GIZMO_COLLIDER_COLOR.x,
                    GIZMO_COLLIDER_COLOR.y,
                    GIZMO_COLLIDER_COLOR.z,
                    GIZMO_COLLIDER_COLOR.w,
                )),
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
    if *DEBUG_HITBOXS {
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