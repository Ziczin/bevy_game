// src/modules/health/bundles.rs
use bevy::prelude::*;
use crate::components::core::DepthLayer;
use crate::components::markers::DepthOrderedDraw;
use crate::core::config::from_toml;
use super::components::HealthBar;

from_toml!("config/health.toml", [
    BAR_WIDTH: f32 = "bar.width",
    BAR_HEIGHT: f32 = "bar.height",
    BAR_OFFSET_Y: f32 = "bar.offset_y",
    COLOR_BACKGROUND: Vec4 = "colors.background",
    COLOR_CURRENT_HP: Vec4 = "colors.current_hp",
    COLOR_DELAYED_DAMAGE: Vec4 = "colors.delayed_damage",
    VISIBILITY_TIMEOUT: f32 = "timing.visibility_timeout",
]);

pub fn spawn_health_bar(
    commands: &mut Commands,
    owner: Entity,
) {
    let bar_width = *BAR_WIDTH;
    let bar_height = *BAR_HEIGHT;
    let offset_y = *BAR_OFFSET_Y;
    
    let background_entity = commands.spawn((
        Sprite {
            color: Color::srgba(
                COLOR_BACKGROUND.x,
                COLOR_BACKGROUND.y,
                COLOR_BACKGROUND.z,
                COLOR_BACKGROUND.w,
            ),
            custom_size: Some(Vec2::new(bar_width, bar_height)),
            ..default()
        },
        Transform::from_xyz(0.0, offset_y, 0.0),
        DepthLayer::Ui(0),
        DepthOrderedDraw,
    )).id();
    
    let delayed_entity = commands.spawn((
        Sprite {
            color: Color::srgba(
                COLOR_DELAYED_DAMAGE.x,
                COLOR_DELAYED_DAMAGE.y,
                COLOR_DELAYED_DAMAGE.z,
                COLOR_DELAYED_DAMAGE.w,
            ),
            custom_size: Some(Vec2::new(bar_width, bar_height)),
            ..default()
        },
        Transform::from_xyz(0.0, offset_y, 0.1),
        DepthLayer::Ui(0),
        DepthOrderedDraw,
    )).id();
    
    let current_hp_entity = commands.spawn((
        Sprite {
            color: Color::srgba(
                COLOR_CURRENT_HP.x,
                COLOR_CURRENT_HP.y,
                COLOR_CURRENT_HP.z,
                COLOR_CURRENT_HP.w,
            ),
            custom_size: Some(Vec2::new(bar_width, bar_height)),
            ..default()
        },
        Transform::from_xyz(0.0, offset_y, 0.2),
        DepthLayer::Ui(0),
        DepthOrderedDraw,
    )).id();
    
    commands.entity(owner).add_children(&[background_entity, delayed_entity, current_hp_entity]);
    
    commands.entity(owner).insert(HealthBar {
        owner,
        background_entity,
        current_hp_entity,
        delayed_entity,
        delayed_hp: 1.0,
        target_delayed_hp: 1.0,
        delay_timer: 0.0,
        animation_timer: 0.0,
        visibility_timer: *VISIBILITY_TIMEOUT,
        is_visible: true,
    });
}