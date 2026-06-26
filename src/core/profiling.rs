use bevy::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;
use crate::core::config::FromTomlValue;

#[derive(Debug, Clone)]
pub struct DefaultProfilingPreset {
    pub interval: f32,
    pub include_tags: Vec<String>,
    pub exclude_tags: Vec<String>,
    pub include_functions: Vec<String>,
    pub exclude_functions: Vec<String>,
    pub strict: bool,
}

impl FromTomlValue for DefaultProfilingPreset {
    fn from_toml_value(value: &toml::Value) -> Self {
        let table = value.as_table().unwrap_or_else(|| panic!("Expected table for [default], got {:?}", value));
        Self {
            interval: table.get("interval").and_then(|v| v.as_float()).unwrap_or_else(|| panic!("Missing 'interval' in [default]")) as f32,
            include_tags: table.get("include_tags").map(Vec::<String>::from_toml_value).unwrap_or_else(|| panic!("Missing 'include_tags' in [default]")),
            exclude_tags: table.get("exclude_tags").map(Vec::<String>::from_toml_value).unwrap_or_else(|| panic!("Missing 'exclude_tags' in [default]")),
            include_functions: table.get("include_functions").map(Vec::<String>::from_toml_value).unwrap_or_else(|| panic!("Missing 'include_functions' in [default]")),
            exclude_functions: table.get("exclude_functions").map(Vec::<String>::from_toml_value).unwrap_or_else(|| panic!("Missing 'exclude_functions' in [default]")),
            strict: table.get("strict").and_then(|v| v.as_bool()).unwrap_or_else(|| panic!("Missing 'strict' in [default]")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProfilingPreset {
    pub name: String,
    pub interval: Option<f32>,
    pub include_tags: Option<Vec<String>>,
    pub exclude_tags: Option<Vec<String>>,
    pub include_functions: Option<Vec<String>>,
    pub exclude_functions: Option<Vec<String>>,
    pub strict: Option<bool>,
}

impl FromTomlValue for ProfilingPreset {
    fn from_toml_value(value: &toml::Value) -> Self {
        let table = value.as_table().unwrap_or_else(|| panic!("Expected table for ProfilingPreset, got {:?}", value));
        Self {
            name: table.get("name").and_then(|v| v.as_str()).unwrap_or_else(|| panic!("Missing 'name' in ProfilingPreset")).to_string(),
            interval: table.get("interval").and_then(|v| v.as_float()).map(|v| v as f32),
            include_tags: table.get("include_tags").map(Vec::<String>::from_toml_value),
            exclude_tags: table.get("exclude_tags").map(Vec::<String>::from_toml_value),
            include_functions: table.get("include_functions").map(Vec::<String>::from_toml_value),
            exclude_functions: table.get("exclude_functions").map(Vec::<String>::from_toml_value),
            strict: table.get("strict").and_then(|v| v.as_bool()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProfileEntry {
    pub name: String,
    pub tags: Vec<String>,
    pub total_time: u128,
    pub call_count: u64,
}

#[cfg(debug_assertions)]
#[derive(Resource, Default)]
pub struct ProfilingBuffer {
    pub enabled: bool,
    pub include_tags: Vec<String>,
    pub exclude_tags: Vec<String>,
    pub include_functions: Vec<String>,
    pub exclude_functions: Vec<String>,
    pub strict: bool,
    pub interval: f32,
    pub timer: f32,
    pub entries: Mutex<HashMap<String, ProfileEntry>>,
    pub frame_start: Option<Instant>,
    pub last_frame_time: u128,
}

#[cfg(debug_assertions)]
pub struct ProfileScope<'a> {
    buffer: &'a ProfilingBuffer,
    name: String,
    tags: Vec<String>,
    start_time: Instant,
    is_active: bool,
}

#[cfg(debug_assertions)]
impl<'a> ProfileScope<'a> {
    pub fn new(buffer: &'a ProfilingBuffer, name: &str, tags: &[&str]) -> Self {
        let is_active = buffer.should_profile(name, tags);
        Self {
            buffer,
            name: name.to_string(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
            start_time: Instant::now(),
            is_active,
        }
    }
}

#[cfg(debug_assertions)]
impl<'a> Drop for ProfileScope<'a> {
    fn drop(&mut self) {
        if self.is_active {
            let elapsed = self.start_time.elapsed().as_micros();
            let mut entries = self.buffer.entries.lock().unwrap();
            let entry = entries.entry(self.name.clone()).or_insert_with(|| ProfileEntry {
                name: self.name.clone(),
                tags: self.tags.clone(),
                total_time: 0,
                call_count: 0,
            });
            entry.total_time += elapsed;
            entry.call_count += 1;
        }
    }
}

#[cfg(not(debug_assertions))]
pub struct ProfileScope;

#[cfg(not(debug_assertions))]
impl ProfileScope {
    #[inline(always)]
    pub fn new(_buffer: &ProfilingBuffer, _name: &str, _tags: &[&str]) -> Self {
        Self
    }
}

#[cfg(debug_assertions)]
impl ProfilingBuffer {
    pub fn should_profile(&self, name: &str, tags: &[&str]) -> bool {
        if !self.enabled {
            return false;
        }

        if !self.exclude_functions.is_empty() && self.exclude_functions.iter().any(|f| name.contains(f)) {
            return false;
        }

        if !self.exclude_tags.is_empty() && self.exclude_tags.iter().any(|t| tags.contains(&t.as_str())) {
            return false;
        }

        let matches_functions = self.include_functions.is_empty() || self.include_functions.iter().any(|f| name.contains(f));
        let matches_tags = if self.include_tags.is_empty() {
            true
        } else if self.strict {
            self.include_tags.len() == tags.len() && self.include_tags.iter().all(|t| tags.contains(&t.as_str()))
        } else {
            self.include_tags.iter().any(|t| tags.contains(&t.as_str()))
        };

        matches_functions && matches_tags
    }

    pub fn update_frame_start(&mut self) {
        if let Some(last_start) = self.frame_start {
            self.last_frame_time = last_start.elapsed().as_micros();
        }
        self.frame_start = Some(Instant::now());
    }
}

#[cfg(not(debug_assertions))]
impl ProfilingBuffer {
    #[inline(always)]
    pub fn should_profile(&self, _name: &str, _tags: &[&str]) -> bool { false }
    
    #[inline(always)]
    pub fn update_frame_start(&mut self) {}
}

#[cfg(debug_assertions)]
pub fn update_frame_start(mut profiling: ResMut<ProfilingBuffer>) {
    let p = &mut *profiling;
    p.update_frame_start();
}

#[cfg(not(debug_assertions))]
pub fn update_frame_start(_profiling: ResMut<ProfilingBuffer>) {}

#[cfg(debug_assertions)]
pub fn flush_profiling(
    mut profiling: ResMut<ProfilingBuffer>,
    time: Res<Time>,
) {
    let p = &mut *profiling;
    if !p.enabled {
        return;
    }

    p.timer += time.delta_secs();

    if p.timer >= p.interval {
        p.timer = 0.0;

        let mut entries = p.entries.lock().unwrap();
        if !entries.is_empty() {
            let mut sorted_entries: Vec<_> = entries.values().collect();
            sorted_entries.sort_by(|a, b| b.total_time.cmp(&a.total_time));

            let total_logic_time: u128 = sorted_entries.iter().map(|e| e.total_time).sum();
            let frame_time = p.last_frame_time;

            println!("\n=== Profiling Report ===");
            println!("Frame time: {:.2}ms", frame_time as f64 / 1000.0);
            println!("Total logic time: {:.2}ms", total_logic_time as f64 / 1000.0);
            println!();

            for entry in sorted_entries {
                let percent_logic = if total_logic_time > 0 {
                    (entry.total_time as f64 / total_logic_time as f64) * 100.0
                } else {
                    0.0
                };
                let percent_frame = if frame_time > 0 {
                    (entry.total_time as f64 / frame_time as f64) * 100.0
                } else {
                    0.0
                };

                println!("{}: {}μs ({:.1}% logic, {:.1}% frame) [{} calls]",
                    entry.name,
                    entry.total_time,
                    percent_logic,
                    percent_frame,
                    entry.call_count
                );
            }
            println!();

            entries.clear();
        }
    }
}

#[cfg(not(debug_assertions))]
pub fn flush_profiling(_profiling: ResMut<ProfilingBuffer>, _time: Res<Time>) {}