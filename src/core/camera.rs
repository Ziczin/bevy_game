use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use bevy::window::{WindowResolution, WindowMode, MonitorSelection};
use avian2d::prelude::*;
use crate::core::config::from_toml;
use crate::core::profiling::{ProfilingBuffer, profile_scope};
use crate::components::core::ZoomState;
use crate::components::markers::Player;
use crate::modules::debug_overlay::DebugOverlay;
use crate::core::navigation::state::AGENT_OUTLINE_THICKNESS;

from_toml!("config/global/display.toml", [
    WINDOW_WIDTH: u32 = "display.window_width",
    WINDOW_HEIGHT: u32 = "display.window_height",
    SCALE_FACTOR_OVERRIDE: f32 = "display.scale_factor_override",
]);

from_toml!("config/global/display.toml", [
    GIZMO_AABB_COLOR: Vec4 = "gizmos.aabb_color",
    GIZMO_COLLIDER_COLOR: Vec4 = "gizmos.collider_color",
]);

#[derive(Resource)]
pub struct ZoomConfig {
    pub min_scale: f32,
    pub max_scale: f32,
    pub step: f32,
    pub lerp_factor: f32,
    pub lerp_speed_scale: f32,
}

#[derive(Resource)]
pub struct FollowConfig {
    pub factor: f32,
    pub speed_scale: f32,
}

from_toml!("config/camera.toml", [
    ZOOM_MIN_SCALE: f32 = "zoom.min_scale",
    ZOOM_MAX_SCALE: f32 = "zoom.max_scale",
    ZOOM_STEP: f32 = "zoom.step",
    ZOOM_LERP_FACTOR: f32 = "zoom.lerp_factor",
    ZOOM_LERP_SPEED_SCALE: f32 = "zoom.lerp_speed_scale",
    ZOOM_INITIAL_SCALE: f32 = "zoom.initial_scale",
    ZOOM_INITIAL_TARGET_SCALE: f32 = "zoom.initial_target_scale",
    FOLLOW_FACTOR: f32 = "follow.factor",
    FOLLOW_SPEED_SCALE: f32 = "follow.speed_scale",
]);

pub fn load_zoom_config() -> ZoomConfig {
    ZoomConfig {
        min_scale: *ZOOM_MIN_SCALE,
        max_scale: *ZOOM_MAX_SCALE,
        step: *ZOOM_STEP,
        lerp_factor: *ZOOM_LERP_FACTOR,
        lerp_speed_scale: *ZOOM_LERP_SPEED_SCALE,
    }
}

pub fn load_follow_config() -> FollowConfig {
    FollowConfig {
        factor: *FOLLOW_FACTOR,
        speed_scale: *FOLLOW_SPEED_SCALE,
    }
}

pub fn zoom_system(
    mut mouse_wheel_events: MessageReader<MouseWheel>,
    mut query: Query<(&mut ZoomState, &mut Projection), With<Camera2d>>,
    zoom_config: Res<ZoomConfig>,
    profiling: Res<ProfilingBuffer>,
    mut overlay: Option<ResMut<DebugOverlay>>,
) {
    profile_scope!(&profiling, "core::camera::zoom_system", &["camera", "zoom", "input"]);

    let mut delta = 0.0;
    for event in mouse_wheel_events.read() {
        delta += event.y;
    }

    if delta != 0.0 {
        if let Ok((mut zoom_state, _)) = query.single_mut() {
            let new_target = (zoom_state.target - delta * zoom_config.step)
                .clamp(zoom_config.min_scale, zoom_config.max_scale);
            zoom_state.target = new_target;
            if let Some(overlay) = overlay.as_mut() {
                overlay.set("Zoom target", format!("{:.2}", zoom_state.target));
            }
        }
    }
}

pub fn apply_zoom_lerp(
    time: Res<Time>,
    mut query: Query<(&mut ZoomState, &mut Projection), With<Camera2d>>,
    zoom_config: Res<ZoomConfig>,
    profiling: Res<ProfilingBuffer>,
    mut overlay: Option<ResMut<DebugOverlay>>,
) {
    profile_scope!(&profiling, "core::camera::apply_zoom_lerp", &["camera", "zoom", "lerp"]);

    if let Ok((mut zoom_state, mut projection)) = query.single_mut() {
        let t = (zoom_config.lerp_factor * zoom_config.lerp_speed_scale * time.delta_secs())
            .clamp(0.0, 1.0);
        zoom_state.current = zoom_state.current.lerp(zoom_state.target, t);

        if let Projection::Orthographic(ortho) = projection.as_mut() {
            ortho.scale = zoom_state.current;
        }

        if let Some(overlay) = overlay.as_mut() {
            overlay.set("Zoom current", format!("{:.2}", zoom_state.current));
            overlay.set("Zoom target", format!("{:.2}", zoom_state.target));
        }
    }
}

pub fn apply_camera_follow(
    time: Res<Time>,
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    follow_config: Res<FollowConfig>,
    profiling: Res<ProfilingBuffer>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    profile_scope!(&profiling, "core::camera::apply_camera_follow", &["camera", "follow", "lerp"]);
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let target_pos = player_transform.translation;
    let t = (follow_config.factor * follow_config.speed_scale * time.delta_secs())
        .clamp(0.0, 1.0);
    for mut transform in &mut camera_query {
        transform.translation = transform.translation.lerp(target_pos, t);
    }
}

pub fn configure_window(app: &mut App) {
    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(*WINDOW_WIDTH, *WINDOW_HEIGHT)
                        .with_scale_factor_override(*SCALE_FACTOR_OVERRIDE),
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            })
    );
}

pub fn configure_gizmos(app: &mut App) {
    app.insert_gizmo_config(
        PhysicsGizmos {
            aabb_color: Some(Color::srgba(
                GIZMO_AABB_COLOR.x,
                GIZMO_AABB_COLOR.y,
                GIZMO_AABB_COLOR.z,
                GIZMO_AABB_COLOR.w,
            )),
            collider_color: Some(Color::srgba(
                GIZMO_COLLIDER_COLOR.x,
                GIZMO_COLLIDER_COLOR.y,
                GIZMO_COLLIDER_COLOR.z,
                GIZMO_COLLIDER_COLOR.w,
            )),
            ..default()
        },
        GizmoConfig {
            line: GizmoLineConfig {
                width: *AGENT_OUTLINE_THICKNESS,
                ..default()
            },
            ..default()
        },
    );
}
