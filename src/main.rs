mod components;
mod systems;
mod macros;
mod entities;
mod core;

use bevy::prelude::*;
use avian2d::prelude::*;
use bevy_spritesheet_animation::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            systems::base::BaseSystemsPlugin,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            SpritesheetAnimationPlugin,
        ))
        .insert_resource(Gravity::ZERO)
        .insert_gizmo_config(
            PhysicsGizmos {
                aabb_color: Some(Color::srgb(1.0, 1.0, 1.0)), // Белый цвет для AABB
                collider_color: Some(Color::srgb(1.0, 1.0, 0.0)), // Желтый цвет для контуров
                ..default()
            },
            GizmoConfig::default(),
        )
        .add_plugins(( // Entities
            entities::RedSlime,
            entities::Player,
        ))
        .add_systems(Startup, (
            systems::setup::ground::spawn_tiles,
            systems::setup::ground::spawn_fences,
        ))
        .add_systems(Update, (
            systems::movement::lerp_follow::lerp_follow_to_player,
        ))
        .run();
}