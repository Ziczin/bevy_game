use bevy::prelude::*;
use crate::components::core::DepthLayer;
use crate::core::config::from_toml;
#[allow(unused_imports)]
use crate::core::macros::bevy_custom::markers;

from_toml!("config/navigation/navigation.toml", [
    NAV_GRID_CELL_SIZE: f32 = "grid.cell_size",
    NAV_GRID_WIDTH: usize = "grid.width",
    NAV_GRID_HEIGHT: usize = "grid.height",
    ASTAR_ORTHOGONAL_COST: i32 = "astar.orthogonal_cost",
    ASTAR_DIAGONAL_COST: i32 = "astar.diagonal_cost",
    COLLIDER_MIN_SIZE: f32 = "physics.collider_min_size",
    ELLIPSE_THRESHOLD: f32 = "physics.ellipse_threshold",
    NO_ROTATION: f32 = "physics.no_rotation",
]);

from_toml!("config/navigation/visual.toml", [
    GRID_WALKABLE_COLOR_VEC: Vec4 = "colors.grid_walkable",
    GRID_BLOCKED_COLOR_VEC: Vec4 = "colors.grid_blocked",
    PATH_POINT_COLOR_VEC: Vec4 = "colors.path_point",
    PATH_LINE_COLOR_VEC: Vec4 = "colors.path_line",
    AGENT_CENTER_COLOR_VEC: Vec4 = "colors.agent_center",
    AGENT_OUTLINE_COLOR_VEC: Vec4 = "colors.agent_outline",
    GRID_WALKABLE_SIZE: f32 = "sizes.grid_walkable",
    GRID_BLOCKED_SIZE: f32 = "sizes.grid_blocked",
    PATH_POINT_SIZE: f32 = "sizes.path_point",
    PATH_LINE_THICKNESS: f32 = "sizes.path_line_thickness",
    AGENT_CENTER_SIZE: f32 = "sizes.agent_center",
    AGENT_OUTLINE_THICKNESS: f32 = "sizes.agent_outline_thickness",
    AGENT_OUTLINE_SEGMENTS: usize = "other.agent_outline_segments",
    NAV_GRID_UI_OFFSET: i32 = "layers.nav_grid_ui_offset",
    NAV_PATH_UI_OFFSET: i32 = "layers.nav_path_ui_offset",
    AGENT_CENTER_UI_OFFSET: i32 = "layers.agent_center_ui_offset",
]);

pub static NAV_GRID_UI_LAYER: std::sync::LazyLock<DepthLayer> = std::sync::LazyLock::new(|| DepthLayer::Ui(*NAV_GRID_UI_OFFSET as i16));
pub static NAV_PATH_UI_LAYER: std::sync::LazyLock<DepthLayer> = std::sync::LazyLock::new(|| DepthLayer::Ui(*NAV_PATH_UI_OFFSET as i16));
pub static AGENT_CENTER_UI_LAYER: std::sync::LazyLock<DepthLayer> = std::sync::LazyLock::new(|| DepthLayer::Ui(*AGENT_CENTER_UI_OFFSET as i16));

pub static GRID_WALKABLE_COLOR: std::sync::LazyLock<Color> = std::sync::LazyLock::new(|| Color::srgba(
    GRID_WALKABLE_COLOR_VEC.x, GRID_WALKABLE_COLOR_VEC.y, GRID_WALKABLE_COLOR_VEC.z, GRID_WALKABLE_COLOR_VEC.w,
));
pub static GRID_BLOCKED_COLOR: std::sync::LazyLock<Color> = std::sync::LazyLock::new(|| Color::srgba(
    GRID_BLOCKED_COLOR_VEC.x, GRID_BLOCKED_COLOR_VEC.y, GRID_BLOCKED_COLOR_VEC.z, GRID_BLOCKED_COLOR_VEC.w,
));
pub static PATH_POINT_COLOR: std::sync::LazyLock<Color> = std::sync::LazyLock::new(|| Color::srgba(
    PATH_POINT_COLOR_VEC.x, PATH_POINT_COLOR_VEC.y, PATH_POINT_COLOR_VEC.z, PATH_POINT_COLOR_VEC.w,
));
pub static PATH_LINE_COLOR: std::sync::LazyLock<Color> = std::sync::LazyLock::new(|| Color::srgba(
    PATH_LINE_COLOR_VEC.x, PATH_LINE_COLOR_VEC.y, PATH_LINE_COLOR_VEC.z, PATH_LINE_COLOR_VEC.w,
));
pub static AGENT_CENTER_COLOR: std::sync::LazyLock<Color> = std::sync::LazyLock::new(|| Color::srgba(
    AGENT_CENTER_COLOR_VEC.x, AGENT_CENTER_COLOR_VEC.y, AGENT_CENTER_COLOR_VEC.z, AGENT_CENTER_COLOR_VEC.w,
));
pub static AGENT_OUTLINE_COLOR: std::sync::LazyLock<Color> = std::sync::LazyLock::new(|| Color::srgba(
    AGENT_OUTLINE_COLOR_VEC.x, AGENT_OUTLINE_COLOR_VEC.y, AGENT_OUTLINE_COLOR_VEC.z, AGENT_OUTLINE_COLOR_VEC.w,
));

markers! {
    NavGridVisualMarker,
    NavPathVisualMarker,
    AgentCenterVisualMarker,
}