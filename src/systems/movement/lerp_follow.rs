use bevy::prelude::*;
use crate::components::markers::Player;
use crate::components::behavior::FollowPlayer;
use crate::core::debug_log::DebugLogBuffer;
use crate::core::profiling::{ProfilingBuffer, ProfileScope};

pub fn lerp_follow_to_player(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    profiling: Res<ProfilingBuffer>,
    mut query: Query<(&FollowPlayer, &mut Transform), Without<Player>>,
    mut debug_log: ResMut<DebugLogBuffer>,
) {
    let _scope = ProfileScope::new(&profiling, "systems::movement::lerp_follow::lerp_follow_to_player", &["movement", "follow", "player", "camera"]);
    
    let Ok(player_transform) = player_query.single() else {
        debug_log.add(&["player"], "lerp_follow_to_player: Player not found!");
        return;
    };
    let target_pos = player_transform.translation;
    for (follow, mut transform) in &mut query {
        let t = (follow.smoothness * time.delta_secs()).clamp(0.0, 1.0);
        transform.translation = transform.translation.lerp(target_pos, t);
    }
}