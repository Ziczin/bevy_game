use bevy::prelude::*;
use crate::components::core::DepthLayer;
use super::components::ValueBar;
use super::config::{ValueBarConfig, DELAY_BEFORE_ANIMATION, ANIMATION_DURATION, VISIBILITY_TIMEOUT, FADE_DURATION, COLOR_TRANSPARENT};

pub fn spawn_value_bar(
    commands: &mut Commands,
    owner: Entity,
    config: ValueBarConfig,
) -> Entity {
    let padding = config.background_padding;
    let bg_width = config.width + padding * 2.0;
    let bg_height = config.height + padding * 2.0;

    let background_entity = commands.spawn((
        Sprite {
            color: Color::srgba(
                config.colors.background.x,
                config.colors.background.y,
                config.colors.background.z,
                config.colors.background.w,
            ),
            custom_size: Some(Vec2::new(bg_width, bg_height)),
            ..default()
        },
        Transform::from_xyz(config.offset_x, config.offset_y, 0.0),
        DepthLayer::Ui(0),
    )).id();

    let delayed_entity = commands.spawn((
        Sprite {
            color: Color::srgba(
                config.colors.delayed_damage.x,
                config.colors.delayed_damage.y,
                config.colors.delayed_damage.z,
                config.colors.delayed_damage.w,
            ),
            custom_size: Some(Vec2::new(config.width, config.height)),
            ..default()
        },
        Transform::from_xyz(config.offset_x, config.offset_y, 0.1),
        DepthLayer::Ui(0),
    )).id();

    let current_entity = commands.spawn((
        Sprite {
            color: Color::srgba(
                config.colors.current.x,
                config.colors.current.y,
                config.colors.current.z,
                config.colors.current.w,
            ),
            custom_size: Some(Vec2::new(config.width, config.height)),
            ..default()
        },
        Transform::from_xyz(config.offset_x, config.offset_y, 0.2),
        DepthLayer::Ui(0),
    )).id();

    commands.entity(owner).add_children(&[background_entity, delayed_entity, current_entity]);

    let bar_entity = commands.spawn(ValueBar {
        owner,
        background_entity,
        current_entity,
        delayed_entity,
        value: 1.0,
        delayed_value: 1.0,
        target_delayed_value: 1.0,
        delay_timer: 0.0,
        animation_timer: 0.0,
        visibility_timer: *VISIBILITY_TIMEOUT,
        is_visible: true,
        width: config.width,
        height: config.height,
        offset_x: config.offset_x,
        offset_y: config.offset_y,
        background_padding: config.background_padding,
        background_color: config.colors.background,
        current_color: config.colors.current,
        delayed_damage_color: config.colors.delayed_damage,
        delayed_heal_color: config.colors.delayed_heal,
    }).id();

    commands.entity(owner).add_child(bar_entity);

    bar_entity
}

pub fn update_delayed_value(
    mut bars: Query<&mut ValueBar>,
    time: Res<Time>,
) {
    for mut bar in &mut bars {
        if (bar.target_delayed_value - bar.value).abs() > 0.001 {
            bar.target_delayed_value = bar.value;
            bar.delay_timer = *DELAY_BEFORE_ANIMATION;
            bar.animation_timer = 0.0;
            bar.visibility_timer = *VISIBILITY_TIMEOUT;
            bar.is_visible = true;
        }

        if bar.delay_timer > 0.0 {
            bar.delay_timer -= time.delta_secs();
        } else if (bar.delayed_value - bar.target_delayed_value).abs() > 0.001 {
            bar.animation_timer += time.delta_secs();
            let t = (bar.animation_timer / *ANIMATION_DURATION).clamp(0.0, 1.0);
            bar.delayed_value = bar.delayed_value.lerp(bar.target_delayed_value, t);

            if bar.animation_timer >= *ANIMATION_DURATION {
                bar.delayed_value = bar.target_delayed_value;
            }
        }
    }
}

