use bevy::prelude::*;
use avian2d::prelude::*;
use crate::core::config::from_toml;

from_toml!("config/global/display.toml", [
    DEPTH_ALWAYS_BOTTOM: f32 = "depth_layers.always_bottom",
    DEPTH_BACKGROUND: f32 = "depth_layers.background",
    DEPTH_GROUND: f32 = "depth_layers.ground",
    DEPTH_ENVIRONMENT: f32 = "depth_layers.environment",
    DEPTH_ENTITIES: f32 = "depth_layers.entities",
    DEPTH_EFFECTS: f32 = "depth_layers.effects",
    DEPTH_FOREGROUND: f32 = "depth_layers.foreground",
    DEPTH_UI: f32 = "depth_layers.ui",
    DEPTH_ALWAYS_TOP: f32 = "depth_layers.always_top",
]);

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    World,
    DynamicBody,
    Projectile,
    VisionBlock,
}

macro_rules! define_depth_layers {
    (
        $(
            $(#[$meta:meta])*
            $name:ident
        ),* $(,)?
    ) => {
        #[allow(dead_code)]
        #[derive(Component, Clone, Copy, Debug)]
        pub enum DepthLayer {
            $(
                $name(i16),
            )*
        }
    };
}

// [-1000 .. +1000]
define_depth_layers! {
    AlwaysBottom,
    Background,
    Ground,
    Environment,
    Entities,
    Effects,
    Foreground,
    Ui,
    AlwaysTop,
}

impl DepthLayer {
    pub fn depth_value(&self) -> f32 {
        match self {
            DepthLayer::AlwaysBottom(local) => *DEPTH_ALWAYS_BOTTOM + (*local as f32),
            DepthLayer::Background(local) => *DEPTH_BACKGROUND + (*local as f32),
            DepthLayer::Ground(local) => *DEPTH_GROUND + (*local as f32),
            DepthLayer::Environment(local) => *DEPTH_ENVIRONMENT + (*local as f32),
            DepthLayer::Entities(local) => *DEPTH_ENTITIES + (*local as f32),
            DepthLayer::Effects(local) => *DEPTH_EFFECTS + (*local as f32),
            DepthLayer::Foreground(local) => *DEPTH_FOREGROUND + (*local as f32),
            DepthLayer::Ui(local) => *DEPTH_UI + (*local as f32),
            DepthLayer::AlwaysTop(local) => *DEPTH_ALWAYS_TOP + (*local as f32),
        }
    }
}