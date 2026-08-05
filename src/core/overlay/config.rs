use crate::core::config::FromTomlValue;

#[derive(Debug, Clone)]
pub struct DefaultOverlayPreset {
    pub tags: Vec<String>,
}

impl FromTomlValue for DefaultOverlayPreset {
    fn from_toml_value(value: &toml::Value) -> Self {
        let table = value.as_table().unwrap_or_else(|| {
            panic!("Expected table for [default], got {:?}", value)
        });
        Self {
            tags: table
                .get("tags")
                .map(Vec::<String>::from_toml_value)
                .unwrap_or_else(|| panic!("Missing 'tags' in [default]")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OverlayPreset {
    pub name: String,
    pub tags: Vec<String>,
}

impl FromTomlValue for OverlayPreset {
    fn from_toml_value(value: &toml::Value) -> Self {
        let table = value.as_table().unwrap_or_else(|| {
            panic!("Expected table for OverlayPreset, got {:?}", value)
        });
        Self {
            name: table
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or_else(|| panic!("Missing 'name' in OverlayPreset"))
                .to_string(),
            tags: table
                .get("tags")
                .map(Vec::<String>::from_toml_value)
                .unwrap_or_else(|| panic!("Missing 'tags' in OverlayPreset")),
        }
    }
}

#[cfg(debug_assertions)]
#[derive(Clone)]
pub struct OverlayConfig {
    pub active_tags: Vec<String>,
}
