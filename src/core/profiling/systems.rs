use bevy::prelude::*;
use super::buffer::ProfilingBuffer;

#[cfg(debug_assertions)]
pub fn update_frame_start(mut profiling: ResMut<ProfilingBuffer>) {
    profiling.update_frame_start();
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
            let interval_secs = p.interval as f64;
            let frame_count = p.frame_count;
            let total_frame_time = p.total_frame_time;
            let avg_frame_time = if frame_count > 0 {
                total_frame_time as f64 / frame_count as f64
            } else {
                0.0
            };
            let min_frame_time = if frame_count > 0 { p.min_frame_time } else { 0 };
            let max_frame_time = if frame_count > 0 { p.max_frame_time } else { 0 };
            let total_logic_time_sum = *p.total_logic_time_per_frame.lock().unwrap();
            let min_logic_time = *p.min_logic_time_per_frame.lock().unwrap();
            let max_logic_time = *p.max_logic_time_per_frame.lock().unwrap();
            let avg_logic_time = if frame_count > 0 {
                total_logic_time_sum as f64 / frame_count as f64
            } else {
                0.0
            };
            let total_logic_time_all: u128 = entries.values().map(|e| e.total_time).sum();
            let mut sorted_entries: Vec<_> = entries.values().collect();
            sorted_entries.sort_by(|a, b| b.total_time.cmp(&a.total_time));
            let headers = vec![
                "Function",
                "total",
                "calls",
                "cpi",
                "min",
                "max",
                "avg",
                "%logic",
                "%frame",
                "%CPU",
            ];
            let mut rows: Vec<Vec<String>> = Vec::new();
            for entry in &sorted_entries {
                let total = entry.total_time;
                let calls = entry.call_count;
                let avg = if calls > 0 { total as f64 / calls as f64 } else { 0.0 };
                let min_time = if calls > 0 { entry.min_time } else { 0 };
                let max_time = if calls > 0 { entry.max_time } else { 0 };
                let cpi = if interval_secs > 0.0 {
                    calls as f64 / interval_secs
                } else {
                    0.0
                };
                let cpu_percent = if interval_secs > 0.0 {
                    (total as f64 / (interval_secs * 1_000_000.0)) * 100.0
                } else {
                    0.0
                };
                let logic_percent = if total_logic_time_all > 0 {
                    (total as f64 / total_logic_time_all as f64) * 100.0
                } else {
                    0.0
                };
                let frame_percent = if total_frame_time > 0 {
                    (total as f64 / total_frame_time as f64) * 100.0
                } else {
                    0.0
                };
                rows.push(vec![
                    entry.name.clone(),
                    total.to_string(),
                    calls.to_string(),
                    format!("{:.0}", cpi),
                    format_time_value(min_time as f64),
                    format_time_value(max_time as f64),
                    format_time_value(avg),
                    format!("{:.1}%", logic_percent),
                    format!("{:.1}%", frame_percent),
                    format!("{:.3}%", cpu_percent),
                ]);
            }
            let mut col_widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
            for row in &rows {
                for (i, cell) in row.iter().enumerate() {
                    if cell.len() > col_widths[i] {
                        col_widths[i] = cell.len();
                    }
                }
            }
            println!("\n=== Profiling Report ===");
            println!("Interval: {:.2}s", interval_secs);
            println!("Frames: {}", frame_count);
            if frame_count > 0 {
                let min_frame_ms = min_frame_time as f64 / 1000.0;
                let max_frame_ms = max_frame_time as f64 / 1000.0;
                println!("Frame time: avg {:.2}ms, min {:.2}ms, max {:.2}ms",
                    avg_frame_time / 1000.0,
                    min_frame_ms,
                    max_frame_ms
                );
                let min_logic_ms = min_logic_time as f64 / 1000.0;
                let max_logic_ms = max_logic_time as f64 / 1000.0;
                println!("Logic time per frame: avg {:.2}ms, min {:.2}ms, max {:.2}ms",
                    avg_logic_time / 1000.0,
                    min_logic_ms,
                    max_logic_ms
                );
            }
            println!();
            let header_parts: Vec<String> = headers.iter()
                .enumerate()
                .map(|(i, &h)| {
                    if i == 0 {
                        format!("{:<width$}", h, width = col_widths[i])
                    } else {
                        format!("{:>width$}", h, width = col_widths[i])
                    }
                })
                .collect();
            let header_line = header_parts.join(" | ");
            println!("{}", header_line);
            println!("{}", "-".repeat(header_line.len()));
            for row in rows {
                let row_parts: Vec<String> = row.iter()
                    .enumerate()
                    .map(|(i, cell)| {
                        if i == 0 {
                            format!("{:<width$}", cell, width = col_widths[i])
                        } else {
                            format!("{:>width$}", cell, width = col_widths[i])
                        }
                    })
                    .collect();
                let row_line = row_parts.join(" | ");
                println!("{}", row_line);
            }
            println!();
            entries.clear();
            p.frame_count = 0;
            p.total_frame_time = 0;
            p.min_frame_time = u128::MAX;
            p.max_frame_time = 0;
            {
                let mut total = p.total_logic_time_per_frame.lock().unwrap();
                *total = 0;
                let mut min = p.min_logic_time_per_frame.lock().unwrap();
                *min = u128::MAX;
                let mut max = p.max_logic_time_per_frame.lock().unwrap();
                *max = 0;
                let mut curr = p.frame_logic_time.lock().unwrap();
                *curr = 0;
            }
        }
    }
}

fn format_time_value(val: f64) -> String {
    if val < 1.0 && val > 0.0 {
        "<1".to_string()
    } else {
        format!("{:.0}", val)
    }
}

#[cfg(not(debug_assertions))]
pub fn flush_profiling(_profiling: ResMut<ProfilingBuffer>, _time: Res<Time>) {}
