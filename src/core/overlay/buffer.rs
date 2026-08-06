use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct DebugOverlay {
    enabled: bool,
    entries: HashMap<String, (String, Vec<String>)>,
}

impl DebugOverlay {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            entries: HashMap::new(),
        }
    }

    pub fn set_with_tags(&mut self, key: impl Into<String>, value: impl ToString, tags: &[&str]) {
        if !self.enabled {
            return;
        }
        let tags: Vec<String> = tags.iter().map(|s| s.to_string()).collect();
        self.entries.insert(key.into(), (value.to_string(), tags));
    }

    pub fn remove(&mut self, key: &str) {
        if !self.enabled {
            return;
        }
        self.entries.remove(key);
    }

    pub fn clear(&mut self) {
        if !self.enabled {
            return;
        }
        self.entries.clear();
    }

    pub fn format_filtered(&self, active_tags: &[String]) -> String {
        if !self.enabled {
            return String::new();
        }
        let mut lines: Vec<String> = Vec::new();
        for (key, (value, tags)) in self.entries.iter() {
            if active_tags.is_empty() {
                lines.push(format!("{}: {}", key, value));
                continue;
            }
            for tag in tags {
                if active_tags.contains(tag) {
                    lines.push(format!("{}: {}", key, value));
                    break;
                }
            }
        }
        lines.sort();
        lines.join("\n")
    }
}

impl Default for DebugOverlay {
    fn default() -> Self {
        Self::new(false)
    }
}
