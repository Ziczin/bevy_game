use bevy::prelude::*;
use crate::components::core::Velocity;

pub fn apply_velocity(
    time: Res<Time<Virtual>>,
    mut query: Query<(&mut Transform, &Velocity)>,
) {
    for (mut tr, vel) in query.iter_mut() {
        tr.translation.x += vel.x * time.delta_secs();
        tr.translation.y += vel.y * time.delta_secs();
    }
}