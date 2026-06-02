use bevy::prelude::*;

use crate::components::markers::Tile;
use crate::{
    components::core::{DepthLayer, GameLayer},
    core::{extensions::EntityBuilderExt, make_spritesheet},
};

pub struct FenceSegment {
    pub x: i32,
    pub y: i32,
    pub variant: usize,
}

pub fn spawn_fences(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let (_spritesheet, sprite_template) = make_spritesheet(
        &asset_server, &mut atlas_layouts,
        "textures/ground/fence_map.png",
        4, 4, 64, 64, 64.0, 64.0
    );

    let fences_to_spawn = vec![
        FenceSegment { x: 1, y: 1, variant: 1 },
        FenceSegment { x: 2, y: 1, variant: 5 },
        FenceSegment { x: 3, y: 1, variant: 10 },
        FenceSegment { x: 4, y: 1, variant: 15 },
        FenceSegment { x: 2, y: 3, variant: 2 },
        FenceSegment { x: 3, y: 3, variant: 11 },
        FenceSegment { x: 4, y: 3, variant: 14 },
        FenceSegment { x: 5, y: 3, variant: 3 },
    ];

    for fence in fences_to_spawn {
        let mut sprite = sprite_template.clone();
        sprite.texture_atlas.as_mut().unwrap().index = fence.variant;

        commands.spawn((sprite, Tile))
            .at(fence.x*64, fence.y*64, DepthLayer::Environment(0))
            .as_static_body()
            .use_depth_ordered_draw_once()
            .with_collider(
                60, 4, -4, -28,
                GameLayer::World,
                [GameLayer::DynamicBody, GameLayer::Projectile],
            );
    }
}