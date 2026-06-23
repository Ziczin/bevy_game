// FILE: src/entities/player/health_input.rs
use bevy::prelude::*;
use crate::components::markers::Player;
use crate::modules::health::{Health, DamageType, DamageEvent, Mana};
use crate::core::config::from_toml;

from_toml!("config/player.toml", [
    HEAL_AMOUNT: f32 = "health.heal_amount",
    DAMAGE_AMOUNT: f32 = "health.damage_amount",
]);

pub fn handle_health_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut health_query: Query<&mut Health, With<Player>>,
    mut mana_query: Query<&mut Mana, With<Player>>,
    mut damage_writer: MessageWriter<DamageEvent>,
    player_query: Query<Entity, With<Player>>,
) {
    let Ok(player_entity) = player_query.single() else { return };
    let Ok(mut health) = health_query.get_mut(player_entity) else { return };
    let Ok(mut mana) = mana_query.get_mut(player_entity) else { return };

    if keyboard.just_pressed(KeyCode::KeyH) {
        health.current = (health.current + *HEAL_AMOUNT).min(health.max);
    }

    if keyboard.just_pressed(KeyCode::KeyD) {
        damage_writer.write(DamageEvent {
            target: player_entity,
            amount: *DAMAGE_AMOUNT,
            damage_types: vec![DamageType::Physical],
        });
    }

    if keyboard.just_pressed(KeyCode::KeyM) {
        damage_writer.write(DamageEvent {
            target: player_entity,
            amount: *DAMAGE_AMOUNT,
            damage_types: vec![DamageType::Magical],
        });
    }

    if keyboard.just_pressed(KeyCode::KeyC) {
        damage_writer.write(DamageEvent {
            target: player_entity,
            amount: *DAMAGE_AMOUNT,
            damage_types: vec![DamageType::Physical, DamageType::Magical],
        });
    }

    if keyboard.just_pressed(KeyCode::Digit1) {
        mana.set_percent(10);
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        mana.set_percent(20);
    }
    if keyboard.just_pressed(KeyCode::Digit3) {
        mana.set_percent(30);
    }
    if keyboard.just_pressed(KeyCode::Digit4) {
        mana.set_percent(40);
    }
    if keyboard.just_pressed(KeyCode::Digit5) {
        mana.set_percent(50);
    }
    if keyboard.just_pressed(KeyCode::Digit6) {
        mana.set_percent(60);
    }
    if keyboard.just_pressed(KeyCode::Digit7) {
        mana.set_percent(70);
    }
    if keyboard.just_pressed(KeyCode::Digit8) {
        mana.set_percent(80);
    }
    if keyboard.just_pressed(KeyCode::Digit9) {
        mana.set_percent(90);
    }
    if keyboard.just_pressed(KeyCode::Digit0) {
        mana.set_percent(100);
    }
    if keyboard.just_pressed(KeyCode::Minus) {
        mana.set_percent(0);
    }
}