// src/core/debug_log.rs
use bevy::prelude::*;
use std::collections::HashSet;
use crate::core::config::FromTomlValue;

#[derive(Debug, Clone)]
pub struct DefaultPresetConfig {
    pub interval: f32,
    pub tags: Vec<String>,
    pub exclude: Vec<String>,
    pub strict: bool,
}

impl FromTomlValue for DefaultPresetConfig {
    fn from_toml_value(value: &toml::Value) -> Self {
        let table = value.as_table().unwrap_or_else(|| panic!("Expected table for [default], got {:?}", value));
        return Self {
            interval: table.get("interval").and_then(|v| v.as_float()).unwrap_or_else(|| panic!("Missing 'interval' in [default]")) as f32,
            tags: table.get("tags").map(Vec::<String>::from_toml_value).unwrap_or_else(|| panic!("Missing 'tags' in [default]")),
            exclude: table.get("exclude").map(Vec::<String>::from_toml_value).unwrap_or_else(|| panic!("Missing 'exclude' in [default]")),
            strict: table.get("strict").and_then(|v| v.as_bool()).unwrap_or_else(|| panic!("Missing 'strict' in [default]")),
        };
    }
}

#[derive(Debug, Clone)]
pub struct LogPreset {
    pub name: String,
    pub interval: Option<f32>,
    pub tags: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub strict: Option<bool>,
}

impl FromTomlValue for LogPreset {
    fn from_toml_value(value: &toml::Value) -> Self {
        let table = value.as_table().unwrap_or_else(|| panic!("Expected table for LogPreset, got {:?}", value));
        return Self {
            name: table.get("name").and_then(|v| v.as_str()).unwrap_or_else(|| panic!("Missing 'name' in LogPreset")).to_string(),
            interval: table.get("interval").and_then(|v| v.as_float()).map(|v| v as f32),
            tags: table.get("tags").map(Vec::<String>::from_toml_value),
            exclude: table.get("exclude").map(Vec::<String>::from_toml_value),
            strict: table.get("strict").and_then(|v| v.as_bool()),
        };
    }
}

#[derive(Resource, Default)]
pub struct DebugLogBuffer {
    pub messages: HashSet<String>,
    pub timer: f32,
    pub enabled: bool,
    pub active_tags: Vec<String>,
    pub interval: f32,
    pub exclude_tags: Vec<String>,
    pub strict: bool,
}

impl DebugLogBuffer {
    #[cfg(debug_assertions)]
    pub fn add(&mut self, tags: &[&str], msg: impl Into<String>) {
        if !self.enabled {
            return;
        }
        
        if tags.is_empty() {
            panic!("Debug log error: message tags (B) cannot be empty!");
        }

        if !self.exclude_tags.is_empty() && self.exclude_tags.iter().any(|t| tags.contains(&t.as_str())) {
            return;
        }

        let should_log = if self.active_tags.is_empty() {
            true
        } else if self.strict {
            if self.active_tags.len() != tags.len() {
                false
            } else {
                self.active_tags.iter().all(|t| tags.contains(&t.as_str()))
            }
        } else {
            self.active_tags.iter().any(|t| tags.contains(&t.as_str()))
        };

        if should_log {
            let formatted_msg = if !self.strict {
                format!("[{}] {}", tags.join(" "), msg.into())
            } else {
                msg.into()
            };
            self.messages.insert(formatted_msg);
        }
    }

    #[cfg(not(debug_assertions))]
    #[inline(always)]
    pub fn add(&mut self, _tags: &[&str], _msg: impl Into<String>) {}
}

#[cfg(debug_assertions)]
pub fn flush_debug_logs(
    mut buffer: ResMut<DebugLogBuffer>,
    time: Res<Time>,
) {
    if !buffer.enabled {
        return;
    }

    buffer.timer += time.delta_secs();
    
    if buffer.timer >= buffer.interval {
        buffer.timer = 0.0;
        
        if !buffer.messages.is_empty() {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            println!("=== Debug Logs [{}] ===", timestamp);
            for msg in &buffer.messages {
                println!("- {}", msg);
            }
            println!();
        }
        
        buffer.messages.clear();
    }
}

#[cfg(not(debug_assertions))]
pub fn flush_debug_logs(
    _buffer: ResMut<DebugLogBuffer>,
    _time: Res<Time>,
) {}