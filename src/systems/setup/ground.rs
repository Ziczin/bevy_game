use bevy::prelude::*;
use avian2d::prelude::*;

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
    let fence_image = asset_server.load("textures/ground/fence_map.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 4, 4, None, None);
    let layout_handle = atlas_layouts.add(layout);

    let row = 0;
    let col = 3;
    let index = row * 4 + col;

    commands.spawn((
        SpriteSheetBundle {
            texture: fence_image,
            atlas: TextureAtlas {
                layout: layout_handle,
                index,
            },
            sprite: Sprite {
                custom_size: Some(Vec2::splat(64.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(1.0, 1.0),
    ));
}