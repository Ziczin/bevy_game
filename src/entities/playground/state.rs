// src/entities/playground/state.rs
use crate::core::config::from_toml;
use crate::core::dto::FenceSegment;

from_toml!("config/playground.toml", [
    TILE_SIZE: f32 = "tile.size",
    GRID_WIDTH: usize = "tile.grid_width",
    GRID_HEIGHT: usize = "tile.grid_height",
    FENCE_ATLAS_COLS: usize = "fence.atlas_cols",
    FENCE_ATLAS_ROWS: usize = "fence.atlas_rows",
    FENCE_IMAGE_WIDTH: u32 = "fence.image_width",
    FENCE_IMAGE_HEIGHT: u32 = "fence.image_height",
    FENCE_SPRITE_SIZE_X: f32 = "fence.sprite_size_x",
    FENCE_SPRITE_SIZE_Y: f32 = "fence.sprite_size_y",
    FENCE_COLLIDER_WIDTH: i32 = "fence.collider_width",
    FENCE_COLLIDER_HEIGHT: i32 = "fence.collider_height",
    FENCE_COLLIDER_OFFSET_X: i32 = "fence.collider_offset_x",
    FENCE_COLLIDER_OFFSET_Y: i32 = "fence.collider_offset_y",
    FENCES: Vec<FenceSegment> = "fence.fences",
    PATH_ATLAS_COLS: usize = "path.atlas_cols",
    PATH_ATLAS_ROWS: usize = "path.atlas_rows",
    PATH_TILE_SIZE: f32 = "path.tile_size",
]);

pub const TILE_TEXTURE_PATH: &str = "textures/ground/tile.png";
pub const FENCE_TEXTURE_PATH: &str = "textures/ground/fence_tilemap.png";
pub const PATH_TEXTURE_PATH: &str = "textures/ground/path_tilemap.png";