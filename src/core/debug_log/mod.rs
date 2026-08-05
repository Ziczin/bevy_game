mod config;
mod buffer;
mod systems;

pub use config::*;
pub use buffer::*;
pub use systems::*;

use bevy::prelude::*;

#[cfg(debug_assertions)]
pub struct DebugLogPlugin {
    pub config: DebugLogConfig,
}

#[cfg(debug_assertions)]
impl Plugin for DebugLogPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DebugLogBuffer {
            messages: std::collections::HashSet::new(),
            timer: 0.0,
            enabled: self.config.enabled,
            active_tags: self.config.active_tags.clone(),
            interval: self.config.interval,
            exclude_tags: self.config.exclude_tags.clone(),
            strict: self.config.strict,
        });
        app.add_systems(Update, flush_debug_logs);
    }
}

#[cfg(not(debug_assertions))]
pub struct DebugLogPlugin;

#[cfg(not(debug_assertions))]
impl Plugin for DebugLogPlugin {
    fn build(&self, _app: &mut App) {}
}

#[cfg(debug_assertions)]
pub fn setup_from_globals(app: &mut App, enabled_override: bool) {
    use crate::core::config::from_toml;
    use crate::core::debug_log::{LogPreset, DefaultPresetConfig};

    from_toml!("config/debug/logging.toml", [
        ACTIVE_PRESET: String = "settings.active_preset",
        DEFAULT_PRESET: DefaultPresetConfig = "default",
        LOG_PRESETS: Vec<LogPreset> = "preset",
    ]);

    let default_preset = &*DEFAULT_PRESET;
    let active_preset_name = &*ACTIVE_PRESET;
    let active_preset = LOG_PRESETS.iter().find(|p| &p.name == active_preset_name)
        .unwrap_or_else(|| panic!("Active preset '{}' not found in logging.toml", active_preset_name));

    let interval = active_preset.interval.unwrap_or(default_preset.interval);
    let tags = active_preset.tags.clone().unwrap_or_else(|| default_preset.tags.clone());
    let exclude = active_preset.exclude.clone().unwrap_or_else(|| default_preset.exclude.clone());
    let strict = active_preset.strict.unwrap_or(default_preset.strict);

    let config = DebugLogConfig {
        enabled: enabled_override,
        active_tags: tags,
        interval,
        exclude_tags: exclude,
        strict,
    };

    app.add_plugins(DebugLogPlugin { config });
}

#[cfg(not(debug_assertions))]
pub fn setup_from_globals(_app: &mut App, _enabled_override: bool) {}
