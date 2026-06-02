use bevy::prelude::*;
use crate::components::core::DepthLayer;
use crate::components::markers::{DepthOrderedDraw, DepthOrderedDrawOnce};

pub fn depth_ordered_draw(
    mut query: Query<(&mut Transform, &DepthLayer), With<DepthOrderedDraw>>,
) {
    for (mut transform, layer) in &mut query {
        transform.translation.z = layer.depth_value() - transform.translation.y / f32::MAX;
    }
}

pub fn depth_ordered_draw_once(
    mut query: Query<(&mut Transform, &DepthLayer), With<DepthOrderedDrawOnce>>,
) {
    for (mut transform, layer) in &mut query {
        transform.translation.z = layer.depth_value() - transform.translation.y / f32::MAX;
    }
}