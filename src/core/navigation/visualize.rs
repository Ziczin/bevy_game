use bevy::prelude::*;
use avian2d::prelude::*;
use crate::components::core::DepthLayer;
use crate::components::pathfinding::Pathfinder;
use super::nav_grid::NavGrid;

#[derive(Component)]
pub struct NavGridVisualMarker;

#[derive(Component)]
pub struct NavPathVisualMarker;

#[derive(Component)]
pub struct AgentCenterVisualMarker;

#[derive(Resource)]
pub struct NavigationVisualSettings {
    pub enabled: bool,
}

impl Default for NavigationVisualSettings {
    fn default() -> Self {
        Self { enabled: false }
    }
}

/// Получает мировую позицию коллайдера агента
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

    let layer = DepthLayer::Ui(-10);
    let z = layer.depth_value();

    for y in 0..grid.height {
        for x in 0..grid.width {
            if let Some(cell) = grid.get_cell(x, y) {
                let world_pos = grid.grid_to_world(x, y);
                let color: Color;
                let size: f32;
                if cell.walkable {
                    color = Color::srgba(0.0, 1.0, 0.0, 0.3);
                } else {
                    color = Color::srgba(1.0, 0.0, 0.0, 0.9);
                };
                if cell.walkable {
                    size = 4.0;
                } else {
                    size = 6.0;
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

    let layer = DepthLayer::Ui(-9);
    let z = layer.depth_value();

    for pathfinder in &pathfinder_query {
        for waypoint in &pathfinder.path {
            commands.spawn((
                Sprite {
                    color: Color::srgba(1.0, 1.0, 0.0, 0.8),
                    custom_size: Some(Vec2::splat(4.0)),
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
                        color: Color::srgba(1.0, 1.0, 0.0, 0.5),
                        custom_size: Some(Vec2::new(length, 2.0)),
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

    let layer = DepthLayer::Ui(-8);
    let z = layer.depth_value();

    for (entity, transform, children, pathfinder) in &pathfinder_query {
        // Получаем позицию коллайдера (физический центр агента)
        let center_pos = get_collider_world_position(transform, children, &child_query);
        
        commands.spawn((
            Sprite {
                color: Color::srgba(0.8, 0.0, 0.8, 1.0),
                custom_size: Some(Vec2::splat(8.0)),
                ..default()
            },
            Transform::from_xyz(center_pos.x, center_pos.y, z),
            layer,
            AgentCenterVisualMarker,
        ));

        let half_size = pathfinder.agent_half_size;
        let segments = 16;
        
        for i in 0..segments {
            let angle1 = (i as f32 / segments as f32) * std::f32::consts::TAU;
            let angle2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;
            
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
                    color: Color::srgba(0.8, 0.0, 0.8, 0.5),
                    custom_size: Some(Vec2::new(length, 2.0)),
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