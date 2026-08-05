use bevy::prelude::*;
use std::collections::HashSet;

#[cfg(debug_assertions)]
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

#[cfg(not(debug_assertions))]
#[derive(Resource, Default)]
pub struct DebugLogBuffer;

#[cfg(debug_assertions)]
impl DebugLogBuffer {
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
}

#[cfg(debug_assertions)]
macro_rules! debug_log {
    ($buffer:expr, $tags:expr, $($arg:tt)*) => {
        $crate::core::debug_log::DebugLogBuffer::add($buffer, $tags, format!($($arg)*))
    };
}

#[cfg(not(debug_assertions))]
macro_rules! debug_log {
    ($buffer:expr, $tags:expr, $($arg:tt)*) => {};
}
pub(crate) use debug_log;
