// src/core/make_spritesheet.rs
use bevy::{asset::AssetPath, prelude::*};
use bevy_spritesheet_animation::prelude::*;

pub fn make_spritesheet(
    asset_server: &Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    path: String,
    columns: usize,
    rows: usize,
    tile_size: f32,
) -> (Spritesheet, Sprite) {
    let image_width = columns as f32 * tile_size;
    let image_height = rows as f32 * tile_size;
    
    let image = asset_server.load(AssetPath::from(path));
    let spritesheet = Spritesheet::new(&image, columns, rows);

    let mut sprite = spritesheet
        .with_size_hint(image_width as u32, image_height as u32)
        .sprite(atlas_layouts);
    sprite.custom_size = Some(Vec2::new(tile_size, tile_size));
    (spritesheet, sprite)
}