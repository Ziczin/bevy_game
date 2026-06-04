use bevy::prelude::*;
use avian2d::prelude::*;
use crate::components::pathfinding::Pathfinder;
use super::nav_grid::NavGrid;
use super::state::{
    NavGridVisualMarker, NavPathVisualMarker, AgentCenterVisualMarker,
    NAV_GRID_UI_LAYER, NAV_PATH_UI_LAYER, AGENT_CENTER_UI_LAYER,
    GRID_WALKABLE_COLOR, GRID_BLOCKED_COLOR,
    PATH_POINT_COLOR, PATH_LINE_COLOR,
    AGENT_CENTER_COLOR, AGENT_OUTLINE_COLOR,
    GRID_WALKABLE_SIZE, GRID_BLOCKED_SIZE,
    PATH_POINT_SIZE, PATH_LINE_THICKNESS,
    AGENT_CENTER_SIZE, AGENT_OUTLINE_THICKNESS, AGENT_OUTLINE_SEGMENTS,
};

#[derive(Resource)]
pub struct NavigationVisualSettings {
    pub enabled: bool,
}

impl Default for NavigationVisualSettings {
    fn default() -> Self {
        Self { enabled: false }
    }
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
    mut commands: Commands,
    grid: Option<Res<NavGrid>>,
    existing_visuals: Query<Entity, With<NavGridVisualMarker>>,
    settings: Res<NavigationVisualSettings>,
) {
    if !settings.enabled {
        return;
    }

    let Some(grid) = grid else { return; };

    for entity in &existing_visuals {
        commands.entity(entity).despawn();
    }

    let layer = NAV_GRID_UI_LAYER;
    let z = layer.depth_value();

    for y in 0..grid.height {
        for x in 0..grid.width {
            if let Some((walkable, _visible)) = grid.get_cell(x, y) {
                let world_pos = grid.grid_to_world(x, y);
                let (color, size) = if walkable {
                    (GRID_WALKABLE_COLOR, GRID_WALKABLE_SIZE)
                } else {
                    (GRID_BLOCKED_COLOR, GRID_BLOCKED_SIZE)
                };

                commands.spawn((
                    Sprite {
                        color,
                        custom_size: Some(Vec2::splat(size)),
                        ..default()
                    },
                    Transform::from_xyz(world_pos.x, world_pos.y, z),
                    layer,
                    NavGridVisualMarker,
                ));
            }
        }
    }
}

pub fn visualize_nav_path(
    mut commands: Commands,
    pathfinder_query: Query<&Pathfinder, Changed<Pathfinder>>,
    existing_paths: Query<Entity, With<NavPathVisualMarker>>,
    settings: Res<NavigationVisualSettings>,
) {
    if !settings.enabled {
        return;
    }

    for entity in &existing_paths {
        commands.entity(entity).despawn();
    }

    let layer = NAV_PATH_UI_LAYER;
    let z = layer.depth_value();

    for pathfinder in &pathfinder_query {
        for waypoint in &pathfinder.path {
            commands.spawn((
                Sprite {
                    color: PATH_POINT_COLOR,
                    custom_size: Some(Vec2::splat(PATH_POINT_SIZE)),
                    ..default()
                },
                Transform::from_xyz(waypoint.x, waypoint.y, z),
                layer,
                NavPathVisualMarker,
            ));
        }

        if pathfinder.path.len() > 1 {
            for i in 0..pathfinder.path.len() - 1 {
                let start = pathfinder.path[i];
                let end = pathfinder.path[i + 1];
                
                let mid = (start + end) / 2.0;
                let length = start.distance(end);
                let angle = (end - start).y.atan2((end - start).x);

                commands.spawn((
                    Sprite {
                        color: PATH_LINE_COLOR,
                        custom_size: Some(Vec2::new(length, PATH_LINE_THICKNESS)),
                        ..default()
                    },
                    Transform::from_xyz(mid.x, mid.y, z)
                        .with_rotation(Quat::from_rotation_z(angle)),
                    layer,
                    NavPathVisualMarker,
                ));
            }
        }
    }
}

pub fn visualize_agent_centers(
    mut commands: Commands,
    pathfinder_query: Query<(Entity, &Transform, &Children, &Pathfinder)>,
    child_query: Query<(&Transform, Option<&Collider>)>,
    existing_centers: Query<Entity, With<AgentCenterVisualMarker>>,
    settings: Res<NavigationVisualSettings>,
) {
    if !settings.enabled {
        return;
    }

    for entity in &existing_centers {
        commands.entity(entity).despawn();
    }

    let layer = AGENT_CENTER_UI_LAYER;
    let z = layer.depth_value();

    for (_entity, transform, children, pathfinder) in &pathfinder_query {
        let center_pos = get_collider_world_position(transform, children, &child_query);
        
        commands.spawn((
            Sprite {
                color: AGENT_CENTER_COLOR,
                custom_size: Some(Vec2::splat(AGENT_CENTER_SIZE)),
                ..default()
            },
            Transform::from_xyz(center_pos.x, center_pos.y, z),
            layer,
            AgentCenterVisualMarker,
        ));

        let half_size = pathfinder.agent_half_size;
        
        for i in 0..AGENT_OUTLINE_SEGMENTS {
            let angle1 = (i as f32 / AGENT_OUTLINE_SEGMENTS as f32) * std::f32::consts::TAU;
            let angle2 = ((i + 1) as f32 / AGENT_OUTLINE_SEGMENTS as f32) * std::f32::consts::TAU;
            
            let x1 = center_pos.x + angle1.cos() * half_size.x;
            let y1 = center_pos.y + angle1.sin() * half_size.y;
            let x2 = center_pos.x + angle2.cos() * half_size.x;
            let y2 = center_pos.y + angle2.sin() * half_size.y;
            
            let start = Vec2::new(x1, y1);
            let end = Vec2::new(x2, y2);
            let mid = (start + end) / 2.0;
            let length = start.distance(end);
            let angle = (end - start).y.atan2((end - start).x);

            commands.spawn((
                Sprite {
                    color: AGENT_OUTLINE_COLOR,
                    custom_size: Some(Vec2::new(length, AGENT_OUTLINE_THICKNESS)),
                    ..default()
                },
                Transform::from_xyz(mid.x, mid.y, z)
                    .with_rotation(Quat::from_rotation_z(angle)),
                layer,
                AgentCenterVisualMarker,
            ));
        }
    }
}