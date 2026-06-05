use bevy::prelude::*;

use crate::components::markers::Tile;
use crate::components::core::DepthLayer;
use super::state::{
    TILE_SIZE, GRID_WIDTH, GRID_HEIGHT, TILE_TEXTURE_PATH,
};

fn get_tile_rotation(x: usize, y: usize) -> Quat {
    let seed = ((x * 17 + y * 31) % 4) as f32;
    Quat::from_rotation_z(seed * std::f32::consts::FRAC_PI_2)
}

pub fn spawn_ground(asset_server: Res<AssetServer>, mut commands: Commands) {
    let tile_tex = asset_server.load(TILE_TEXTURE_PATH);
    
    let offset_x = GRID_WIDTH as f32 * TILE_SIZE / 2.0;
    let offset_y = GRID_HEIGHT as f32 * TILE_SIZE / 2.0;

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let world_x = x as f32 * TILE_SIZE - offset_x;
            let world_y = y as f32 * TILE_SIZE - offset_y;
            let layer = DepthLayer::Ground(0);
            
            let mut transform = Transform::from_xyz(world_x, world_y, layer.depth_value());
            transform.rotation = get_tile_rotation(x, y);

            commands.spawn((
                Sprite {
                    image: tile_tex.clone(),
                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                    ..default()
                },
                Tile,
                transform,
                layer,
            ));
        }
    }
}
