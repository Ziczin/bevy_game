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
        Self {
            interval: table.get("interval").and_then(|v| v.as_float()).unwrap_or_else(|| panic!("Missing 'interval' in [default]")) as f32,
            tags: table.get("tags").map(Vec::<String>::from_toml_value).unwrap_or_else(|| panic!("Missing 'tags' in [default]")),
            exclude: table.get("exclude").map(Vec::<String>::from_toml_value).unwrap_or_else(|| panic!("Missing 'exclude' in [default]")),
            strict: table.get("strict").and_then(|v| v.as_bool()).unwrap_or_else(|| panic!("Missing 'strict' in [default]")),
        }
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
        Self {
            name: table.get("name").and_then(|v| v.as_str()).unwrap_or_else(|| panic!("Missing 'name' in LogPreset")).to_string(),
            interval: table.get("interval").and_then(|v| v.as_float()).map(|v| v as f32),
            tags: table.get("tags").map(Vec::<String>::from_toml_value),
            exclude: table.get("exclude").map(Vec::<String>::from_toml_value),
            strict: table.get("strict").and_then(|v| v.as_bool()),
        }
    }
}

#[cfg(debug_assertions)]
#[derive(Clone)]
pub struct DebugLogConfig {
    pub enabled: bool,
    pub active_tags: Vec<String>,
    pub interval: f32,
    pub exclude_tags: Vec<String>,
    pub strict: bool,
}
