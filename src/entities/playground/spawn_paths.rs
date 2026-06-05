use bevy::prelude::*;
use std::collections::HashSet;

use crate::components::markers::Tile as TileMarker;
use crate::components::core::DepthLayer;
use crate::core::blob_tilemap::get_blob_tile_index;
use super::state::{
    PATH_TEXTURE_PATH, PATH_ATLAS_COLS, PATH_ATLAS_ROWS,
    PATH_TILE_SIZE, TILE_SIZE,
};

const PATTERN: &[&str] = &[
    ".#........#.#....",
    ".#####.####.#.#..",
    "...#.#####.#.###.",
    "..#####.######...",
    "..##.###.#.###.##",
    "...#.##.######.#.",
    "..##########.###.",
    "..########..#.##.",
    "....###########..",
    "..####.########..",
    ".#.######.###..#.",
    "..##.###########.",
    "..#####.#.#####..",
    ".....#.####.#....",
    ".##.###.#.#.####.",
    "..#.###.##.##.#..",
    "........#........",
    "........#........",
    "........#........",
    "........#........",
    "#################",

];

const PATTERN_OFFSET_X: i32 = -10;
const PATTERN_OFFSET_Y: i32 = 20;

#[inline]
fn compute_moore_mask(x: i32, y: i32, tiles: &HashSet<(i32, i32)>) -> u8 {
    let mut mask = 0u8;
    let neighbors: [(i32, i32); 8] = [
        (-1, -1), (0, -1), (1, -1),
        ( 1,  0),          (1,  1),
        ( 0,  1),
        (-1,  1), (-1, 0),
    ];
    for (bit, &(dx, dy)) in neighbors.iter().enumerate() {
        if tiles.contains(&(x + dx, y + dy)) {
            mask |= 1 << (7 - bit);
        }
    }
    mask
}

fn create_pattern_tiles() -> HashSet<(i32, i32)> {
    let mut tiles = HashSet::new();
    for (row, line) in PATTERN.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                tiles.insert((
                    PATTERN_OFFSET_X + col as i32,
                    PATTERN_OFFSET_Y - row as i32,
                ));
            }
        }
    }
    tiles
}

pub fn spawn_paths(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let image = asset_server.load(PATH_TEXTURE_PATH);
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(PATH_TILE_SIZE as u32),
        PATH_ATLAS_COLS as u32,
        PATH_ATLAS_ROWS as u32,
        None,
        None,
    );
    let layout_handle = atlas_layouts.add(layout);

    let path_tiles = create_pattern_tiles();
    let mut unique_sprites = HashSet::new();

    for &(x, y) in &path_tiles {
        let mask = compute_moore_mask(x, y, &path_tiles);
        let sprite_index = get_blob_tile_index(mask);
        unique_sprites.insert(sprite_index);

        let layer = DepthLayer::Ground(1);
        let transform = Transform::from_xyz(
            x as f32 * TILE_SIZE,
            y as f32 * TILE_SIZE,
            layer.depth_value(),
        );

        commands.spawn((
            Sprite {
                image: image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: layout_handle.clone(),
                    index: sprite_index,
                }),
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..default()
            },
            TileMarker,
            transform,
            layer,
        ));
    }

    println!("🗺️  Path tiles spawned: {} total, {} unique sprites", 
             path_tiles.len(), unique_sprites.len());
}