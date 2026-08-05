use bevy::prelude::*;
use crate::components::core::DepthLayer;
use crate::components::markers::{DepthOrderedDraw, DepthOrderedDrawOnce};
use crate::core::profiling::{ProfilingBuffer, profile_scope};

pub fn depth_ordered_draw(
    profiling: Res<ProfilingBuffer>,
    mut query: Query<(&DepthLayer, &mut Transform), With<DepthOrderedDraw>>,
) {
    profile_scope!(&profiling, "systems::base::depth_ordered_draw::depth_ordered_draw", &["render", "depth", "sorting", "draw"]);
    for (layer, mut transform) in &mut query {
        transform.translation.z = layer.depth_value() - transform.translation.y / f32::MAX;
    }
}

pub fn depth_ordered_draw_once(
    profiling: Res<ProfilingBuffer>,
    mut query: Query<(&DepthLayer, &mut Transform), With<DepthOrderedDrawOnce>>,
) {
    profile_scope!(&profiling, "systems::base::depth_ordered_draw::depth_ordered_draw_once", &["render", "depth", "sorting", "draw", "startup"]);
    for (layer, mut transform) in &mut query {
        transform.translation.z = layer.depth_value() - transform.translation.y / f32::MAX;
    }
}