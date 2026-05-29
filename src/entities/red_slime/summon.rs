use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::core::make_spritesheet;
use crate::components::core::Velocity;
use super::state::{RedSlimeAnimation, RedSlimeStateHandler};
use super::animation::{create_idle_animation, create_walk_animation};

pub fn summon(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut animations: ResMut<Assets<Animation>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {

    let (spritesheet, sprite) = make_spritesheet(
        &asset_server, &mut atlas_layouts,
        "textures/red_slime/map.png",
        5, 1, 80, 16, 64.0, 64.0
    );

    let idle_handler = create_idle_animation(&spritesheet, &mut animations);
    let walk_handler = create_walk_animation(&spritesheet, &mut animations);

    commands.spawn((
        //View
        sprite,
        SpritesheetAnimation::new(idle_handler.clone()),
        RedSlimeAnimation {
            idle: idle_handler,
            walk: walk_handler,
        },
        //Phy
        Transform::from_xyz(200.0, 0.0, 0.0),
        Velocity::default(),
        //Beh
        RedSlimeStateHandler::default(),
    ));
}