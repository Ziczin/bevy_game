use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct ValueBar {
    pub owner: Entity,
    pub background_entity: Entity,
    pub current_entity: Entity,
    pub delayed_entity: Entity,
    pub value: f32,
    pub delayed_value: f32,
    pub target_delayed_value: f32,
    pub delay_timer: f32,
    pub animation_timer: f32,
    pub visibility_timer: f32,
    pub is_visible: bool,
    pub width: f32,
    pub height: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub background_padding: f32,
    pub background_color: Vec4,
    pub current_color: Vec4,
    pub delayed_damage_color: Vec4,
    pub delayed_heal_color: Vec4,
}