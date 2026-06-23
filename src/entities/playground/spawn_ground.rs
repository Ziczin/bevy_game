use bevy::prelude::*;

use crate::components::markers::Tile;
use crate::components::core::DepthLayer;
use super::state::{TILE_SIZE, GRID_WIDTH, GRID_HEIGHT, TILE_TEXTURE_PATH, ROTATION_SEED_X, ROTATION_SEED_Y, ROTATION_VARIANTS};

fn get_tile_rotation(x: usize, y: usize) -> Quat {
    let seed = ((x as i32 * *ROTATION_SEED_X + y as i32 * *ROTATION_SEED_Y) % *ROTATION_VARIANTS) as f32;
    Quat::from_rotation_z(seed * std::f32::consts::FRAC_PI_2)
}

pub fn spawn_ground(asset_server: Res<AssetServer>, mut commands: Commands) {
    let tile_tex = asset_server.load(TILE_TEXTURE_PATH);
    
    let tile_size = *TILE_SIZE;
    let grid_width = *GRID_WIDTH;
    let grid_height = *GRID_HEIGHT;
    
    let offset_x = grid_width as f32 * tile_size / 2.0;
    let offset_y = grid_height as f32 * tile_size / 2.0;

    for y in 0..grid_height {
        for x in 0..grid_width {
            let world_x = x as f32 * tile_size - offset_x;
            let world_y = y as f32 * tile_size - offset_y;
            let layer = DepthLayer::Ground(0);
            
            let mut transform = Transform::from_xyz(world_x, world_y, layer.depth_value());
            transform.rotation = get_tile_rotation(x, y);

            commands.spawn((
                Sprite {
                    image: tile_tex.clone(),
                    custom_size: Some(Vec2::splat(tile_size)),
                    ..default()
                },
                Tile,
                transform,
                layer,
            ));
        }
    }
}