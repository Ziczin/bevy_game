// src/core/navigation/state.rs
use bevy::prelude::*;
use crate::components::core::DepthLayer;
use crate::core::config::from_toml;
#[allow(unused_imports)]
use crate::core::macros::bevy_custom::markers;

from_toml!("config/navigation.toml", [
    NAV_GRID_CELL_SIZE: f32 = "grid.cell_size",
    NAV_GRID_WIDTH: usize = "grid.width",
    NAV_GRID_HEIGHT: usize = "grid.height",
    ASTAR_ORTHOGONAL_COST: i32 = "astar.orthogonal_cost",
    ASTAR_DIAGONAL_COST: i32 = "astar.diagonal_cost",
    COLLIDER_MIN_SIZE: f32 = "physics.collider_min_size",
    ELLIPSE_THRESHOLD: f32 = "physics.ellipse_threshold",
    NO_ROTATION: f32 = "physics.no_rotation",
]);

pub const NAV_GRID_UI_LAYER: DepthLayer = DepthLayer::Ui(-10);
pub const NAV_PATH_UI_LAYER: DepthLayer = DepthLayer::Ui(-9);
pub const AGENT_CENTER_UI_LAYER: DepthLayer = DepthLayer::Ui(-8);

pub const GRID_WALKABLE_COLOR: Color = Color::srgba(1.0, 0.5, 0.5, 0.5);
pub const GRID_BLOCKED_COLOR: Color = Color::srgba(1.0, 0.0, 0.0, 0.9);
pub const PATH_POINT_COLOR: Color = Color::srgba(1.0, 1.0, 0.0, 0.8);
pub const PATH_LINE_COLOR: Color = Color::srgba(1.0, 1.0, 0.0, 0.5);
pub const AGENT_CENTER_COLOR: Color = Color::srgba(0.8, 0.0, 0.8, 1.0);
pub const AGENT_OUTLINE_COLOR: Color = Color::srgba(0.8, 0.0, 0.8, 0.5);

pub const GRID_WALKABLE_SIZE: f32 = 1.0;
pub const GRID_BLOCKED_SIZE: f32 = 2.0;
pub const PATH_POINT_SIZE: f32 = 1.0;
pub const PATH_LINE_THICKNESS: f32 = 1.0;
pub const AGENT_CENTER_SIZE: f32 = 2.0;
pub const AGENT_OUTLINE_THICKNESS: f32 = 1.0;
pub const AGENT_OUTLINE_SEGMENTS: usize = 16;

markers! {
    NavGridVisualMarker,
    NavPathVisualMarker,
    AgentCenterVisualMarker,
}