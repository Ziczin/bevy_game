// src/core/config.rs
use std::fs;
use bevy::math::Vec2;

pub trait FromTomlValue: Sized {
    fn from_toml_value(value: &toml::Value) -> Self;
}

impl FromTomlValue for f32 {
    fn from_toml_value(value: &toml::Value) -> Self {
        value.as_float().unwrap_or_else(|| panic!("Expected f32, got {:?}", value)) as f32
    }
}

impl FromTomlValue for i32 {
    fn from_toml_value(value: &toml::Value) -> Self {
        value.as_integer().unwrap_or_else(|| panic!("Expected i32, got {:?}", value)) as i32
    }
}

impl FromTomlValue for u32 {
    fn from_toml_value(value: &toml::Value) -> Self {
        value.as_integer().unwrap_or_else(|| panic!("Expected u32, got {:?}", value)) as u32
    }
}

impl FromTomlValue for u64 {
    fn from_toml_value(value: &toml::Value) -> Self {
        value.as_integer().unwrap_or_else(|| panic!("Expected u64, got {:?}", value)) as u64
    }
}

impl FromTomlValue for usize {
    fn from_toml_value(value: &toml::Value) -> Self {
        value.as_integer().unwrap_or_else(|| panic!("Expected usize, got {:?}", value)) as usize
    }
}

impl FromTomlValue for bool {
    fn from_toml_value(value: &toml::Value) -> Self {
        value.as_bool().unwrap_or_else(|| panic!("Expected bool, got {:?}", value))
    }
}

impl FromTomlValue for String {
    fn from_toml_value(value: &toml::Value) -> Self {
        value.as_str().unwrap_or_else(|| panic!("Expected String, got {:?}", value)).to_string()
    }
}

impl FromTomlValue for Vec2 {
    fn from_toml_value(value: &toml::Value) -> Self {
        let arr = value.as_array().unwrap_or_else(|| panic!("Expected array for Vec2, got {:?}", value));
        Vec2::new(
            arr[0].as_float().unwrap_or_else(|| panic!("Expected f32 for Vec2.x")) as f32,
            arr[1].as_float().unwrap_or_else(|| panic!("Expected f32 for Vec2.y")) as f32,
        )
    }
}

impl<T: FromTomlValue> FromTomlValue for Vec<T> {
    fn from_toml_value(value: &toml::Value) -> Self {
        let arr = value.as_array().unwrap_or_else(|| panic!("Expected array, got {:?}", value));
        arr.iter().map(|v| T::from_toml_value(v)).collect()
    }
}

pub fn read_toml_value<T: FromTomlValue>(path: &str, key_path: &str) -> T {
    let full_path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), path);
    let content = fs::read_to_string(&full_path)
        .unwrap_or_else(|e| panic!("Cannot read {}: {}", full_path, e));
    let table: toml::Table = toml::from_str(&content)
        .unwrap_or_else(|e| panic!("Cannot parse {}: {}", full_path, e));
    
    let parts: Vec<&str> = key_path.split('.').collect();
    let mut current = &table;
    
    for i in 0..parts.len() - 1 {
        current = current.get(parts[i])
            .and_then(|v| v.as_table())
            .unwrap_or_else(|| panic!("Section '{}' not found in {}", parts[i], path));
    }
    
    let value = current.get(parts[parts.len() - 1])
        .unwrap_or_else(|| panic!("Key '{}' not found in {}", key_path, path));
    
    T::from_toml_value(value)
}

macro_rules! from_toml {
    ($file:expr, [$( $name:ident : $ty:ty $(= $key:expr)? ),* $(,)?]) => {
        $(
            from_toml!(@parse $file, $name, $ty $(, $key)?);
        )*
    };
    (@parse $file:expr, $name:ident, $ty:ty, $key:expr) => {
        pub static $name: std::sync::LazyLock<$ty> = std::sync::LazyLock::new(|| {
            $crate::core::config::read_toml_value::<$ty>($file, $key)
        });
    };
    (@parse $file:expr, $name:ident, $ty:ty) => {
        pub static $name: std::sync::LazyLock<$ty> = std::sync::LazyLock::new(|| {
            $crate::core::config::read_toml_value::<$ty>($file, stringify!($name))
        });
    };
}

pub(crate) use from_toml;