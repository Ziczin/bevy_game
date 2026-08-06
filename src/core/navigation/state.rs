use crate::core::config::from_toml;
use bevy::prelude::*;

from_toml!(
    "config/navigation/visual.toml",
    [
        PATH_POINT_COLOR_VEC: Vec4 = "colors.path_point",
        PATH_LINE_COLOR_VEC: Vec4 = "colors.path_line",
        AGENT_CENTER_COLOR_VEC: Vec4 = "colors.agent_center",
        AGENT_OUTLINE_COLOR_VEC: Vec4 = "colors.agent_outline",
        PATH_POINT_SIZE: f32 = "sizes.path_point",
        PATH_LINE_THICKNESS: f32 = "sizes.path_line_thickness",
        AGENT_OUTLINE_THICKNESS: f32 = "sizes.agent_outline_thickness",
        AGENT_OUTLINE_SCALE: f32 = "sizes.agent_outline_scale",
    ]
);

from_toml!(
    "config/navigation/navigation.toml",
    [
        ASTAR_ORTHOGONAL_COST: i32 = "astar.orthogonal_cost",
        ASTAR_DIAGONAL_COST: i32 = "astar.diagonal_cost",
        ASTAR_MAX_EXPANSIONS: usize = "astar.max_expansions",
        ASTAR_MAX_SEARCHES_PER_FRAME: usize =
            "astar.max_searches_per_frame",
        ASTAR_STEP_DISTANCE_DIVISOR: f32 =
            "astar.step_distance_divisor",
        ASTAR_MIN_STEP: f32 = "astar.min_step",
        ASTAR_MAX_STEP: f32 = "astar.max_step",
        ASTAR_AGENT_OCCUPIED_MULTIPLIER: f32 =
            "astar.agent_occupied_multiplier",
        ASTAR_AGENT_CHECK_RADIUS_MULTIPLIER: f32 =
            "astar.agent_check_radius_multiplier",
        ASTAR_MAX_AGENT_CHECKS_PER_CELL: usize =
            "astar.max_agent_checks_per_cell",
        RAYCAST_COLLIDER_RADIUS_SCALE: f32 =
            "raycast.collider_radius_scale",
        RAYCAST_MIN_DISTANCE: f32 = "raycast.min_distance",
        RAYCAST_STEP_RADIUS_SCALE: f32 = "raycast.step_radius_scale",
        RAYCAST_MIN_STEP: f32 = "raycast.min_step",
        RAYCAST_AGENT_COLLISION_MULTIPLIER: f32 =
            "raycast.agent_collision_multiplier",
        COLLIDER_MIN_SIZE: f32 = "physics.collider_min_size",
        NO_ROTATION: f32 = "physics.no_rotation",
    ]
);

pub static PATH_POINT_COLOR: std::sync::LazyLock<Color> =
    std::sync::LazyLock::new(|| {
        Color::srgba(
            PATH_POINT_COLOR_VEC.x,
            PATH_POINT_COLOR_VEC.y,
            PATH_POINT_COLOR_VEC.z,
            PATH_POINT_COLOR_VEC.w,
        )
    });

pub static PATH_LINE_COLOR: std::sync::LazyLock<Color> =
    std::sync::LazyLock::new(|| {
        Color::srgba(
            PATH_LINE_COLOR_VEC.x,
            PATH_LINE_COLOR_VEC.y,
            PATH_LINE_COLOR_VEC.z,
            PATH_LINE_COLOR_VEC.w,
        )
    });

pub static AGENT_CENTER_COLOR: std::sync::LazyLock<Color> =
    std::sync::LazyLock::new(|| {
        Color::srgba(
            AGENT_CENTER_COLOR_VEC.x,
            AGENT_CENTER_COLOR_VEC.y,
            AGENT_CENTER_COLOR_VEC.z,
            AGENT_CENTER_COLOR_VEC.w,
        )
    });

pub static AGENT_OUTLINE_COLOR: std::sync::LazyLock<Color> =
    std::sync::LazyLock::new(|| {
        Color::srgba(
            AGENT_OUTLINE_COLOR_VEC.x,
            AGENT_OUTLINE_COLOR_VEC.y,
            AGENT_OUTLINE_COLOR_VEC.z,
            AGENT_OUTLINE_COLOR_VEC.w,
        )
    });
