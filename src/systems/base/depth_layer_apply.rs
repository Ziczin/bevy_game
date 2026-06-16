use bevy::prelude::*;
use crate::components::core::DepthLayer;

pub fn apply_depth_layer(
    mut query: Query<(&mut Transform, &DepthLayer), Changed<DepthLayer>>,
) {
    for (mut transform, layer) in &mut query {
        transform.translation.z = layer.depth_value();
    }
}