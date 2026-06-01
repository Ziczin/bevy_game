use bevy::prelude::*;
use crate::components::markers::DepthOrderedDraw;

pub fn depth_ordered_draw(
    mut query: Query<&mut Transform, With<DepthOrderedDraw>>,
) {
    for mut transform in &mut query {
        transform.translation.z = -transform.translation.y / f32::MAX;
    }
    println!()
}