// src/modules/health/systems.rs
use bevy::prelude::*;
use crate::core::config::from_toml;
use super::components::*;

from_toml!("config/health.toml", [
    BAR_WIDTH: f32 = "bar.width",
    BAR_HEIGHT: f32 = "bar.height",
    BAR_OFFSET_Y: f32 = "bar.offset_y",
    COLOR_BACKGROUND: Vec4 = "colors.background",
    COLOR_CURRENT_HP: Vec4 = "colors.current_hp",
    COLOR_DELAYED_DAMAGE: Vec4 = "colors.delayed_damage",
    COLOR_DELAYED_HEAL: Vec4 = "colors.delayed_heal",
    DELAY_BEFORE_ANIMATION: f32 = "timing.delay_before_animation",
    ANIMATION_DURATION: f32 = "timing.animation_duration",
    VISIBILITY_TIMEOUT: f32 = "timing.visibility_timeout",
    FADE_DURATION: f32 = "timing.fade_duration",
]);

pub fn apply_damage_events(
    mut health_query: Query<&mut Health>,
    resistances_query: Query<&Resistances>,
    mut damage_events: MessageReader<DamageEvent>,
) {
    for event in damage_events.read() {
        if let Ok(mut health) = health_query.get_mut(event.target) {
            let resistance = resistances_query
                .get(event.target)
                .map(|r| r.average_resistance(&event.damage_types))
                .unwrap_or(0.0);
            
            let final_damage = event.amount * (1.0 - resistance);
            health.current = (health.current - final_damage).max(0.0);
        }
    }
}

pub fn update_delayed_health(
    mut health_bars: Query<(&mut HealthBar, &Health)>,
    time: Res<Time>,
) {
    for (mut bar, health) in &mut health_bars {
        let current_ratio = health.ratio();
        
        if (bar.target_delayed_hp - current_ratio).abs() > 0.001 {
            bar.target_delayed_hp = current_ratio;
            bar.delay_timer = *DELAY_BEFORE_ANIMATION;
            bar.animation_timer = 0.0;
            bar.visibility_timer = *VISIBILITY_TIMEOUT;
            bar.is_visible = true;
        }
        
        if bar.delay_timer > 0.0 {
            bar.delay_timer -= time.delta_secs();
        } else if (bar.delayed_hp - bar.target_delayed_hp).abs() > 0.001 {
            bar.animation_timer += time.delta_secs();
            let t = (bar.animation_timer / *ANIMATION_DURATION).clamp(0.0, 1.0);
            bar.delayed_hp = bar.delayed_hp.lerp(bar.target_delayed_hp, t);
            
            if bar.animation_timer >= *ANIMATION_DURATION {
                bar.delayed_hp = bar.target_delayed_hp;
            }
        }
    }
}

pub fn update_health_bar_visuals(
    health_bars: Query<&HealthBar>,
    health_query: Query<&Health>,
    mut sprites: Query<(&mut Sprite, &mut Transform)>,
) {
    for bar in &health_bars {
        let Ok(health) = health_query.get(bar.owner) else { continue };
        let current_ratio = health.ratio();
        let bar_width = *BAR_WIDTH;
        let bar_height = *BAR_HEIGHT;
        let bar_left = -bar_width / 2.0;
        
        if let Ok((mut bg_sprite, mut bg_transform)) = sprites.get_mut(bar.background_entity) {
            bg_sprite.custom_size = Some(Vec2::new(bar_width, bar_height));
            bg_sprite.color = Color::srgba(
                COLOR_BACKGROUND.x,
                COLOR_BACKGROUND.y,
                COLOR_BACKGROUND.z,
                COLOR_BACKGROUND.w,
            );
            bg_transform.translation.x = 0.0;
        }
        
        if bar.delayed_hp > current_ratio {
            if let Ok((mut delayed_sprite, mut delayed_transform)) = sprites.get_mut(bar.delayed_entity) {
                let width = bar.delayed_hp * bar_width;
                delayed_sprite.custom_size = Some(Vec2::new(width, bar_height));
                delayed_sprite.color = Color::srgba(
                    COLOR_DELAYED_DAMAGE.x,
                    COLOR_DELAYED_DAMAGE.y,
                    COLOR_DELAYED_DAMAGE.z,
                    COLOR_DELAYED_DAMAGE.w,
                );
                delayed_transform.translation.x = bar_left + width / 2.0;
            }
            
            if let Ok((mut current_sprite, mut current_transform)) = sprites.get_mut(bar.current_hp_entity) {
                let width = current_ratio * bar_width;
                current_sprite.custom_size = Some(Vec2::new(width, bar_height));
                current_sprite.color = Color::srgba(
                    COLOR_CURRENT_HP.x,
                    COLOR_CURRENT_HP.y,
                    COLOR_CURRENT_HP.z,
                    COLOR_CURRENT_HP.w,
                );
                current_transform.translation.x = bar_left + width / 2.0;
            }
        } else {
            if let Ok((mut delayed_sprite, mut delayed_transform)) = sprites.get_mut(bar.delayed_entity) {
                let width = current_ratio * bar_width;
                delayed_sprite.custom_size = Some(Vec2::new(width, bar_height));
                delayed_sprite.color = Color::srgba(
                    COLOR_DELAYED_HEAL.x,
                    COLOR_DELAYED_HEAL.y,
                    COLOR_DELAYED_HEAL.z,
                    COLOR_DELAYED_HEAL.w,
                );
                delayed_transform.translation.x = bar_left + width / 2.0;
            }
            
            if let Ok((mut current_sprite, mut current_transform)) = sprites.get_mut(bar.current_hp_entity) {
                let width = bar.delayed_hp * bar_width;
                current_sprite.custom_size = Some(Vec2::new(width, bar_height));
                current_sprite.color = Color::srgba(
                    COLOR_CURRENT_HP.x,
                    COLOR_CURRENT_HP.y,
                    COLOR_CURRENT_HP.z,
                    COLOR_CURRENT_HP.w,
                );
                current_transform.translation.x = bar_left + width / 2.0;
            }
        }
    }
}

pub fn update_health_bar_visibility(
    mut health_bars: Query<(&mut HealthBar, &Health)>,
    mut sprites: Query<&mut Sprite>,
    time: Res<Time>,
) {
    for (mut bar, health) in &mut health_bars {
        let current_ratio = health.ratio();
        
        if bar.visibility_timer > 0.0 {
            bar.visibility_timer -= time.delta_secs();
            
            let alpha = if bar.visibility_timer < *FADE_DURATION {
                (bar.visibility_timer / *FADE_DURATION).clamp(0.0, 1.0)
            } else {
                1.0
            };
            
            let entities_and_colors = [
                (bar.background_entity, *COLOR_BACKGROUND),
                (bar.current_hp_entity, *COLOR_CURRENT_HP),
                (bar.delayed_entity, if bar.delayed_hp > current_ratio {
                    *COLOR_DELAYED_DAMAGE
                } else {
                    *COLOR_DELAYED_HEAL
                }),
            ];
            
            for (entity, base_color) in entities_and_colors {
                if let Ok(mut sprite) = sprites.get_mut(entity) {
                    sprite.color = Color::srgba(base_color.x, base_color.y, base_color.z, alpha);
                }
            }
        } else if bar.is_visible {
            bar.is_visible = false;
            
            for entity in [bar.background_entity, bar.current_hp_entity, bar.delayed_entity] {
                if let Ok(mut sprite) = sprites.get_mut(entity) {
                    sprite.color = Color::srgba(0.0, 0.0, 0.0, 0.0);
                }
            }
        }
    }
}