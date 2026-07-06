use bevy::prelude::*;
use crate::components::markers::Tile;
use crate::components::core::{DepthLayer, GameLayer};
use crate::core::{make_spritesheet, extensions::EntityBuilderExt};
use crate::core::profiling::{ProfilingBuffer, ProfileScope};
use super::state::{
    FENCE_TEXTURE_PATH, FENCE_ATLAS_COLS, FENCE_ATLAS_ROWS,
    FENCE_COLLIDER_WIDTH, FENCE_COLLIDER_HEIGHT, FENCE_COLLIDER_OFFSET_X, FENCE_COLLIDER_OFFSET_Y,
    FENCES, TILE_SIZE,
};

pub fn spawn_fences(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    profiling: Res<ProfilingBuffer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let _scope = ProfileScope::new(&profiling, "entities::playground::spawn_fences::spawn_fences", &["playground", "setup", "spawn", "fence", "collision"]);
    
    let tile_size = *TILE_SIZE;

    let (_spritesheet, sprite_template) = make_spritesheet(
        &asset_server, &mut atlas_layouts,
        FENCE_TEXTURE_PATH.to_string(),
        *FENCE_ATLAS_COLS, *FENCE_ATLAS_ROWS,
        tile_size
    );

    for fence in FENCES.iter() {
        let mut sprite = sprite_template.clone();
        sprite.texture_atlas.as_mut().unwrap().index = fence.variant;

        commands.spawn((sprite, Tile))
            .at(fence.x * tile_size as i32, fence.y * tile_size as i32, DepthLayer::Entities(0))
            .into_static_body()
            .use_depth_ordered_draw_once()
            .with_rect_collider(
                *FENCE_COLLIDER_WIDTH,
                *FENCE_COLLIDER_HEIGHT,
                *FENCE_COLLIDER_OFFSET_X,
                *FENCE_COLLIDER_OFFSET_Y,
                [GameLayer::World, GameLayer::VisionBlock],
                [GameLayer::DynamicBody, GameLayer::Projectile],
            );
    }
}