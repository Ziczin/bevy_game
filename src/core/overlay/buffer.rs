use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct DebugOverlay {
    entries: HashMap<String, (String, Vec<String>)>,
}

impl DebugOverlay {
    /// Добавляет или обновляет запись с указанными тегами.
    pub fn set_with_tags(&mut self, key: impl Into<String>, value: impl ToString, tags: &[&str]) {
        let tags: Vec<String> = tags.iter().map(|s| s.to_string()).collect();
        self.entries.insert(key.into(), (value.to_string(), tags));
    }

    /// Удаляет запись.
    pub fn remove(&mut self, key: &str) {
        self.entries.remove(key);
    }

    /// Очищает все записи.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Возвращает отформатированную строку для отображения, фильтруя по активным тегам.
    pub fn format_filtered(&self, active_tags: &[String]) -> String {
        let mut lines: Vec<String> = Vec::new();
        for (key, (value, tags)) in self.entries.iter() {
            // Если активные теги пусты – показываем всё.
            if active_tags.is_empty() {
                lines.push(format!("{}: {}", key, value));
                continue;
            }
            // Проверяем, есть ли пересечение тегов записи с активными.
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
