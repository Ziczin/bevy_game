// src/systems/movement/lerp_follow.rs
use bevy::prelude::*;
use crate::components::markers::Player;
use crate::components::behavior::FollowPlayer;
use crate::core::debug_log::DebugLogBuffer;

pub fn lerp_follow_to_player(
    mut query: Query<(&mut Transform, &FollowPlayer), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
    mut debug_log: ResMut<DebugLogBuffer>,
) {
    let Ok(player_transform) = player_query.single() else {
        debug_log.add("⚠️ lerp_follow_to_player: Player not found!");
        return;
    };
    let target_pos = player_transform.translation;
    for (mut transform, follow) in &mut query {
        let t = (follow.smoothness * time.delta_secs()).clamp(0.0, 1.0);
        transform.translation = transform.translation.lerp(target_pos, t);
    }
}