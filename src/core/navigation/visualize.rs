use bevy::prelude::*;
use avian2d::prelude::*;
use crate::components::pathfinding::Pathfinder;
use crate::core::profiling::{ProfilingBuffer, profile_scope};
use super::nav_grid::NavGrid;
use super::state::{
    GRID_WALKABLE_COLOR, GRID_BLOCKED_COLOR,
    PATH_POINT_COLOR, PATH_LINE_COLOR,
    AGENT_CENTER_COLOR, AGENT_OUTLINE_COLOR,
    GRID_WALKABLE_SIZE, GRID_BLOCKED_SIZE,
    PATH_POINT_SIZE, PATH_LINE_THICKNESS,
    AGENT_OUTLINE_SCALE,
    NAV_GRID_UI_LAYER, NAV_PATH_UI_LAYER,
};

// Публичные маркеры для сущностей визуализации
#[derive(Component)]
pub struct NavGridVisualMarker;

#[derive(Component)]
pub struct NavPathVisualMarker;

#[derive(Resource, Default)]
pub struct NavigationVisualSettings {
    pub points: bool,
    pub paths: bool,
    pub agents: bool,
}

fn get_collider_world_position(
    transform: &Transform,
    children: &Children,
    child_query: &Query<(&Transform, Option<&Collider>)>,
) -> Vec2 {
    for child in children.iter() {
        if let Ok((child_transform, Some(_collider))) = child_query.get(child) {
            return transform.translation.xy() + child_transform.translation.xy();
        }
    }
    transform.translation.xy()
}

/// Визуализация сетки навигации с использованием спрайтов и кэшированием.
pub fn visualize_nav_grid(
    mut commands: Commands,
    grid: Option<Res<NavGrid>>,
    existing_visuals: Query<Entity, With<NavGridVisualMarker>>,
    settings: Res<NavigationVisualSettings>,
    profiling: Res<ProfilingBuffer>,
) {
    profile_scope!(&profiling, "core::navigation::visualize::visualize_nav_grid", &["pathfinding", "navgrid", "debug", "visual"]);
    if !settings.points {
        for entity in &existing_visuals {
            commands.entity(entity).despawn();
        }
        return;
    }
    let Some(grid) = grid else { return };

    for entity in &existing_visuals {
        commands.entity(entity).despawn();
    }

    let layer = *NAV_GRID_UI_LAYER;
    let z = layer.depth_value();
    let walkable_color = *GRID_WALKABLE_COLOR;
    let blocked_color = *GRID_BLOCKED_COLOR;
    let walkable_size = *GRID_WALKABLE_SIZE;
    let blocked_size = *GRID_BLOCKED_SIZE;

    for y in 0..grid.height {
        for x in 0..grid.width {
            if let Some((walkable, _)) = grid.get_cell(x, y) {
                let world_pos = grid.grid_to_world(x, y);
                let (color, size) = if walkable {
                    (walkable_color, walkable_size)
                } else {
                    (blocked_color, blocked_size)
                };
                commands.spawn((
                    Sprite {
                        color,
                        custom_size: Some(Vec2::splat(size)),
                        ..default()
                    },
                    Transform::from_xyz(world_pos.x, world_pos.y, z),
                    NavGridVisualMarker,
                ));
            }
        }
    }
}

/// Визуализация пути с использованием спрайтов.
pub fn visualize_nav_path(
    mut commands: Commands,
    pathfinder_query: Query<&Pathfinder, Changed<Pathfinder>>,
    existing_paths: Query<Entity, With<NavPathVisualMarker>>,
    settings: Res<NavigationVisualSettings>,
    profiling: Res<ProfilingBuffer>,
) {
    profile_scope!(&profiling, "core::navigation::visualize::visualize_nav_path", &["pathfinding", "path", "debug", "visual"]);
    if !settings.paths {
        for entity in &existing_paths {
            commands.entity(entity).despawn();
        }
        return;
    }

    for entity in &existing_paths {
        commands.entity(entity).despawn();
    }

    let layer = *NAV_PATH_UI_LAYER;
    let z = layer.depth_value();
    let point_color = *PATH_POINT_COLOR;
    let line_color = *PATH_LINE_COLOR;
    let point_size = *PATH_POINT_SIZE;
    let line_thickness = *PATH_LINE_THICKNESS;

    for pathfinder in &pathfinder_query {
        let points = &pathfinder.path;
        if points.len() < 2 {
            if let Some(&p) = points.first() {
                commands.spawn((
                    Sprite {
                        color: point_color,
                        custom_size: Some(Vec2::splat(point_size)),
                        ..default()
                    },
                    Transform::from_xyz(p.x, p.y, z),
                    NavPathVisualMarker,
                ));
            }
            continue;
        }

        for i in 0..points.len() - 1 {
            let start = points[i];
            let end = points[i + 1];
            let mid = (start + end) / 2.0;
            let length = start.distance(end);
            let angle = (end - start).y.atan2((end - start).x);
            commands.spawn((
                Sprite {
                    color: line_color,
                    custom_size: Some(Vec2::new(length, line_thickness)),
                    ..default()
                },
                Transform::from_xyz(mid.x, mid.y, z)
                    .with_rotation(Quat::from_rotation_z(angle)),
                NavPathVisualMarker,
            ));
        }

        for &p in points {
            commands.spawn((
                Sprite {
                    color: point_color,
                    custom_size: Some(Vec2::splat(point_size)),
                    ..default()
                },
                Transform::from_xyz(p.x, p.y, z),
                NavPathVisualMarker,
            ));
        }
    }
}

/// Визуализация агентов с помощью Gizmos.
pub fn visualize_agent_centers(
    pathfinder_query: Query<(Entity, &Transform, &Children, &Pathfinder)>,
    child_query: Query<(&Transform, Option<&Collider>)>,
    settings: Res<NavigationVisualSettings>,
    mut gizmos: Gizmos,
    profiling: Res<ProfilingBuffer>,
) {
    profile_scope!(&profiling, "core::navigation::visualize::visualize_agent_centers", &["pathfinding", "agent", "debug", "visual"]);
    if !settings.agents {
        return;
    }
    let center_color = *AGENT_CENTER_COLOR;
    let outline_color = *AGENT_OUTLINE_COLOR;
    let scale = *AGENT_OUTLINE_SCALE;

    for (_entity, transform, children, pathfinder) in &pathfinder_query {
        let center_pos = get_collider_world_position(transform, children, &child_query);
        let half_size = pathfinder.agent_half_size * scale;
        gizmos.circle_2d(center_pos, 0.5, center_color);
        gizmos.ellipse_2d(center_pos, half_size, outline_color);
    }
}
