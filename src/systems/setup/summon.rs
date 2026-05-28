use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use crate::components::core::Velocity;
use crate::components::behavior::{ FollowPlayer, RedSlimeStateHandler, PlayerStateHandler};
use crate::components::markers::Player;
use crate::components::animations::{ PlayerAnimations, RedSlimeAnimations };

fn create_idle_animation(
    spritesheet: &Spritesheet,
    animations: &mut ResMut<Assets<Animation>>,
) -> Handle<Animation> {
    let animation = spritesheet
        .create_animation()
        .add_cell(0, 0)
        .add_cell(1, 0)
        .set_duration(AnimationDuration::PerFrame(500))
        .build();

    return animations.add(animation);
}

fn create_walk_animation(
    spritesheet: &Spritesheet,
    animations: &mut ResMut<Assets<Animation>>,
) -> Handle<Animation> {
    let animation = spritesheet
        .create_animation()
        .add_row(0)
        .add_cell(2, 0)
        .add_cell(1, 0)
        .set_duration(AnimationDuration::PerFrame(100))
        .set_repetitions(AnimationRepeat::Loop)
        .build();

    return animations.add(animation);
}

pub fn player(
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

    let image = asset_server.load("textures/player/map.png");
    let spritesheet = Spritesheet::new(&image, 4, 1);

    let idle_handle = create_idle_animation(&spritesheet, &mut animations);
    let walk_handle = create_walk_animation(&spritesheet, &mut animations);

    let mut sprite = spritesheet
        .with_size_hint(64, 16)
        .sprite(&mut atlas_layouts);
    sprite.custom_size = Some(Vec2::new(64.0, 64.0));

    commands.spawn((
        //View
        sprite,
        SpritesheetAnimation::new(idle_handle.clone()),
        PlayerAnimations {
            idle: idle_handle,
            walk: walk_handle,
        },
        //Phy
        Transform::from_xyz(0.0, 0.0, 0.0),
        Velocity::default(),
        //Beh
        PlayerStateHandler::default(),
        Player,
    ));
}

pub fn red_slime(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut animations: ResMut<Assets<Animation>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {

    let image = asset_server.load("textures/enemy/map.png");
    let spritesheet = Spritesheet::new(&image, 4, 1);
    
    let idle_handle = create_idle_animation(&spritesheet, &mut animations);
    let walk_handle = create_walk_animation(&spritesheet, &mut animations);

    let mut sprite = spritesheet
        .with_size_hint(64, 16)
        .sprite(&mut atlas_layouts);
    sprite.custom_size = Some(Vec2::new(64.0, 64.0));

    commands.spawn((
        //View
        sprite,
        SpritesheetAnimation::new(idle_handle.clone()),
        RedSlimeAnimations {
            idle: idle_handle,
            walk: walk_handle,
        },
        //Phy
        Transform::from_xyz(200.0, 0.0, 0.0),
        Velocity::default(),
        //Beh
        RedSlimeStateHandler::default(),
    ));
}