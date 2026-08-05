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

#[cfg(debug_assertions)]
#[derive(Clone)]
pub struct DebugProfilingConfig {
    pub enabled: bool,
    pub include_tags: Vec<String>,
    pub exclude_tags: Vec<String>,
    pub include_functions: Vec<String>,
    pub exclude_functions: Vec<String>,
    pub strict: bool,
    pub interval: f32,
}
