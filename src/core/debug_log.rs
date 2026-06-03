use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Resource, Default)]
pub struct DebugLogBuffer {
    pub messages: HashSet<String>,
    pub timer: f32,
    pub enabled: bool,
}

impl DebugLogBuffer {
    pub fn add(&mut self, msg: impl Into<String>) {
        if self.enabled {
            self.messages.insert(msg.into());
        }
    }
}

pub fn flush_debug_logs(
    mut buffer: ResMut<DebugLogBuffer>,
    time: Res<Time>,
) {
    if !buffer.enabled {
        return;
    }

    buffer.timer += time.delta_secs();
    
    if buffer.timer >= 1.0 {
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
            println!("");
        }
        
        buffer.messages.clear();
    }
}