use bevy::prelude::*;
use avian2d::prelude::*;
use std::collections::{BinaryHeap, HashMap};
use crate::components::core::GameLayer;
use super::nav_grid::NavGrid;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Node {
    x: usize,
    y: usize,
    f_score: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f_score.cmp(&self.f_score) // Min-heap
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

/// Находит путь от start до goal по сетке, учитывая физический размер агента.
/// Останавливается на первой клетке в радиусе arrival_threshold от цели.
pub fn find_path(
    grid: &NavGrid, 
    start: Vec2, 
    goal: Vec2,
    spatial_query: &SpatialQuery,
    agent_half_size: Vec2,
    arrival_threshold: f32,
) -> Option<Vec<Vec2>> {
    let Some((start_x, start_y)) = grid.world_to_grid(start) else { return None; };
    let Some((goal_x, goal_y)) = grid.world_to_grid(goal) else { return None; };

    // Создаем коллайдер агента и фильтр один раз перед циклом
    let agent_collider = Collider::ellipse(
        agent_half_size.x.max(0.1), 
        agent_half_size.y.max(0.1)
    );
    let filter = SpatialQueryFilter::from_mask([GameLayer::World]);

    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut g_score: HashMap<(usize, usize), i32> = HashMap::new();

    open_set.push(Node { x: start_x, y: start_y, f_score: 0 });
    g_score.insert((start_x, start_y), 0);

    // 8-связность (включая диагонали)
    let directions = [
        (0, 1), (1, 0), (0, -1), (-1, 0),
        (1, 1), (-1, 1), (1, -1), (-1, -1),
    ];

    while let Some(current) = open_set.pop() {
        // Проверяем, находимся ли мы в радиусе arrival_threshold от цели
        let current_world = grid.grid_to_world(current.x, current.y);
        let distance_to_goal = current_world.distance(goal);
        
        if distance_to_goal <= arrival_threshold {
            // Проверяем, что эта клетка физически проходима
            let is_blocked = !spatial_query.shape_intersections(
                &agent_collider,
                current_world,
                0.0,
                &filter,
            ).is_empty();
            
            if !is_blocked {
                // Нашли подходящую клетку — восстанавливаем путь
                let mut path = vec![current_world];
                let mut curr = (current.x, current.y);
                while let Some(&prev) = came_from.get(&curr) {
                    path.push(grid.grid_to_world(prev.0, prev.1));
                    curr = prev;
                }
                path.reverse();
                return Some(path);
            }
        }

        for &(dx, dy) in &directions {
            let nx = current.x as isize + dx;
            let ny = current.y as isize + dy;

            if nx >= 0 && nx < grid.width as isize && ny >= 0 && ny < grid.height as isize {
                let nx = nx as usize;
                let ny = ny as usize;

                // Базовая проверка по NavGrid (быстрая)
                let Some(cell) = grid.get_cell(nx, ny) else { continue; };
                if !cell.walkable {
                    continue;
                }

                // Для диагоналей проверяем, что оба соседних клетки тоже проходимы
                if dx != 0 && dy != 0 {
                    let Some(adjacent1) = grid.get_cell(current.x, ny) else { continue; };
                    let Some(adjacent2) = grid.get_cell(nx, current.y) else { continue; };
                    if !adjacent1.walkable || !adjacent2.walkable {
                        continue;
                    }
                }

                // Физическая проверка: может ли агент находиться в этой клетке?
                let cell_center = grid.grid_to_world(nx, ny);
                let is_blocked = !spatial_query.shape_intersections(
                    &agent_collider,
                    cell_center,
                    0.0,
                    &filter,
                ).is_empty();
                if is_blocked {
                    continue;
                }

                // Стоимость: 10 для ортогональных, 14 для диагональных
                let move_cost = if dx != 0 && dy != 0 { 14 } else { 10 };
                let tentative_g = g_score.get(&(current.x, current.y)).unwrap_or(&0) + move_cost;

                if tentative_g < *g_score.get(&(nx, ny)).unwrap_or(&i32::MAX) {
                    came_from.insert((nx, ny), (current.x, current.y));
                    g_score.insert((nx, ny), tentative_g);
                    
                    let h_score = ((nx as i32 - goal_x as i32).abs() + (ny as i32 - goal_y as i32).abs()) * 10;
                    open_set.push(Node {
                        x: nx,
                        y: ny,
                        f_score: tentative_g + h_score,
                    });
                }
            }
        }
    }

    return None;
}