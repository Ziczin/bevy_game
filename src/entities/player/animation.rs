use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

pub fn create_walk_animation(
    spritesheet: &Spritesheet,
    animations: &mut ResMut<Assets<Animation>>,
) -> Handle<Animation> {
    return animations.add(
        spritesheet
        .create_animation()
        .add_partial_row(0, 1..)
        .add_cell(3, 0)
        .add_cell(2, 0)
        .set_clip_direction(AnimationDirection::PingPong)
        .set_duration(AnimationDuration::PerFrame(70))
        .set_repetitions(AnimationRepeat::Loop)
        .build()
    );
}

pub fn create_idle_animation(
    spritesheet: &Spritesheet,
    animations: &mut ResMut<Assets<Animation>>,
) -> Handle<Animation> {
    return animations.add(
        spritesheet
        .create_animation()
        .add_partial_row(0, ..3)
        .set_clip_direction(AnimationDirection::PingPong)
        .set_duration(AnimationDuration::PerFrame(1111))
        .build()
    );
}