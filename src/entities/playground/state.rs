// src/entities/playground/state.rs
use crate::core::config::from_toml;
use crate::core::dto::FenceSegment;

from_toml!("config/playground.toml", [
    GRID_WIDTH: usize = "tile.grid_width",
    GRID_HEIGHT: usize = "tile.grid_height",
    FENCE_ATLAS_COLS: usize = "fence.atlas_cols",
    FENCE_ATLAS_ROWS: usize = "fence.atlas_rows",
    FENCE_COLLIDER_WIDTH: i32 = "fence.collider_width",
    FENCE_COLLIDER_HEIGHT: i32 = "fence.collider_height",
    FENCE_COLLIDER_OFFSET_X: i32 = "fence.collider_offset_x",
    FENCE_COLLIDER_OFFSET_Y: i32 = "fence.collider_offset_y",
    FENCES: Vec<FenceSegment> = "fence.fences",
    PATH_ATLAS_COLS: usize = "path.atlas_cols",
    PATH_ATLAS_ROWS: usize = "path.atlas_rows",
]);

from_toml!("config/global.toml", [
    TILE_SIZE: f32 = "display.tile_size",
]);

pub const TILE_TEXTURE_PATH: &str = "textures/ground/tile.png";
pub const FENCE_TEXTURE_PATH: &str = "textures/ground/fence_tilemap.png";
pub const PATH_TEXTURE_PATH: &str = "textures/ground/path_tilemap.png";