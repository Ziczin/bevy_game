use bevy::prelude::*;

#[derive(Resource)]
pub struct NavGrid {
    pub cell_size: f32,
    pub width: usize,
    pub height: usize,
    pub origin: Vec2,
    // SoA (Structure of Arrays): разделяем данные для максимальной кэш-локальности
    pub walkable: Vec<bool>,
    pub blocks_vision: Vec<bool>,
}

impl NavGrid {
    pub fn new(cell_size: f32, width: usize, height: usize, origin: Vec2) -> Self {
        let len = width * height;
        Self {
            cell_size,
            width,
            height,
            origin,
            // Инициализируем двумя непрерывными массивами
            walkable: vec![false; len],
            blocks_vision: vec![false; len],
        }
    }

    pub fn world_to_grid(&self, pos: Vec2) -> Option<(usize, usize)> {
        let half_width = self.width as f32 * self.cell_size / 2.0;
        let half_height = self.height as f32 * self.cell_size / 2.0;
        
        let x = ((pos.x - self.origin.x + half_width) / self.cell_size).floor() as isize;
        let y = ((pos.y - self.origin.y + half_height) / self.cell_size).floor() as isize;
        
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            return Some((x as usize, y as usize));
        }
        return None;
    }

    pub fn grid_to_world(&self, x: usize, y: usize) -> Vec2 {
        let half_width = self.width as f32 * self.cell_size / 2.0;
        let half_height = self.height as f32 * self.cell_size / 2.0;
        
        return Vec2::new(
            self.origin.x - half_width + (x as f32 + 0.5) * self.cell_size,
            self.origin.y - half_height + (y as f32 + 0.5) * self.cell_size,
        );
    }

    /// Возвращает кортеж (walkable, blocks_vision)
    pub fn get_cell(&self, x: usize, y: usize) -> Option<(bool, bool)> {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            Some((self.walkable[idx], self.blocks_vision[idx]))
        } else {
            None
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, walkable: bool, blocks_vision: bool) {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.walkable[idx] = walkable;
            self.blocks_vision[idx] = blocks_vision;
        }
    }
}