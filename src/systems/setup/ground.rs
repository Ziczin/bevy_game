use bevy::prelude::*;
use crate::components::core::DepthLayer;
use crate::core::extensions::EntityBuilderExt;

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
            println!("{}:{}", ((x as f32 * TILE_SIZE - offset_x) as i32), ((y as f32 * TILE_SIZE - offset_y) as i32));
            commands.spawn((
                Sprite {
                    image: tile_tex.clone(),
                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                    ..default()
                },
                Tile,
            ))
            .at(
                (x as f32 * TILE_SIZE - offset_x) as i32, 
                (y as f32 * TILE_SIZE - offset_y) as i32, 
                DepthLayer::Ground(0)
            );
        }
    }
}

