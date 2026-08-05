use bevy::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct ProfileEntry {
    pub name: String,
    pub tags: Vec<String>,
    pub total_time: u128,
    pub call_count: u64,
    pub min_time: u128,
    pub max_time: u128,
}

#[cfg(debug_assertions)]
#[derive(Resource)]
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
    pub frame_count: u64,
    pub total_frame_time: u128,
    pub min_frame_time: u128,
    pub max_frame_time: u128,
    pub frame_logic_time: Mutex<u128>,
    pub total_logic_time_per_frame: Mutex<u128>,
    pub min_logic_time_per_frame: Mutex<u128>,
    pub max_logic_time_per_frame: Mutex<u128>,
}

#[cfg(debug_assertions)]
impl Default for ProfilingBuffer {
    fn default() -> Self {
        Self {
            enabled: false,
            include_tags: Vec::new(),
            exclude_tags: Vec::new(),
            include_functions: Vec::new(),
            exclude_functions: Vec::new(),
            strict: false,
            interval: 1.0,
            timer: 0.0,
            entries: Mutex::new(HashMap::new()),
            frame_start: None,
            last_frame_time: 0,
            frame_count: 0,
            total_frame_time: 0,
            min_frame_time: u128::MAX,
            max_frame_time: 0,
            frame_logic_time: Mutex::new(0),
            total_logic_time_per_frame: Mutex::new(0),
            min_logic_time_per_frame: Mutex::new(u128::MAX),
            max_logic_time_per_frame: Mutex::new(0),
        }
    }
}

#[cfg(not(debug_assertions))]
#[derive(Resource, Default)]
pub struct ProfilingBuffer;

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
                min_time: u128::MAX,
                max_time: 0,
            });
            entry.total_time += elapsed;
            entry.call_count += 1;
            if elapsed < entry.min_time {
                entry.min_time = elapsed;
            }
            if elapsed > entry.max_time {
                entry.max_time = elapsed;
            }
            drop(entries);
            let mut frame_logic = self.buffer.frame_logic_time.lock().unwrap();
            *frame_logic += elapsed;
        }
    }
}

#[cfg(debug_assertions)]
macro_rules! profile_scope {
    ($buffer:expr, $name:expr, $tags:expr) => {
        let _scope = $crate::core::profiling::ProfileScope::new($buffer, $name, $tags);
    };
}

#[cfg(not(debug_assertions))]
macro_rules! profile_scope {
    ($buffer:expr, $name:expr, $tags:expr) => {};
}
pub(crate) use profile_scope;

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
            let frame_duration = last_start.elapsed().as_micros();
            self.last_frame_time = frame_duration;
            self.frame_count += 1;
            self.total_frame_time += frame_duration;
            if frame_duration < self.min_frame_time {
                self.min_frame_time = frame_duration;
            }
            if frame_duration > self.max_frame_time {
                self.max_frame_time = frame_duration;
            }
            let logic_time = {
                let mut lock = self.frame_logic_time.lock().unwrap();
                let val = *lock;
                *lock = 0;
                val
            };
            if logic_time > 0 {
                let mut total = self.total_logic_time_per_frame.lock().unwrap();
                *total += logic_time;
                let mut min = self.min_logic_time_per_frame.lock().unwrap();
                if logic_time < *min {
                    *min = logic_time;
                }
                let mut max = self.max_logic_time_per_frame.lock().unwrap();
                if logic_time > *max {
                    *max = logic_time;
                }
            }
        }
        self.frame_start = Some(Instant::now());
    }
}
