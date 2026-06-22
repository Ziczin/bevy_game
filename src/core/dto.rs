// src/core/dto.rs
use bevy::prelude::*;
use crate::core::config::FromTomlValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationMode {
    Once,
    Loop,
    PingPong,
}

impl FromTomlValue for AnimationMode {
    fn from_toml_value(value: &toml::Value) -> Self {
        match value.as_str().unwrap_or_else(|| panic!("Expected string for AnimationMode, got {:?}", value)) {
            "once" => AnimationMode::Once,
            "loop" => AnimationMode::Loop,
            "ping_pong" => AnimationMode::PingPong,
            other => panic!("Unknown AnimationMode: '{}'. Expected 'once', 'loop', or 'ping_pong'", other),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnimationConfig {
    pub name: String,
    pub frames: Vec<usize>,
    pub duration_ms: u64,
    pub mode: AnimationMode,
}

impl FromTomlValue for AnimationConfig {
    fn from_toml_value(value: &toml::Value) -> Self {
        let table = value.as_table().unwrap_or_else(|| panic!("Expected table for AnimationConfig, got {:?}", value));
        Self {
            name: table.get("name").and_then(|v| v.as_str()).unwrap_or_else(|| panic!("Missing 'name' in AnimationConfig")).to_string(),
            frames: table.get("frames").map(|v| Vec::<usize>::from_toml_value(v)).unwrap_or_else(|| panic!("Missing 'frames' in AnimationConfig")),
            duration_ms: table.get("duration_ms").and_then(|v| v.as_integer()).unwrap_or_else(|| panic!("Missing 'duration_ms' in AnimationConfig")) as u64,
            mode: table.get("mode").map(|v| AnimationMode::from_toml_value(v)).unwrap_or(AnimationMode::Once),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FenceSegment {
    pub x: i32,
    pub y: i32,
    pub variant: usize,
}

impl FromTomlValue for FenceSegment {
    fn from_toml_value(value: &toml::Value) -> Self {
        let table = value.as_table().unwrap_or_else(|| panic!("Expected table for FenceSegment, got {:?}", value));
        Self {
            x: table.get("x").and_then(|v| v.as_integer()).unwrap_or_else(|| panic!("Missing 'x' in FenceSegment")) as i32,
            y: table.get("y").and_then(|v| v.as_integer()).unwrap_or_else(|| panic!("Missing 'y' in FenceSegment")) as i32,
            variant: table.get("variant").and_then(|v| v.as_integer()).unwrap_or_else(|| panic!("Missing 'variant' in FenceSegment")) as usize,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpriteSheetConfig {
    pub path: String,
    pub columns: usize,
    pub rows: usize,
}

impl FromTomlValue for SpriteSheetConfig {
    fn from_toml_value(value: &toml::Value) -> Self {
        let table = value.as_table().unwrap_or_else(|| panic!("Expected table for SpriteSheetConfig, got {:?}", value));
        Self {
            path: table.get("path").and_then(|v| v.as_str()).unwrap_or_else(|| panic!("Missing 'path' in SpriteSheetConfig")).to_string(),
            columns: table.get("columns").and_then(|v| v.as_integer()).unwrap_or_else(|| panic!("Missing 'columns' in SpriteSheetConfig")) as usize,
            rows: table.get("rows").and_then(|v| v.as_integer()).unwrap_or_else(|| panic!("Missing 'rows' in SpriteSheetConfig")) as usize,
        }
    }
}