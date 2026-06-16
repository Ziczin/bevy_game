pub const TILE_SIZE: f32 = 16.0;
pub const GRID_WIDTH: usize = 128;
pub const GRID_HEIGHT: usize = 128;
pub const TILE_TEXTURE_PATH: &str = "textures/ground/tile.png";

pub const FENCE_TEXTURE_PATH: &str = "textures/ground/fence_tilemap.png";
pub const FENCE_ATLAS_COLS: usize = 4;
pub const FENCE_ATLAS_ROWS: usize = 4;
pub const FENCE_IMAGE_WIDTH: u32 = 64;
pub const FENCE_IMAGE_HEIGHT: u32 = 64;
pub const FENCE_SPRITE_SIZE_X: f32 = 16.0;
pub const FENCE_SPRITE_SIZE_Y: f32 = 16.0;

pub const FENCE_COLLIDER_WIDTH: i32 = 14;
pub const FENCE_COLLIDER_HEIGHT: i32 = 1;
pub const FENCE_COLLIDER_OFFSET_X: i32 = 0;
pub const FENCE_COLLIDER_OFFSET_Y: i32 = -6;

pub struct FenceSegment {
    pub x: i32,
    pub y: i32,
    pub variant: usize,
}

pub const FENCES_TO_SPAWN: [FenceSegment; 16] = [
    FenceSegment { x: -4, y: 1, variant: 0 },
    FenceSegment { x: -3, y: 1, variant: 1 },
    FenceSegment { x: -2, y: 1, variant: 2 },
    FenceSegment { x: -1, y: 1, variant: 3 },

    FenceSegment { x: 1, y: 1, variant: 4 },
    FenceSegment { x: 2, y: 1, variant: 4 },
    FenceSegment { x: 3, y: 1, variant: 5 },
    FenceSegment { x: 4, y: 1, variant: 6 },

    FenceSegment { x: -4, y: 4, variant: 8 },
    FenceSegment { x: -3, y: 4, variant: 7 },
    FenceSegment { x: -2, y: 4, variant: 10 },
    FenceSegment { x: -1, y: 4, variant: 11 },

    FenceSegment { x: 1, y: 4, variant: 12 },
    FenceSegment { x: 2, y: 4, variant: 13 },
    FenceSegment { x: 3, y: 4, variant: 14 },
    FenceSegment { x: 4, y: 4, variant: 15 },
];

// === Дорожки (авто-тайлинг по соседству Мура) ===
pub const PATH_TEXTURE_PATH: &str = "textures/ground/path_tilemap.png";
pub const PATH_ATLAS_COLS: usize = 12;
pub const PATH_ATLAS_ROWS: usize = 4;
pub const PATH_IMAGE_WIDTH: u32 = 196;
pub const PATH_IMAGE_HEIGHT: u32 = 64;
pub const PATH_TILE_SIZE: f32 = 16.0;