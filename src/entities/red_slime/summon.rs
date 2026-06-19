use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use avian2d::prelude::*;

use crate::components::core::{DepthLayer, GameLayer};
use crate::components::pathfinding::Pathfinder;
use crate::core::extensions::EntityBuilderExt;
use crate::core::make_spritesheet;
use crate::core::debug_log::DebugLogBuffer;
use crate::entities::red_slime::state::RedSlimeLogicFlags;
use super::state::{
    RedSlimeAnimation, RedSlimeStateHandler, MovingDirection,
    WALK_DISTANCE_END, PATHFINDER_UPDATE_INTERVAL, SPRITE_SIZE_MULTIPLIER_X,
    SPRITE_SIZE_MULTIPLIER_Y, COLLIDER_PADDING, COLLIDER_OFFSET_X, COLLIDER_OFFSET_Y,
};
use super::animation::{create_idle_animation, create_walk_animation};

pub fn spawn_single_red_slime(
    commands: &mut Commands,
    x: i32,
    y: i32,
    sprite_template: &Sprite,
    idle_anim: Handle<Animation>,
    walk_anim: Handle<Animation>,
    debug_log: &mut DebugLogBuffer,
    size: i32,
) {
    let sprite = sprite_template.clone();
    
    let width = size * SPRITE_SIZE_MULTIPLIER_X;
    let height = size * SPRITE_SIZE_MULTIPLIER_Y;

    let agent_half_size = Vec2::new(width as f32 + COLLIDER_PADDING, height as f32 + COLLIDER_PADDING);
    
    let entity = commands.spawn((
        sprite,
        SpritesheetAnimation::new(idle_anim.clone()),
        RedSlimeAnimation { 
            idle: idle_anim, 
            walk: walk_anim, 
        },
        RedSlimeStateHandler::default(),
        Pathfinder::new(PATHFINDER_UPDATE_INTERVAL, agent_half_size, WALK_DISTANCE_END),
        LinearVelocity::default(),
        MovingDirection::default(),
        RedSlimeLogicFlags::default(),
    ))
    .at(x, y, DepthLayer::Entities(0))
    .as_dynamic_body()
    .use_depth_ordered_draw()
    .with_oval_collider(
        width, height,
        COLLIDER_OFFSET_X, COLLIDER_OFFSET_Y,
        GameLayer::DynamicBody,
        [GameLayer::World, GameLayer::DynamicBody],
    )
    .id();

    debug_log.add(format!("✅ RedSlime spawned at ({}, {}) with entity ID: {:?}", x, y, entity));
}

pub fn summon(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut animations: ResMut<Assets<Animation>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut debug_log: ResMut<DebugLogBuffer>,
) {
    debug_log.add("🎬 RedSlime summon started");

    let (spritesheet, sprite_template) = make_spritesheet(
        &asset_server, &mut atlas_layouts,
        "textures/red_slime/red_slime_tilemap.png",
        8, 1, 128, 16, 16.0, 16.0
    );

    let idle_handler = create_idle_animation(&spritesheet, &mut animations);
    let walk_handler = create_walk_animation(&spritesheet, &mut animations);

    spawn_single_red_slime(
        &mut commands,
        50, 0,
        &sprite_template,
        idle_handler.clone(),
        walk_handler.clone(),
        &mut debug_log,
        1
    );

    spawn_single_red_slime(
        &mut commands,
        -50, 25,
        &sprite_template,
        idle_handler.clone(),
        walk_handler.clone(),
        &mut debug_log,
        1
    );

    spawn_single_red_slime(
        &mut commands,
        0, -38,
        &sprite_template,
        idle_handler.clone(),
        walk_handler.clone(),
        &mut debug_log,
        1
    );

    spawn_single_red_slime(
        &mut commands,
        0, -62,
        &sprite_template,
        idle_handler.clone(),
        walk_handler.clone(),
        &mut debug_log,
        1
    );

    debug_log.add("✅ All RedSlimes spawned successfully");
}