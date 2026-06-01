use bevy::prelude::*;
use avian2d::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::core::make_spritesheet;
use crate::components::behavior::FollowPlayer;
use crate::components::markers::Player;

use super::state::{PlayerAnimation, PlayerStateHandler};
use super::animation::{create_idle_animation, create_walk_animation};

pub fn summon(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut animations: ResMut<Assets<Animation>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    //Camera
    commands.spawn((
        Camera2d, 
        FollowPlayer { smoothness: 0.99 }
    ));

    let (spritesheet, sprite) = make_spritesheet(
        &asset_server, &mut atlas_layouts,
        "textures/player/map.png",
        8, 1, 128, 16, 64.0, 64.0
    );

    let idle_handler = create_idle_animation(&spritesheet, &mut animations);
    let walk_handler = create_walk_animation(&spritesheet, &mut animations);

    commands.spawn((
        sprite,
        //View
        SpritesheetAnimation::new(idle_handler.clone()),
        PlayerAnimation {
            idle: idle_handler,
            walk: walk_handler,
        },
        //Phy
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Dynamic,
        Collider::rectangle(1.0, 1.0),
        //Beh
        PlayerStateHandler::default(),
        Player,
    ));
}
