// FILE: src/entities/player/mod.rs
use bevy::prelude::*;

mod state;
mod summon;
mod brain;
mod handle_input;
mod behavior;
mod health_input;

use crate::modules::health::{Health, Mana};
use crate::modules::value_bar::ValueBar;
use self::state::{HEALTH_BAR_OFFSET_Y, MANA_BAR_OFFSET_Y};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, summon::summon)
            .add_systems(Update, (
                handle_input::handle_input,
                health_input::handle_health_input,
                brain::brain,
                behavior::behavior,
                sync_health_to_bar,
                sync_mana_to_bar,
            ).chain()
        );
    }
}

fn sync_health_to_bar(
    health_query: Query<&Health, With<crate::components::markers::Player>>,
    mut value_bars: Query<&mut ValueBar>,
    player_query: Query<Entity, With<crate::components::markers::Player>>,
) {
    let Ok(player_entity) = player_query.single() else { return };
    let Ok(health) = health_query.get(player_entity) else { return };

    for mut bar in &mut value_bars {
        if bar.owner == player_entity && bar.offset_y == *HEALTH_BAR_OFFSET_Y {
            bar.value = health.ratio();
        }
    }
}

fn sync_mana_to_bar(
    mana_query: Query<&Mana, With<crate::components::markers::Player>>,
    mut value_bars: Query<&mut ValueBar>,
    player_query: Query<Entity, With<crate::components::markers::Player>>,
) {
    let Ok(player_entity) = player_query.single() else { return };
    let Ok(mana) = mana_query.get(player_entity) else { return };

    for mut bar in &mut value_bars {
        if bar.owner == player_entity && bar.offset_y == *MANA_BAR_OFFSET_Y {
            bar.value = mana.ratio();
        }
    }
}