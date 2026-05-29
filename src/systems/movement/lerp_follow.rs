use bevy::prelude::*;
use crate::components::markers::Player;
use crate::components::behavior::FollowPlayer;

pub fn lerp_follow_to_player(
    mut query: Query<(&mut Transform, &FollowPlayer), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let target_pos = player_transform.translation;
    for (mut transform, follow) in &mut query {
        let t = (follow.smoothness * time.delta_secs()).clamp(0.0, 1.0);
        transform.translation = transform.translation.lerp(target_pos, t);
    }
}