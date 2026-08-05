use crate::components::markers::Player;
use crate::modules::health::{Health, Mana};
use bevy::prelude::*;

pub fn handle_health_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut health_query: Query<&mut Health, With<Player>>,
    mut mana_query: Query<&mut Mana, With<Player>>,
    player_query: Query<Entity, With<Player>>,
) {
    let Ok(player_entity) = player_query.single() else {
        return;
    };
    let Ok(mut health) = health_query.get_mut(player_entity) else {
        return;
    };
    let Ok(mut mana) = mana_query.get_mut(player_entity) else {
        return;
    };

    if keyboard.just_pressed(KeyCode::ArrowUp) {
        health.current = (health.current + 10.0).min(health.max);
    }
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        health.current = (health.current - 17.0).max(0.0);
    }
    if keyboard.just_pressed(KeyCode::ArrowRight) {
        mana.current = (mana.current + 10.0).min(mana.max);
    }
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        mana.current = (mana.current - 17.0).max(0.0);
    }
}
