use bevy::prelude::*;
use avian2d::prelude::*;

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
            $name:ident : $base:expr
        ),* $(,)?
    ) => {
        #[allow(dead_code)]
        #[derive(Component, Clone, Copy, Debug)]
        pub enum DepthLayer {
            $(
                $name(i16),
            )*
        }

        impl DepthLayer {
            pub fn depth_value(&self) -> f32 {
                match self {
                    $(
                        DepthLayer::$name(local) => $base + (*local as f32),
                    )*
                }
            }
        }
    };
}
// [-1000 .. +1000]
define_depth_layers! {
    AlwaysBottom: -800.0,
    Background: -600.0,
    Ground: -400.0,
    Environment: -200.0,
    Entities: 0.0,
    Effects: 200.0,
    Foreground: 400.0,
    Ui: 600.0,
    AlwaysTop: 800.0,
}