use bevy::prelude::*;
use crate::components::markers::Player;
use crate::components::pathfinding::Pathfinder;
use crate::core::overlay::DebugOverlay;
use state::{RedSlimeStateHandler, RedSlimeLogicFlags};

mod behavior;
mod brain;
mod state;
mod summon;
mod utils;

pub struct RedSlimePlugin;

impl Plugin for RedSlimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, summon::summon)
            .add_systems(Update, (brain::brain, behavior::behavior).chain())
            .add_systems(Update, update_slime_overlay.after(behavior::behavior));
    }
}

fn update_slime_overlay(
    mut overlay: ResMut<DebugOverlay>,
    player_query: Query<&Transform, With<Player>>,
    slime_query: Query<(
        Entity,
        &RedSlimeStateHandler,
        &RedSlimeLogicFlags,
        &Pathfinder,
        &Transform,
    )>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let player_pos = player_transform.translation.xy();

    for (entity, state_handler, logic_flags, pathfinder, transform) in slime_query.iter() {
        let slime_pos = transform.translation.xy();
        let distance = slime_pos.distance(player_pos);
        let state = format!("{:?}", state_handler.get());
        let can_move = if logic_flags.contains(RedSlimeLogicFlags::CanMove) {
            "Yes"
        } else {
            "No "
        };
        let is_active = if pathfinder.is_active { "Yes" } else { "No " };

        let info = format!(
            "State: {}, CanMove: {}, PathActive: {}, Threshold: {:.1}, Dist: {:.1}",
            state, can_move, is_active, pathfinder.arrival_threshold, distance
        );
        let key = format!("Slime {:?}", entity);
        overlay.set_with_tags(key, info, &["red_slime"]);
    }
}