pub fn update_value_bar_visuals(
    mut bars: Query<&mut ValueBar>,
    mut sprites: Query<(&mut Sprite, &mut Transform)>,
) {
    for bar in &mut bars {
        if !bar.is_visible {
            continue;
        }

        let current_ratio = bar.value.clamp(0.0, 1.0);
        let bar_width = bar.width;
        let bar_height = bar.height;
        let bar_left = bar.offset_x - bar_width / 2.0;
        let padding = bar.background_padding;

        if let Ok((mut bg_sprite, mut bg_transform)) = sprites.get_mut(bar.background_entity) {
            bg_sprite.custom_size = Some(Vec2::new(bar_width + padding * 2.0, bar_height + padding * 2.0));
            bg_sprite.color = Color::srgba(
                bar.background_color.x,
                bar.background_color.y,
                bar.background_color.z,
                bar.background_color.w,
            );
            bg_transform.translation = Vec3::new(bar.offset_x, bar.offset_y, 0.0);
        }

        if bar.delayed_value > current_ratio {
            if let Ok((mut delayed_sprite, mut delayed_transform)) = sprites.get_mut(bar.delayed_entity) {
                let width = bar.delayed_value * bar_width;
                delayed_sprite.custom_size = Some(Vec2::new(width, bar_height));
                delayed_sprite.color = Color::srgba(
                    bar.delayed_damage_color.x,
                    bar.delayed_damage_color.y,
                    bar.delayed_damage_color.z,
                    bar.delayed_damage_color.w,
                );
                delayed_transform.translation = Vec3::new(bar_left + width / 2.0, bar.offset_y, 0.1);
            }

            if let Ok((mut current_sprite, mut current_transform)) = sprites.get_mut(bar.current_entity) {
                let width = current_ratio * bar_width;
                current_sprite.custom_size = Some(Vec2::new(width, bar_height));
                current_sprite.color = Color::srgba(
                    bar.current_color.x,
                    bar.current_color.y,
                    bar.current_color.z,
                    bar.current_color.w,
                );
                current_transform.translation = Vec3::new(bar_left + width / 2.0, bar.offset_y, 0.2);
            }
        } else {
            if let Ok((mut delayed_sprite, mut delayed_transform)) = sprites.get_mut(bar.delayed_entity) {
                let width = current_ratio * bar_width;
                delayed_sprite.custom_size = Some(Vec2::new(width, bar_height));
                delayed_sprite.color = Color::srgba(
                    bar.delayed_heal_color.x,
                    bar.delayed_heal_color.y,
                    bar.delayed_heal_color.z,
                    bar.delayed_heal_color.w,
                );
                delayed_transform.translation = Vec3::new(bar_left + width / 2.0, bar.offset_y, 0.1);
            }

            if let Ok((mut current_sprite, mut current_transform)) = sprites.get_mut(bar.current_entity) {
                let width = bar.delayed_value * bar_width;
                current_sprite.custom_size = Some(Vec2::new(width, bar_height));
                current_sprite.color = Color::srgba(
                    bar.current_color.x,
                    bar.current_color.y,
                    bar.current_color.z,
                    bar.current_color.w,
                );
                current_transform.translation = Vec3::new(bar_left + width / 2.0, bar.offset_y, 0.2);
            }
        }
    }
}

pub fn update_value_bar_visibility(
    mut bars: Query<&mut ValueBar>,
    mut sprites: Query<&mut Sprite>,
    time: Res<Time>,
) {
    for mut bar in &mut bars {
        if bar.visibility_timer > 0.0 {
            bar.visibility_timer -= time.delta_secs();

            let alpha = if bar.visibility_timer < *FADE_DURATION {
                (bar.visibility_timer / *FADE_DURATION).clamp(0.0, 1.0)
            } else {
                1.0
            };

            let entities = [
                (bar.background_entity, bar.background_color),
                (bar.current_entity, bar.current_color),
                (bar.delayed_entity, if bar.delayed_value > bar.value {
                    bar.delayed_damage_color
                } else {
                    bar.delayed_heal_color
                }),
            ];

            for (entity, base_color) in entities {
                if let Ok(mut sprite) = sprites.get_mut(entity) {
                    sprite.color = Color::srgba(base_color.x, base_color.y, base_color.z, alpha);
                }
            }
        } else if bar.is_visible {
            bar.is_visible = false;

            for entity in [bar.background_entity, bar.current_entity, bar.delayed_entity] {
                if let Ok(mut sprite) = sprites.get_mut(entity) {
                    sprite.color = Color::srgba(
                        COLOR_TRANSPARENT.x,
                        COLOR_TRANSPARENT.y,
                        COLOR_TRANSPARENT.z,
                        COLOR_TRANSPARENT.w,
                    );
                }
            }
        }
    }
}