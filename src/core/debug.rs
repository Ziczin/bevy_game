use bevy::prelude::*;
use avian2d::prelude::PhysicsDebugPlugin;
use crate::core::config::from_toml;
use crate::core::debug_log;
use crate::core::profiling;
use crate::modules::debug_overlay::DebugOverlayPlugin;
use crate::core::navigation::NavigationVisualSettings;

from_toml!("config/debug/global.toml", [
    DEBUG_GLOBAL_ENABLED: bool = "global.enabled",
    DEBUG_LOGGING_ENABLED: bool = "modules.logging",
    DEBUG_PROFILING_ENABLED: bool = "modules.profiling",
    DEBUG_VISUAL_ENABLED: bool = "modules.visual",
]);

from_toml!("config/debug/visual.toml", [
    DEBUG_NAVMESH_ENABLED: bool = "navmesh.enabled",
    DEBUG_NAVMESH_POINTS: bool = "navmesh.points",
    DEBUG_NAVMESH_PATHS: bool = "navmesh.paths",
    DEBUG_NAVMESH_AGENTS: bool = "navmesh.agents",
    DEBUG_HITBOXS_ENABLED: bool = "hitboxes.enabled",
    DEBUG_OVERLAY_ENABLED: bool = "overlay.enabled",
]);

#[cfg(debug_assertions)]
pub struct DebugToolsPlugin;

#[cfg(debug_assertions)]
impl Plugin for DebugToolsPlugin {
    fn build(&self, app: &mut App) {
        if !*DEBUG_GLOBAL_ENABLED {
            return;
        }

        debug_log::setup_from_globals(app, *DEBUG_LOGGING_ENABLED);
        profiling::setup_from_globals(app, *DEBUG_PROFILING_ENABLED);

        if *DEBUG_VISUAL_ENABLED {
            if *DEBUG_NAVMESH_ENABLED {
                app.insert_resource(NavigationVisualSettings {
                    points: *DEBUG_NAVMESH_POINTS,
                    paths: *DEBUG_NAVMESH_PATHS,
                    agents: *DEBUG_NAVMESH_AGENTS,
                });
            } else {
                app.insert_resource(NavigationVisualSettings::default());
            }

            if *DEBUG_HITBOXS_ENABLED {
                app.add_plugins(PhysicsDebugPlugin);
            }

            if *DEBUG_OVERLAY_ENABLED {
                app.add_plugins(DebugOverlayPlugin);
            }
        }
    }
}

#[cfg(not(debug_assertions))]
pub struct DebugToolsPlugin;

#[cfg(not(debug_assertions))]
impl Plugin for DebugToolsPlugin {
    fn build(&self, _app: &mut App) {}
}
