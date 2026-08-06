use bevy::prelude::*;
use avian2d::prelude::*;
use crate::components::pathfinding::Pathfinder;
use crate::core::profiling::{ProfilingBuffer, profile_scope};
use super::state::{
    PATH_POINT_COLOR, PATH_LINE_COLOR,
    AGENT_CENTER_COLOR, AGENT_OUTLINE_COLOR,
    PATH_POINT_SIZE, PATH_LINE_THICKNESS,
    AGENT_OUTLINE_SCALE,
};

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

pub fn visualize_nav_grid(
    settings: Res<NavigationVisualSettings>,
) {
    // Сетка больше не используется, визуализация отключена
}

pub fn visualize_nav_path(
    pathfinder_query: Query<&Pathfinder, Changed<Pathfinder>>,
    settings: Res<NavigationVisualSettings>,
    mut gizmos: Gizmos,
    profiling: Res<ProfilingBuffer>,
) {
    profile_scope!(&profiling, "core::navigation::visualize::visualize_nav_path", &["pathfinding", "path", "debug", "visual"]);
    if !settings.paths {
        return;
    }
    let point_color = *PATH_POINT_COLOR;
    let line_color = *PATH_LINE_COLOR;
    let point_size = *PATH_POINT_SIZE;

    for pathfinder in &pathfinder_query {
        let points = &pathfinder.path;
        if points.len() < 2 {
            if let Some(&p) = points.first() {
                gizmos.circle_2d(p, point_size / 2.0, point_color);
            }
            continue;
        }
        for i in 0..points.len() - 1 {
            let start = points[i];
            let end = points[i + 1];
            gizmos.line_2d(start, end, line_color);
        }
        for &p in points {
            gizmos.circle_2d(p, point_size / 2.0, point_color);
        }
    }
}

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
