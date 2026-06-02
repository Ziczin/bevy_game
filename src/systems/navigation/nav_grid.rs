use bevy::prelude::*;
use avian2d::prelude::*;
use crate::components::core::GameLayer;
use crate::components::markers::Player;

#[derive(Clone, Copy, Debug, Default)]
pub struct CellData {
    pub walkable: bool,
    pub blocks_vision: bool,
}

#[derive(Resource)]
pub struct NavGrid {
    pub cell_size: f32,
    pub width: usize,
    pub height: usize,
    pub origin: Vec2, // Центр сетки в мировых координатах
    pub cells: Vec<CellData>,
}

impl NavGrid {
    pub fn new(cell_size: f32, width: usize, height: usize, origin: Vec2) -> Self {
        Self {
            cell_size,
            width,
            height,
            origin,
            cells: vec![CellData::default(); width * height],
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

    pub fn get_cell(&self, x: usize, y: usize) -> Option<CellData> {
        if x < self.width && y < self.height {
            return Some(self.cells[y * self.width + x]);
        }
        return None;
    }

    pub fn set_cell(&mut self, x: usize, y: usize, walkable: bool, blocks_vision: bool) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = CellData { walkable, blocks_vision };
        }
    }
}

/// Обновляет позицию сетки, если игрок вышел за пределы
pub fn update_nav_grid_position(
    mut grid: Option<ResMut<NavGrid>>, // <-- Option вместо прямого ResMut
    player_query: Query<&Transform, With<Player>>,
    spatial_query: SpatialQuery,
) {
    // Если сетка ещё не создана, просто выходим
    let Some(mut grid) = grid else { return; };
    
    let Ok(player_transform) = player_query.single() else { return; };
    let player_pos = player_transform.translation.xy();
    
    // Проверяем, вышел ли игрок за пределы сетки (с запасом в 1 клетку)
    let threshold = grid.cell_size * 0.5;
    let distance = player_pos - grid.origin;
    
    if distance.x.abs() > threshold || distance.y.abs() > threshold {
        // Игрок вышел за пределы — сдвигаем сетку
        grid.origin = player_pos;
        
        // Пересчитываем всю сетку
        rebuild_nav_grid(&mut grid, &spatial_query);
    }
}


fn rebuild_nav_grid(grid: &mut NavGrid, spatial_query: &SpatialQuery) {
    let movement_filter = SpatialQueryFilter::from_mask([GameLayer::World]);
    let vision_filter = SpatialQueryFilter::from_mask([GameLayer::VisionBlock]);

    for y in 0..grid.height {
        for x in 0..grid.width {
            let center = grid.grid_to_world(x, y);
            
            let is_blocked_movement = !spatial_query.point_intersections(center, &movement_filter).is_empty();
            let is_blocked_vision = !spatial_query.point_intersections(center, &vision_filter).is_empty();
            
            grid.set_cell(x, y, !is_blocked_movement, is_blocked_vision);
        }
    }
}

/// Строит начальную сетку при старте
pub fn build_initial_nav_grid(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    spatial_query: SpatialQuery,
) {
    let Ok(player_transform) = player_query.single() else { return; };
    let player_pos = player_transform.translation.xy();
    
    // Сетка 48x36 клетки по 32px = 1536x1152 (покрывает всю карту 24x18 тайлов по 64px)
    let cell_size = 32.0;
    let width = 48;
    let height = 36;
    
    let mut grid = NavGrid::new(cell_size, width, height, player_pos);
    rebuild_nav_grid(&mut grid, &spatial_query);
    
    commands.insert_resource(grid);
    info!("NavGrid построен: {}x{} клеток с центром на игроке", width, height);
}