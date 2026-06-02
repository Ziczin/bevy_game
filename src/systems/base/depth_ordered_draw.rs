use bevy::prelude::*;
use crate::components::core::DepthLayer;
use crate::components::markers::DepthOrderedDraw;

pub fn depth_ordered_draw(
    mut query: Query<(&mut Transform, &DepthLayer), With<DepthOrderedDraw>>,
) {
    for (mut transform, layer) in &mut query {
        transform.translation.z = layer.depth_value() - transform.translation.y / f32::MAX;
    }
}