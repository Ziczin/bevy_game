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
    // Статистика по кадрам (время полного кадра)
    pub frame_count: u64,
    pub total_frame_time: u128,
    pub min_frame_time: u128,
    pub max_frame_time: u128,
    // Статистика по логике на кадр
    pub frame_logic_time: Mutex<u128>,      // текущее накопление за кадр
    pub total_logic_time_per_frame: Mutex<u128>, // сумма логики за все кадры
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
            // Обновляем запись функции
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
            drop(entries); // освобождаем мьютекс

            // Добавляем время к логике текущего кадра
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

            // Завершаем учёт логики за предыдущий кадр
            let logic_time = {
                let mut lock = self.frame_logic_time.lock().unwrap();
                let val = *lock;
                *lock = 0; // сбрасываем для нового кадра
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

            // Статистика по полному времени кадра
            let total_frame_time = p.total_frame_time;
            let avg_frame_time = if frame_count > 0 {
                total_frame_time as f64 / frame_count as f64
            } else {
                0.0
            };
            let min_frame_time = if frame_count > 0 { p.min_frame_time } else { 0 };
            let max_frame_time = if frame_count > 0 { p.max_frame_time } else { 0 };

            // Статистика по логике на кадр
            let total_logic_time_sum = *p.total_logic_time_per_frame.lock().unwrap();
            let min_logic_time = *p.min_logic_time_per_frame.lock().unwrap();
            let max_logic_time = *p.max_logic_time_per_frame.lock().unwrap();
            let avg_logic_time = if frame_count > 0 {
                total_logic_time_sum as f64 / frame_count as f64
            } else {
                0.0
            };

            // Получаем общее время логики (сумма всех функций) для процентов
            let total_logic_time_all: u128 = entries.values().map(|e| e.total_time).sum();

            let mut sorted_entries: Vec<_> = entries.values().collect();
            sorted_entries.sort_by(|a, b| b.total_time.cmp(&a.total_time));

            // Подготовка данных для таблицы
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
                    format!("{:.0}", cpi),  // целое число
                    format_time_value(min_time as f64),
                    format_time_value(max_time as f64),
                    format_time_value(avg),
                    format!("{:.1}%", logic_percent),
                    format!("{:.1}%", frame_percent),
                    format!("{:.3}%", cpu_percent),
                ]);
            }

            // Вычисляем ширину колонок
            let mut col_widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
            for row in &rows {
                for (i, cell) in row.iter().enumerate() {
                    if cell.len() > col_widths[i] {
                        col_widths[i] = cell.len();
                    }
                }
            }

            // Вывод сводки
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

            // Заголовок таблицы
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

            // Строки таблицы
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

            // Сброс для следующего интервала
            entries.clear();
            p.frame_count = 0;
            p.total_frame_time = 0;
            p.min_frame_time = u128::MAX;
            p.max_frame_time = 0;
            // Сброс логической статистики
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

/// Вспомогательная функция для форматирования времени: если значение < 1, выводим "<1", иначе целое число
fn format_time_value(val: f64) -> String {
    if val < 1.0 && val > 0.0 {
        "<1".to_string()
    } else {
        format!("{:.0}", val)
    }
}

#[cfg(not(debug_assertions))]
pub fn flush_profiling(_profiling: ResMut<ProfilingBuffer>, _time: Res<Time>) {}