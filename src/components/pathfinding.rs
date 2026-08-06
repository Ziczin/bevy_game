use bevy::prelude::*;

#[derive(Component)]
pub struct Pathfinder {
    pub path: Vec<Vec2>,
    pub current_waypoint: usize,
    pub update_timer: f32,
    pub update_interval: f32,
    pub current_target: Option<Vec2>,
    pub is_active: bool,
    pub agent_half_size: Vec2,
    pub arrival_threshold: f32,
    pub grid_generation: usize,
    pub next_update_time: f32,
    pub use_raycast: bool,
}

impl Pathfinder {
    pub fn new(
        update_interval: f32,
        agent_half_size: Vec2,
        arrival_threshold: f32,
    ) -> Self {
        let delay = rand::random::<f32>() * update_interval;
        Self {
            path: Vec::new(),
            current_waypoint: 0,
            update_timer: 0.0,
            update_interval,
            current_target: None,
            is_active: false,
            agent_half_size,
            arrival_threshold,
            grid_generation: 0,
            next_update_time: delay,
            use_raycast: false,
        }
    }
}
