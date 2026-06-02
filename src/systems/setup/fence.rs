use bevy::prelude::*;
use crate::{
    components::core::{DepthLayer, GameLayer},
    core::{extensions::EntityBuilderExt, make_spritesheet}
};

pub fn spawn_fences(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    #[allow(unused_variables)]
    let (spritesheet, mut sprite) = make_spritesheet(
        &asset_server, &mut atlas_layouts,
        "textures/ground/fence_map.png",
        4, 4, 64, 64, 64.0, 64.0
    );

    sprite.texture_atlas.as_mut().unwrap().index = 15;

    commands.spawn((sprite,))
    .at(64, 64, DepthLayer::Entities(0))
    .use_depth_ordered_draw()
    .as_static_body()
    .with_collider(
        64, 4, 0, -28,
        GameLayer::World,
        [GameLayer::DynamicBody, GameLayer::Projectile]
    );
}