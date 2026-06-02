use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::components::core::{DepthLayer, GameLayer};
use crate::core::extensions::EntityBuilderExt;
use crate::core::make_spritesheet;
use super::state::{RedSlimeAnimation, RedSlimeStateHandler};
use super::animation::{create_idle_animation, create_walk_animation};

pub fn summon(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut animations: ResMut<Assets<Animation>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {

    let (spritesheet, sprite) = make_spritesheet(
        &asset_server, &mut atlas_layouts,
        "textures/red_slime/map.png",
        8, 1, 128, 16, 64.0, 64.0
    );

    let idle_handler = create_idle_animation(&spritesheet, &mut animations);
    let walk_handler = create_walk_animation(&spritesheet, &mut animations);

    commands.spawn((
        sprite,
        SpritesheetAnimation::new(idle_handler.clone()),
        RedSlimeAnimation { idle: idle_handler, walk: walk_handler, },
        RedSlimeStateHandler::default(),
    ))
    .at(200, 0, DepthLayer::Entities(0))
    .as_dynamic_body()
    .use_depth_ordered_draw()
    .with_collider(
        32, 24, 0, -16,
        GameLayer::DynamicBody,
        [GameLayer::World, GameLayer::DynamicBody],
    );
}