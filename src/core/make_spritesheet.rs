// src/core/make_spritesheet.rs
use bevy::{asset::AssetPath, prelude::*};
use bevy_spritesheet_animation::prelude::*;

pub fn make_spritesheet(
    asset_server: &Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    path: String,
    columns: usize,
    rows: usize,
    image_width: u32,
    image_height: u32,
    size_x: f32,
    size_y: f32,
) -> (Spritesheet, Sprite) {
    let image = asset_server.load(AssetPath::from(path));
    let spritesheet = Spritesheet::new(&image, columns, rows);

    let mut sprite = spritesheet
        .with_size_hint(image_width, image_height)
        .sprite(atlas_layouts);
    sprite.custom_size = Some(Vec2::new(size_x, size_y));
    (spritesheet, sprite)
}