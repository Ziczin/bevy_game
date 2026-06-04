use bevy::prelude::*;
use std::collections::HashSet;

use crate::components::markers::Tile;
use crate::components::core::DepthLayer;
use super::state::{
    PATH_TEXTURE_PATH, PATH_ATLAS_COLS, PATH_ATLAS_ROWS,
    PATH_TILE_SIZE, PATH_TILE_MAP,
    TILE_SIZE, GRID_WIDTH, GRID_HEIGHT,
};

/// Вычисляет индекс спрайта по маске соседства Мура
fn get_path_sprite_index(mask: u8) -> Option<usize> {
    let idx = PATH_TILE_MAP[mask as usize];
    if idx >= 0 {
        Some(idx as usize)
    } else {
        None
    }
}

/// Вычисляет маску соседства для позиции (x, y) в сетке дорожек
fn compute_moore_mask(
    x: usize,
    y: usize,
    path_tiles: &HashSet<(usize, usize)>,
    grid_width: usize,
    grid_height: usize,
) -> u8 {
    let mut mask = 0u8;
    
    // Проверяем 8 соседей в порядке: NW, N, NE, E, SE, S, SW, W
    let neighbors = [
        (-1, -1), // NW (bit 7)
        (0, -1),  // N  (bit 6)
        (1, -1),  // NE (bit 5)
        (1, 0),   // E  (bit 4)
        (1, 1),   // SE (bit 3)
        (0, 1),   // S  (bit 2)
        (-1, 1),  // SW (bit 1)
        (-1, 0),  // W  (bit 0)
    ];
    
    for (bit, (dx, dy)) in neighbors.iter().enumerate() {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        
        if nx >= 0 && nx < grid_width as isize && ny >= 0 && ny < grid_height as isize {
            if path_tiles.contains(&(nx as usize, ny as usize)) {
                mask |= 1 << (7 - bit);
            }
        }
    }
    
    mask
}

/// Создает тестовый паттерн дорожек для калибровки маппинга
fn create_test_pattern() -> HashSet<(usize, usize)> {
    let mut tiles = HashSet::new();
    
    // Пример паттерна: горизонтальная линия + вертикальная линия + кривая
    // Горизонтальная линия (y = 5)
    for x in 3..8 {
        tiles.insert((x, 5));
    }
    
    // Вертикальная линия (x = 10)
    for y in 3..8 {
        tiles.insert((10, y));
    }
    
    // Угол (L-shape)
    for x in 14..17 {
        tiles.insert((x, 3));
    }
    for y in 3..6 {
        tiles.insert((16, y));
    }
    
    // Перекресток
    for x in 19..22 {
        tiles.insert((x, 10));
    }
    for y in 9..12 {
        tiles.insert((20, y));
    }
    
    // Одиночный тайл
    tiles.insert((2, 2));
    
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
    
    let path_tiles = create_test_pattern();
    
    let offset_x = GRID_WIDTH as f32 * TILE_SIZE / 2.0;
    let offset_y = GRID_HEIGHT as f32 * TILE_SIZE / 2.0;
    
    for &(x, y) in &path_tiles {
        let mask = compute_moore_mask(x, y, &path_tiles, GRID_WIDTH, GRID_HEIGHT);
        let Some(sprite_index) = get_path_sprite_index(mask) else {
            continue;
        };
        
        let world_x = x as f32 * TILE_SIZE - offset_x;
        let world_y = y as f32 * TILE_SIZE - offset_y;
        let layer = DepthLayer::Ground(1); // Поверх земли
        
        let transform = Transform::from_xyz(world_x, world_y, layer.depth_value());
        
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
            Tile,
            transform,
            layer,
        ));
    }
}