// src/core/dto.rs
use bevy::prelude::*;
use crate::core::config::FromTomlValue;

#[derive(Debug, Clone)]
pub struct AnimationConfig {
    pub name: String,
    pub frames: Vec<usize>,
    pub duration_ms: u64,
    pub loop_: bool,
    pub ping_pong: bool,
}

impl FromTomlValue for AnimationConfig {
    fn from_toml_value(value: &toml::Value) -> Self {
        let table = value.as_table().unwrap_or_else(|| panic!("Expected table for AnimationConfig, got {:?}", value));
        Self {
            name: table.get("name").and_then(|v| v.as_str()).unwrap_or_else(|| panic!("Missing 'name' in AnimationConfig")).to_string(),
            frames: table.get("frames").map(|v| Vec::<usize>::from_toml_value(v)).unwrap_or_else(|| panic!("Missing 'frames' in AnimationConfig")),
            duration_ms: table.get("duration_ms").and_then(|v| v.as_integer()).unwrap_or_else(|| panic!("Missing 'duration_ms' in AnimationConfig")) as u64,
            loop_: table.get("loop").and_then(|v| v.as_bool()).unwrap_or_else(|| panic!("Missing 'loop' in AnimationConfig")),
            ping_pong: table.get("ping_pong").and_then(|v| v.as_bool()).unwrap_or(false),
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
    pub image_width: u32,
    pub image_height: u32,
    pub size_x: f32,
    pub size_y: f32,
}

impl FromTomlValue for SpriteSheetConfig {
    fn from_toml_value(value: &toml::Value) -> Self {
        let table = value.as_table().unwrap_or_else(|| panic!("Expected table for SpriteSheetConfig, got {:?}", value));
        Self {
            path: table.get("path").and_then(|v| v.as_str()).unwrap_or_else(|| panic!("Missing 'path' in SpriteSheetConfig")).to_string(),
            columns: table.get("columns").and_then(|v| v.as_integer()).unwrap_or_else(|| panic!("Missing 'columns' in SpriteSheetConfig")) as usize,
            rows: table.get("rows").and_then(|v| v.as_integer()).unwrap_or_else(|| panic!("Missing 'rows' in SpriteSheetConfig")) as usize,
            image_width: table.get("image_width").and_then(|v| v.as_integer()).unwrap_or_else(|| panic!("Missing 'image_width' in SpriteSheetConfig")) as u32,
            image_height: table.get("image_height").and_then(|v| v.as_integer()).unwrap_or_else(|| panic!("Missing 'image_height' in SpriteSheetConfig")) as u32,
            size_x: table.get("size_x").and_then(|v| v.as_float()).unwrap_or_else(|| panic!("Missing 'size_x' in SpriteSheetConfig")) as f32,
            size_y: table.get("size_y").and_then(|v| v.as_float()).unwrap_or_else(|| panic!("Missing 'size_y' in SpriteSheetConfig")) as f32,
        }
    }
}