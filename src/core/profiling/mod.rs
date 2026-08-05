mod config;
mod buffer;
mod systems;

pub use config::*;
pub use buffer::*;
pub use systems::*;

use bevy::prelude::*;

#[cfg(debug_assertions)]
pub struct DebugProfilingPlugin {
    pub config: DebugProfilingConfig,
}

#[cfg(debug_assertions)]
impl Plugin for DebugProfilingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ProfilingBuffer {
            enabled: self.config.enabled,
            include_tags: self.config.include_tags.clone(),
            exclude_tags: self.config.exclude_tags.clone(),
            include_functions: self.config.include_functions.clone(),
            exclude_functions: self.config.exclude_functions.clone(),
            strict: self.config.strict,
            interval: self.config.interval,
            ..Default::default()
        });
        app.add_systems(First, update_frame_start);
        app.add_systems(Update, flush_profiling);
    }
}

#[cfg(not(debug_assertions))]
pub struct DebugProfilingPlugin;

#[cfg(not(debug_assertions))]
impl Plugin for DebugProfilingPlugin {
    fn build(&self, _app: &mut App) {}
}

#[cfg(debug_assertions)]
pub fn setup_from_globals(app: &mut App, enabled_override: bool) {
    use crate::core::config::from_toml;
    use crate::core::profiling::{ProfilingPreset, DefaultProfilingPreset};

    from_toml!("config/debug/profiling.toml", [
        PROFILING_ACTIVE_PRESET: String = "settings.active_preset",
        PROFILING_DEFAULT_PRESET: DefaultProfilingPreset = "default",
        PROFILING_PRESETS: Vec<ProfilingPreset> = "preset",
    ]);

    let profiling_default = &*PROFILING_DEFAULT_PRESET;
    let profiling_preset_name = &*PROFILING_ACTIVE_PRESET;
    let profiling_preset = PROFILING_PRESETS.iter().find(|p| &p.name == profiling_preset_name)
        .unwrap_or_else(|| panic!("Profiling preset '{}' not found in profiling.toml", profiling_preset_name));

    let profiling_interval = profiling_preset.interval.unwrap_or(profiling_default.interval);
    let profiling_include_tags = profiling_preset.include_tags.clone().unwrap_or_else(|| profiling_default.include_tags.clone());
    let profiling_exclude_tags = profiling_preset.exclude_tags.clone().unwrap_or_else(|| profiling_default.exclude_tags.clone());
    let profiling_include_functions = profiling_preset.include_functions.clone().unwrap_or_else(|| profiling_default.include_functions.clone());
    let profiling_exclude_functions = profiling_preset.exclude_functions.clone().unwrap_or_else(|| profiling_default.exclude_functions.clone());
    let profiling_strict = profiling_preset.strict.unwrap_or(profiling_default.strict);

    let config = DebugProfilingConfig {
        enabled: enabled_override,
        include_tags: profiling_include_tags,
        exclude_tags: profiling_exclude_tags,
        include_functions: profiling_include_functions,
        exclude_functions: profiling_exclude_functions,
        strict: profiling_strict,
        interval: profiling_interval,
    };

    app.add_plugins(DebugProfilingPlugin { config });
}

#[cfg(not(debug_assertions))]
pub fn setup_from_globals(_app: &mut App, _enabled_override: bool) {}
