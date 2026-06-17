use bevy::prelude::*;
use crate::components::core::DepthLayer;
#[allow(unused_imports)]
use crate::core::macros::bevy_custom::markers;

// === Размеры навигационной сетки ===
pub const NAV_GRID_CELL_SIZE: f32 = 4.0;
pub const NAV_GRID_WIDTH: usize = 96;
pub const NAV_GRID_HEIGHT: usize = 72;

// === Стоимость шагов в A* ===
pub const ASTAR_ORTHOGONAL_COST: i32 = 10;
pub const ASTAR_DIAGONAL_COST: i32 = 14;

// === Физические пороги ===
pub const COLLIDER_MIN_SIZE: f32 = 0.1;
pub const ELLIPSE_THRESHOLD: f32 = 1.0;
pub const NO_ROTATION: f32 = 0.0;

// === UI-слои визуализации (готовые DepthLayer) ===
pub const NAV_GRID_UI_LAYER: DepthLayer = DepthLayer::Ui(-10);
pub const NAV_PATH_UI_LAYER: DepthLayer = DepthLayer::Ui(-9);
pub const AGENT_CENTER_UI_LAYER: DepthLayer = DepthLayer::Ui(-8);

// === Цвета визуализации ===
pub const GRID_WALKABLE_COLOR: Color = Color::srgba(1.0, 0.5, 0.5, 0.5);
pub const GRID_BLOCKED_COLOR: Color = Color::srgba(1.0, 0.0, 0.0, 0.9);
pub const PATH_POINT_COLOR: Color = Color::srgba(1.0, 1.0, 0.0, 0.8);
pub const PATH_LINE_COLOR: Color = Color::srgba(1.0, 1.0, 0.0, 0.5);
pub const AGENT_CENTER_COLOR: Color = Color::srgba(0.8, 0.0, 0.8, 1.0);
pub const AGENT_OUTLINE_COLOR: Color = Color::srgba(0.8, 0.0, 0.8, 0.5);

// === Размеры визуализации ===
pub const GRID_WALKABLE_SIZE: f32 = 1.0;
pub const GRID_BLOCKED_SIZE: f32 = 2.0;
pub const PATH_POINT_SIZE: f32 = 1.0;
pub const PATH_LINE_THICKNESS: f32 = 1.0;
pub const AGENT_CENTER_SIZE: f32 = 2.0;
pub const AGENT_OUTLINE_THICKNESS: f32 = 1.0;
pub const AGENT_OUTLINE_SEGMENTS: usize = 16;

// === Маркеры визуализации ===
markers! {
    NavGridVisualMarker,
    NavPathVisualMarker,
    AgentCenterVisualMarker,
}
