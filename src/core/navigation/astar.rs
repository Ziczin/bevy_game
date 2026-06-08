use bevy::prelude::*;
use avian2d::prelude::*;
use std::collections::BinaryHeap;
use crate::components::core::GameLayer;
use super::nav_grid::NavGrid;
use super::state::{
    ASTAR_ORTHOGONAL_COST, ASTAR_DIAGONAL_COST, COLLIDER_MIN_SIZE, NO_ROTATION, NAV_GRID_CELL_SIZE,
};

#[derive(Clone, Copy, PartialEq, Eq)]
struct Node {
    x: usize,
    y: usize,
    f_score: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

pub fn find_path(
    grid: &NavGrid, 
    start: Vec2, 
    goal: Vec2,
    spatial_query: &SpatialQuery,
    agent_half_size: Vec2,
    arrival_threshold: f32, 
    occupied_cells: &std::collections::HashSet<(usize, usize)>,
) -> Option<Vec<Vec2>> {
    let Some((start_x, start_y)) = grid.world_to_grid(start) else { return None; };
    let Some((goal_x, goal_y)) = grid.world_to_grid(goal) else { return None; };

    let agent_collider = Collider::ellipse(
        agent_half_size.x.max(COLLIDER_MIN_SIZE), 
        agent_half_size.y.max(COLLIDER_MIN_SIZE)
    );
    let filter = SpatialQueryFilter::from_mask([GameLayer::World]);

    let mut open_set = BinaryHeap::new();
    
    let grid_size = grid.width * grid.height;
    let mut came_from: Vec<usize> = vec![usize::MAX; grid_size];
    let mut g_score: Vec<i32> = vec![i32::MAX; grid_size];

    let start_idx = start_y * grid.width + start_x;
    g_score[start_idx] = 0;
    open_set.push(Node { x: start_x, y: start_y, f_score: 0 });

    let directions = [
        (0, 1), (1, 0), (0, -1), (-1, 0),
        (1, 1), (-1, 1), (1, -1), (-1, -1),
    ];

    // ИСПРАВЛЕНИЕ: Используем константу NAV_GRID_CELL_SIZE вместо grid.cell_size и магического числа 4.0.
    // Это компенсирует погрешность округления координат (мир -> сетка -> мир) ровно на размер одной клетки.
    // Для arrival_threshold = 40.0 и NAV_GRID_CELL_SIZE = 4.0, порог будет 36.0.
    let safe_threshold = (arrival_threshold - NAV_GRID_CELL_SIZE).max(NAV_GRID_CELL_SIZE);

    while let Some(current) = open_set.pop() {
        let current_world = grid.grid_to_world(current.x, current.y);
        let distance_to_goal = current_world.distance(goal);
        
        // Останавливаем поиск, как только подходим на безопасное расстояние.
        // Это экономит ресурсы CPU, не просчитывая путь до самых 0 единиц.
        if distance_to_goal <= safe_threshold { 
            let is_blocked = !spatial_query.shape_intersections(
                &agent_collider,
                current_world,
                NO_ROTATION,
                &filter,
            ).is_empty();
            
            if !is_blocked {
                let mut path = vec![current_world];
                let mut curr_idx = current.y * grid.width + current.x;
                
                while curr_idx != start_idx {
                    curr_idx = came_from[curr_idx];
                    if curr_idx == usize::MAX { break; }
                    
                    let cx = curr_idx % grid.width;
                    let cy = curr_idx / grid.width;
                    path.push(grid.grid_to_world(cx, cy));
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

                if occupied_cells.contains(&(nx, ny)) {
                    continue;
                }

                let Some((walkable, _)) = grid.get_cell(nx, ny) else { continue; };
                if !walkable {
                    continue;
                }

                if dx != 0 && dy != 0 {
                    let Some((w1, _)) = grid.get_cell(current.x, ny) else { continue; };
                    let Some((w2, _)) = grid.get_cell(nx, current.y) else { continue; };
                    if !w1 || !w2 {
                        continue;
                    }
                }

                let cell_center = grid.grid_to_world(nx, ny);
                let is_blocked = !spatial_query.shape_intersections(
                    &agent_collider,
                    cell_center,
                    NO_ROTATION,
                    &filter,
                ).is_empty();
                if is_blocked {
                    continue;
                }

                let move_cost = if dx != 0 && dy != 0 { ASTAR_DIAGONAL_COST } else { ASTAR_ORTHOGONAL_COST };
                
                let curr_idx = current.y * grid.width + current.x;
                let n_idx = ny * grid.width + nx;
                
                let tentative_g = g_score[curr_idx] + move_cost;

                if tentative_g < g_score[n_idx] {
                    came_from[n_idx] = curr_idx;
                    g_score[n_idx] = tentative_g;
                    
                    let h_score = ((nx as i32 - goal_x as i32).abs() + (ny as i32 - goal_y as i32).abs()) * ASTAR_ORTHOGONAL_COST;
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