use bevy::prelude::*;
use avian2d::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::core::make_spritesheet;

const TILE_SIZE: f32 = 64.0;
const GRID_WIDTH: usize = 24;
const GRID_HEIGHT: usize = 18;

#[derive(Component)]
pub struct Tile;

pub fn spawn_tiles(asset_server: Res<AssetServer>, mut commands: Commands) {
    let tile_tex = asset_server.load("textures/ground/tile.png");
    
    let offset_x = GRID_WIDTH as f32 * TILE_SIZE / 2.0;
    let offset_y = GRID_HEIGHT as f32 * TILE_SIZE / 2.0;

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            commands.spawn((
                Sprite {
                    image: tile_tex.clone(),
                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                    ..default()
                },
                Transform::from_xyz(
                    x as f32 * TILE_SIZE - offset_x,
                    y as f32 * TILE_SIZE - offset_y,
                    -10.0,
                ),
                Tile,
            ));
        }
    }
}

pub fn spawn_fences(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let (spritesheet, mut sprite) = make_spritesheet(
        &asset_server, &mut atlas_layouts,
        "textures/ground/fence_map.png",
        4, 4, 64, 64, 64.0, 64.0
    );

    sprite.texture_atlas.as_mut().unwrap().index = 15;

    commands.spawn((
        sprite,
        Transform::from_xyz(0.0, 0.0, 1.0),
        RigidBody::Static,
        Collider::rectangle(1.0, 1.0),
    ));
}