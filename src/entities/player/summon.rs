use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::components::core::GameLayer;
use crate::core::{make_spritesheet, extensions::EntityBuilderExt};
use crate::components::{markers::Player, behavior::FollowPlayer, core::DepthLayer};
use crate::core::debug_log::DebugLogBuffer;

use super::state::{
    PlayerAnimation, PlayerStateHandler, CAMERA_FOLLOW_SMOOTHNESS,
    PLAYER_COLLIDER_HALF_WIDTH, PLAYER_COLLIDER_HALF_HEIGHT,
    PLAYER_COLLIDER_OFFSET_X, PLAYER_COLLIDER_OFFSET_Y,
};
use super::animation::{create_idle_animation, create_walk_animation};

pub fn summon(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut animations: ResMut<Assets<Animation>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut debug_log: ResMut<DebugLogBuffer>,
) {
    debug_log.add("🎬 Player summon started");
    
    commands.spawn((
        Camera2d,
        Msaa::Off,
        FollowPlayer { smoothness: CAMERA_FOLLOW_SMOOTHNESS }
    ));

    let (spritesheet, sprite) = make_spritesheet(
        &asset_server, &mut atlas_layouts,
        "textures/player/player_tilemap.png",
        8, 1, 128, 16, 16.0, 16.0
    );

    let idle_handler = create_idle_animation(&spritesheet, &mut animations);
    let walk_handler = create_walk_animation(&spritesheet, &mut animations);

    commands.spawn((
        sprite,
        SpritesheetAnimation::new(idle_handler.clone()),
        PlayerAnimation { idle: idle_handler, walk: walk_handler },
        PlayerStateHandler::default(),
        Player,
    ))
    .at(0, 0, DepthLayer::Entities(0))
    .as_dynamic_body()
    .use_depth_ordered_draw()
    .with_oval_collider(
        PLAYER_COLLIDER_HALF_WIDTH,
        PLAYER_COLLIDER_HALF_HEIGHT,
        PLAYER_COLLIDER_OFFSET_X,
        PLAYER_COLLIDER_OFFSET_Y,
        GameLayer::DynamicBody,
        [GameLayer::World, GameLayer::DynamicBody],
    );
    
    debug_log.add("✅ Player spawned");
}