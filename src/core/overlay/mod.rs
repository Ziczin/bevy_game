mod config;
mod buffer;
mod systems;

pub use config::*;
pub use buffer::*;
pub use systems::*;

use bevy::prelude::*;

#[cfg(debug_assertions)]
pub struct DebugOverlayPlugin {
    pub config: OverlayConfig,
}

#[cfg(debug_assertions)]
impl Plugin for DebugOverlayPlugin {
    fn build(&self, app: &mut App) {
        if !app.world().contains_resource::<DebugOverlay>() {
            app.insert_resource(DebugOverlay::new(true));
        }
        app.insert_resource(OverlayActiveTags(self.config.active_tags.clone()))
            .add_systems(Startup, spawn_overlay)
            .add_systems(Update, update_overlay);
    }
}

#[cfg(not(debug_assertions))]
pub struct DebugOverlayPlugin;

#[cfg(not(debug_assertions))]
impl Plugin for DebugOverlayPlugin {
    fn build(&self, _app: &mut App) {}
}

#[cfg(debug_assertions)]
pub fn setup_from_globals(app: &mut App, enabled_override: bool) {
    app.insert_resource(DebugOverlay::new(enabled_override));

    if !enabled_override {
        return;
    }

    use crate::core::config::from_toml;
    use crate::core::overlay::{OverlayPreset, DefaultOverlayPreset};

    from_toml!("config/debug/overlay.toml", [
        ACTIVE_PRESET: String = "settings.active_preset",
        DEFAULT_PRESET: DefaultOverlayPreset = "default",
        OVERLAY_PRESETS: Vec<OverlayPreset> = "preset",
    ]);

    let default_preset = &*DEFAULT_PRESET;
    let active_preset_name = &*ACTIVE_PRESET;
    let active_preset = OVERLAY_PRESETS
        .iter()
        .find(|p| p.name == *active_preset_name)
        .unwrap_or_else(|| {
            panic!(
                "Active preset '{}' not found in overlay.toml",
                active_preset_name
            )
        });

    let tags = if active_preset.tags.is_empty() {
        default_preset.tags.clone()
    } else {
        active_preset.tags.clone()
    };

    let config = OverlayConfig { active_tags: tags };

    app.add_plugins(DebugOverlayPlugin { config });
}

#[cfg(not(debug_assertions))]
pub fn setup_from_globals(_app: &mut App, _enabled_override: bool) {}
