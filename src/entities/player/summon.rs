// src/entities/player/summon.rs
use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::components::core::GameLayer;
use crate::core::{make_spritesheet, extensions::EntityBuilderExt, animation::create_animation};
use crate::components::{markers::Player, behavior::FollowPlayer, core::DepthLayer};
use crate::core::debug_log::DebugLogBuffer;
use crate::core::config::from_toml;
use crate::entities::player::state::MovingDirection;
use crate::modules::health::{Health, Resistances, DamageType, spawn_health_bar};

use super::state::{
    PlayerAnimation, PlayerStateHandler, CAMERA_FOLLOW_SMOOTHNESS, TILE_SIZE,
    PLAYER_COLLIDER_HALF_WIDTH, PLAYER_COLLIDER_HALF_HEIGHT,
    PLAYER_COLLIDER_OFFSET_X, PLAYER_COLLIDER_OFFSET_Y,
    PlayerLogicFlags, SPRITESHEET, ANIMATIONS
};

from_toml!("config/player.toml", [
    DEFAULT_HEALTH_MAX: f32 = "health.max",
    DEFAULT_MAGICAL_RESISTANCE: f32 = "health.magical_resistance",
]);

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
        FollowPlayer { smoothness: *CAMERA_FOLLOW_SMOOTHNESS }
    ));

    let ss = &*SPRITESHEET;
    let tile_size = *TILE_SIZE;
    let (spritesheet, sprite) = make_spritesheet(
        &asset_server, &mut atlas_layouts,
        ss.path.clone(), ss.columns, ss.rows, tile_size
    );

    let anim_configs = &*ANIMATIONS;
    let walk_config = anim_configs.iter().find(|c| c.name == "walk").expect("Missing 'walk' animation");
    let idle_config = anim_configs.iter().find(|c| c.name == "idle").expect("Missing 'idle' animation");
    
    let walk_handler = create_animation(&spritesheet, &mut animations, walk_config);
    let idle_handler = create_animation(&spritesheet, &mut animations, idle_config);

    let mut resistances = Resistances::new();
    resistances.set(DamageType::Magical, *DEFAULT_MAGICAL_RESISTANCE);

    let player_entity = commands.spawn((
        sprite,
        SpritesheetAnimation::new(idle_handler.clone()),
        PlayerAnimation { idle: idle_handler, walk: walk_handler },
        PlayerStateHandler::default(),
        Player,
        PlayerLogicFlags::default(),
        MovingDirection::default(),
        Health::new(*DEFAULT_HEALTH_MAX),
        resistances,
    ))
    .at(0, 0, DepthLayer::Entities(0))
    .as_dynamic_body()
    .use_depth_ordered_draw()
    .with_oval_collider(
        *PLAYER_COLLIDER_HALF_WIDTH,
        *PLAYER_COLLIDER_HALF_HEIGHT,
        *PLAYER_COLLIDER_OFFSET_X,
        *PLAYER_COLLIDER_OFFSET_Y,
        GameLayer::DynamicBody,
        [GameLayer::World, GameLayer::DynamicBody],
    )
    .id();
    
    spawn_health_bar(&mut commands, player_entity);
    
    debug_log.add("✅ Player spawned");
}