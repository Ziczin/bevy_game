use bevy::prelude::*;

#[derive(Component)]
pub struct Pathfinder {
    pub path: Vec<Vec2>,
    pub current_waypoint: usize,
    pub update_timer: f32,
    pub update_interval: f32,
    pub current_target: Option<Vec2>,
    pub is_active: bool, 
}

impl Pathfinder {
    pub fn new(update_interval: f32) -> Self {
        Self {
            path: Vec::new(),
            current_waypoint: 0,
            update_timer: 0.0,
            update_interval,
            current_target: None,
            is_active: false,
        }
    }
}