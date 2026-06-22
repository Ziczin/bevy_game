// src/core/animation.rs
use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use crate::core::dto::{AnimationConfig, AnimationMode};

pub fn create_animation(
    spritesheet: &Spritesheet,
    animations: &mut ResMut<Assets<Animation>>,
    config: &AnimationConfig,
) -> Handle<Animation> {
    let mut builder = spritesheet.create_animation();
    
    for &frame in &config.frames {
        builder = builder.add_cell(frame, 0);
    }
    
    builder = builder.set_duration(AnimationDuration::PerFrame(config.duration_ms as u32));
    
    match config.mode {
        AnimationMode::Loop => {
            builder = builder.set_repetitions(AnimationRepeat::Loop);
        }
        AnimationMode::PingPong => {
            builder = builder.set_clip_direction(AnimationDirection::PingPong);
            builder = builder.set_repetitions(AnimationRepeat::Loop);
        }
        AnimationMode::Once => {}
    }
    
    animations.add(builder.build())
}