#![allow(dead_code)]

mod components;
mod core;
mod entities;
mod modules;
mod systems;

use std::collections::HashSet;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::window::{MonitorSelection, WindowMode, WindowResolution};
use bevy_spritesheet_animation::prelude::*;

use crate::core::config::from_toml;
use crate::core::debug_log::{DefaultPresetConfig, LogPreset};
use crate::core::profiling::{DefaultProfilingPreset, ProfilingPreset};

from_toml!("config/debug/logging.toml", [
    DEBUG_MESSAGE: bool = "settings.enabled",
    ACTIVE_PRESET: String = "settings.active_preset",
    DEFAULT_PRESET: DefaultPresetConfig = "default",
    LOG_PRESETS: Vec<LogPreset> = "preset",
]);

from_toml!("config/debug/profiling.toml", [
    PROFILING_ENABLED: bool = "settings.enabled",
    PROFILING_ACTIVE_PRESET: String = "settings.active_preset",
    PROFILING_DEFAULT_PRESET: DefaultProfilingPreset = "default",
    PROFILING_PRESETS: Vec<ProfilingPreset> = "preset",
]);

from_toml!("config/debug/visual.toml", [
    DEBUG_NAVMESH_POINTS: bool = "visual.navmesh_points",
    DEBUG_NAVMESH_PATHS: bool = "visual.navmesh_paths",
    DEBUG_NAVMESH_AGENTS: bool = "visual.navmesh_agents",
    DEBUG_HITBOXS: bool = "visual.hitboxes",
]);

from_toml!("config/global/display.toml", [
    WINDOW_WIDTH: u32 = "display.window_width",
    WINDOW_HEIGHT: u32 = "display.window_height",
    SCALE_FACTOR_OVERRIDE: f32 = "display.scale_factor_override",
    GIZMO_AABB_COLOR: Vec4 = "gizmos.aabb_color",
    GIZMO_COLLIDER_COLOR: Vec4 = "gizmos.collider_color",
]);

fn main() {
    let default_preset = &*DEFAULT_PRESET;
    let active_preset_name = &*ACTIVE_PRESET;
    let active_preset = LOG_PRESETS
        .iter()
        .find(|p| &p.name == active_preset_name)
        .unwrap_or_else(|| {
            panic!(
                "Active preset '{}' not found in logging.toml",
                active_preset_name
            )
        });

    let interval = active_preset.interval.unwrap_or(default_preset.interval);
    let tags = active_preset
        .tags
        .clone()
        .unwrap_or_else(|| default_preset.tags.clone());
    let exclude = active_preset
        .exclude
        .clone()
        .unwrap_or_else(|| default_preset.exclude.clone());
    let strict = active_preset.strict.unwrap_or(default_preset.strict);

    let profiling_default = &*PROFILING_DEFAULT_PRESET;
    let profiling_preset_name = &*PROFILING_ACTIVE_PRESET;
    let profiling_preset = PROFILING_PRESETS
        .iter()
        .find(|p| &p.name == profiling_preset_name)
        .unwrap_or_else(|| {
            panic!(
                "Profiling preset '{}' not found in profiling.toml",
                profiling_preset_name
            )
        });

    let profiling_interval = profiling_preset
        .interval
        .unwrap_or(profiling_default.interval);
    let profiling_include_tags = profiling_preset
        .include_tags
        .clone()
        .unwrap_or_else(|| profiling_default.include_tags.clone());
    let profiling_exclude_tags = profiling_preset
        .exclude_tags
        .clone()
        .unwrap_or_else(|| profiling_default.exclude_tags.clone());
    let profiling_include_functions = profiling_preset
        .include_functions
        .clone()
        .unwrap_or_else(|| profiling_default.include_functions.clone());
    let profiling_exclude_functions = profiling_preset
        .exclude_functions
        .clone()
        .unwrap_or_else(|| profiling_default.exclude_functions.clone());
    let profiling_strict =
        profiling_preset.strict.unwrap_or(profiling_default.strict);

    let mut binding = App::new();
    let app = binding
        .add_plugins((
            systems::base::BaseSystemsPlugin,
            PhysicsPlugins::default(),
            DefaultPlugins.set(ImagePlugin::default_nearest()).set(
                WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(
                            *WINDOW_WIDTH,
                            *WINDOW_HEIGHT,
                        )
                        .with_scale_factor_override(*SCALE_FACTOR_OVERRIDE),
                        mode: WindowMode::BorderlessFullscreen(
                            MonitorSelection::Primary,
                        ),
                        ..default()
                    }),
                    ..default()
                },
            ),
            SpritesheetAnimationPlugin,
            core::navigation::NavigationPlugin,
            entities::Playground,
        ))
        .add_plugins(modules::health::HealthModulePlugin)
        .add_plugins(modules::value_bar::ValueBarPlugin)
        .insert_resource(Gravity::ZERO)
        .insert_resource(core::navigation::NavigationVisualSettings {
            points: *DEBUG_NAVMESH_POINTS,
            paths: *DEBUG_NAVMESH_PATHS,
            agents: *DEBUG_NAVMESH_AGENTS,
        })
        .insert_resource(core::debug_log::DebugLogBuffer {
            messages: HashSet::new(),
            timer: 0.0,
            enabled: *DEBUG_MESSAGE,
            active_tags: tags,
            interval,
            exclude_tags: exclude,
            strict,
        })
        .insert_resource(core::profiling::ProfilingBuffer {
            enabled: *PROFILING_ENABLED,
            include_tags: profiling_include_tags,
            exclude_tags: profiling_exclude_tags,
            include_functions: profiling_include_functions,
            exclude_functions: profiling_exclude_functions,
            strict: profiling_strict,
            interval: profiling_interval,
            ..default()
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
        .add_plugins((entities::RedSlime, entities::Player))
        .add_systems(First, core::profiling::update_frame_start)
        .add_systems(
            Update,
            (
                systems::movement::lerp_follow::lerp_follow_to_player,
                core::debug_log::flush_debug_logs,
                core::profiling::flush_profiling,
                exit_on_escape,
            ),
        );
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
