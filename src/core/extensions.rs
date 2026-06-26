use bevy::prelude::*;
use avian2d::prelude::*;

use crate::components::{core::DepthLayer, markers::{DepthOrderedDraw, DepthOrderedDrawOnce}};

pub trait EntityBuilderExt {
    fn at(self, x: i32, y: i32, layer: DepthLayer) -> Self;
    fn into_dynamic_body(self) -> Self;
    fn into_static_body(self) -> Self;
    fn use_depth_ordered_draw(self) -> Self;
    fn use_depth_ordered_draw_once(self) -> Self;
    
    fn with_rect_collider(
        self, 
        w: i32, 
        h: i32, 
        x: i32, 
        y: i32, 
        layers: impl Into<LayerMask>, 
        filters: impl Into<LayerMask>
    ) -> Self;

    fn with_oval_collider(
        self,
        half_width: i32,
        half_height: i32,
        x: i32,
        y: i32,
        layers: impl Into<LayerMask>,
        filters: impl Into<LayerMask>
    ) -> Self;
}

impl EntityBuilderExt for EntityCommands<'_> {
    fn at(mut self, x: i32, y: i32, layer: DepthLayer) -> Self {
        self.insert((
            Transform::from_xyz(x as f32, y as f32, layer.depth_value()),
            layer,
        ));
        self
    }

    fn into_dynamic_body(mut self) -> Self {
        self.insert((
            RigidBody::Dynamic, 
            LockedAxes::ROTATION_LOCKED,
            LinearVelocity::default(),
        ));
        self
    }

    fn into_static_body(mut self) -> Self {
        self.insert(RigidBody::Static);
        self
    }

    fn use_depth_ordered_draw(mut self) -> Self {
        self.insert(DepthOrderedDraw);
        self
    }

    fn use_depth_ordered_draw_once(mut self) -> Self {
        self.insert(DepthOrderedDrawOnce);
        self
    }

    fn with_rect_collider(
        mut self, 
        w: i32, 
        h: i32, 
        x: i32, 
        y: i32, 
        layers: impl Into<LayerMask>, 
        filters: impl Into<LayerMask>
    ) -> Self {
        self.with_children(|children| {
            children.spawn((
                Collider::rectangle(w as f32, h as f32),
                Transform::from_xyz(x as f32, y as f32, 0.0),
                CollisionLayers::new(layers, filters),
            ));
        });
        self
    }

    fn with_oval_collider(
        mut self,
        half_width: i32,
        half_height: i32,
        x: i32,
        y: i32,
        layers: impl Into<LayerMask>,
        filters: impl Into<LayerMask>
    ) -> Self {
        self.with_children(|children| {
            children.spawn((
                Collider::ellipse(half_width as f32, half_height as f32),
                Transform::from_xyz(x as f32, y as f32, 0.0),
                CollisionLayers::new(layers, filters),
            ));
        });
        self
    }
}