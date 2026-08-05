use bevy::prelude::*;
use super::buffer::DebugLogBuffer;

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
