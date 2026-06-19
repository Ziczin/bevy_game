// src/core/config.rs
use std::collections::HashMap;
use std::fs;
use std::sync::{OnceLock, RwLock};
use bevy::math::Vec2;

static CONFIG_CACHE: OnceLock<RwLock<HashMap<&'static str, toml::Table>>> = OnceLock::new();

fn cache() -> &'static RwLock<HashMap<&'static str, toml::Table>> {
    CONFIG_CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

pub struct TomlConfig;

impl TomlConfig {
    pub fn get<T: FromToml>(path: &'static str, key: &str) -> T {
        {
            let guard = cache().read().unwrap();
            if let Some(table) = guard.get(path) {
                if let Some(value) = table.get(key) {
                    return T::from_toml(value);
                }
            }
        }
        
        let mut guard = cache().write().unwrap();
        if !guard.contains_key(path) {
            let content = fs::read_to_string(path)
                .unwrap_or_else(|e| panic!("Failed to read TOML {}: {}", path, e));
            let table: toml::Table = toml::from_str(&content)
                .unwrap_or_else(|e| panic!("Failed to parse TOML {}: {}", path, e));
            guard.insert(path, table);
        }
        
        let table = guard.get(path).unwrap();
        let value = table.get(key)
            .unwrap_or_else(|| panic!("Key '{}' not found in {}", key, path));
        T::from_toml(value)
    }
}

pub trait FromToml: Sized {
    fn from_toml(value: &toml::Value) -> Self;
}

impl FromToml for f32 {
    fn from_toml(value: &toml::Value) -> Self {
        value.as_float().unwrap_or_else(|| panic!("Expected f32, got {:?}", value)) as f32
    }
}

impl FromToml for i32 {
    fn from_toml(value: &toml::Value) -> Self {
        value.as_integer().unwrap_or_else(|| panic!("Expected i32, got {:?}", value)) as i32
    }
}

impl FromToml for bool {
    fn from_toml(value: &toml::Value) -> Self {
        value.as_bool().unwrap_or_else(|| panic!("Expected bool, got {:?}", value))
    }
}

impl FromToml for String {
    fn from_toml(value: &toml::Value) -> Self {
        value.as_str().unwrap_or_else(|| panic!("Expected String, got {:?}", value)).to_string()
    }
}

impl FromToml for Vec2 {
    fn from_toml(value: &toml::Value) -> Self {
        let arr = value.as_array().unwrap_or_else(|| panic!("Expected array for Vec2, got {:?}", value));
        if arr.len() != 2 {
            panic!("Expected array of 2 elements for Vec2, got {}", arr.len());
        }
        let x = arr[0].as_float().unwrap_or_else(|| panic!("Expected f32 for Vec2.x")) as f32;
        let y = arr[1].as_float().unwrap_or_else(|| panic!("Expected f32 for Vec2.y")) as f32;
        Vec2::new(x, y)
    }
}

#[macro_export]
macro_rules! config {
    ($file:expr, $key:expr) => {
        $crate::core::config::TomlConfig::get::<_>(
            concat!(env!("CARGO_MANIFEST_DIR"), "/", $file), 
            $key
        )
    };
}