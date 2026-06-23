use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::components::core::GameLayer;
use crate::core::{make_spritesheet, extensions::EntityBuilderExt, animation::create_animation};
use crate::components::{markers::Player, behavior::FollowPlayer, core::DepthLayer};
use crate::core::debug_log::DebugLogBuffer;
use crate::core::config::from_toml;
use crate::entities::player::state::MovingDirection;
use crate::modules::health::{Health, Mana, Resistances, DamageType};
use crate::modules::value_bar::{spawn_value_bar, ValueBarConfig, ValueBarColors};

use super::state::{
    PlayerAnimation, PlayerStateHandler, CAMERA_FOLLOW_SMOOTHNESS, TILE_SIZE,
    PLAYER_COLLIDER_HALF_WIDTH, PLAYER_COLLIDER_HALF_HEIGHT,
    PLAYER_COLLIDER_OFFSET_X, PLAYER_COLLIDER_OFFSET_Y,
    PlayerLogicFlags, SPRITESHEET, ANIMATIONS
};

from_toml!("config/player.toml", [
    DEFAULT_HEALTH_MAX: f32 = "health.max",
    DEFAULT_MAGICAL_RESISTANCE: f32 = "health.magical_resistance",
    DEFAULT_MANA_MAX: f32 = "mana.max",
    HEALTH_BAR_OFFSET_Y: f32 = "health.bar_offset_y",
    MANA_BAR_OFFSET_Y: f32 = "mana.bar_offset_y",
    MANA_BAR_COLOR_BACKGROUND: Vec4 = "mana.bar_color_background",
    MANA_BAR_COLOR_CURRENT: Vec4 = "mana.bar_color_current",
    MANA_BAR_COLOR_DELAYED_DAMAGE: Vec4 = "mana.bar_color_delayed_damage",
    MANA_BAR_COLOR_DELAYED_HEAL: Vec4 = "mana.bar_color_delayed_heal",
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
        Mana::new(*DEFAULT_MANA_MAX),
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

    let health_bar_config = ValueBarConfig::new()
        .with_offset(0.0, *HEALTH_BAR_OFFSET_Y);

    spawn_value_bar(&mut commands, player_entity, health_bar_config);

    let mana_colors = ValueBarColors {
        background: *MANA_BAR_COLOR_BACKGROUND,
        current: *MANA_BAR_COLOR_CURRENT,
        delayed_damage: *MANA_BAR_COLOR_DELAYED_DAMAGE,
        delayed_heal: *MANA_BAR_COLOR_DELAYED_HEAL,
        transparent: [0.0, 0.0, 0.0, 0.0].into(), // Используем прозрачный цвет из конфига UI, но для маны можно оставить дефолтный или вынести отдельно
    };

    let mana_bar_config = ValueBarConfig::new()
        .with_offset(0.0, *MANA_BAR_OFFSET_Y)
        .with_colors(mana_colors);

    spawn_value_bar(&mut commands, player_entity, mana_bar_config);

    debug_log.add("✅ Player spawned");
}