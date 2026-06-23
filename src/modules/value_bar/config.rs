// FILE: src/modules/value_bar/config.rs
use bevy::math::Vec4;
use crate::core::config::from_toml;

from_toml!("config/default/value_bar.toml", [
    DEFAULT_WIDTH: f32 = "default.width",
    DEFAULT_HEIGHT: f32 = "default.height",
    DEFAULT_OFFSET_X: f32 = "default.offset_x",
    DEFAULT_OFFSET_Y: f32 = "default.offset_y",
    COLOR_BACKGROUND: Vec4 = "colors.background",
    COLOR_CURRENT_HP: Vec4 = "colors.current_hp",
    COLOR_DELAYED_DAMAGE: Vec4 = "colors.delayed_damage",
    COLOR_DELAYED_HEAL: Vec4 = "colors.delayed_heal",
    DELAY_BEFORE_ANIMATION: f32 = "timing.delay_before_animation",
    ANIMATION_DURATION: f32 = "timing.animation_duration",
    VISIBILITY_TIMEOUT: f32 = "timing.visibility_timeout",
    FADE_DURATION: f32 = "timing.fade_duration",
]);

pub struct ValueBarColors {
    pub background: Vec4,
    pub current: Vec4,
    pub delayed_damage: Vec4,
    pub delayed_heal: Vec4,
}

impl Default for ValueBarColors {
    fn default() -> Self {
        Self {
            background: *COLOR_BACKGROUND,
            current: *COLOR_CURRENT_HP,
            delayed_damage: *COLOR_DELAYED_DAMAGE,
            delayed_heal: *COLOR_DELAYED_HEAL,
        }
    }
}

pub struct ValueBarConfig {
    pub width: f32,
    pub height: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub colors: ValueBarColors,
}

impl Default for ValueBarConfig {
    fn default() -> Self {
        Self {
            width: *DEFAULT_WIDTH,
            height: *DEFAULT_HEIGHT,
            offset_x: *DEFAULT_OFFSET_X,
            offset_y: *DEFAULT_OFFSET_Y,
            colors: ValueBarColors::default(),
        }
    }
}

impl ValueBarConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn with_offset(mut self, x: f32, y: f32) -> Self {
        self.offset_x = x;
        self.offset_y = y;
        self
    }

    pub fn with_colors(mut self, colors: ValueBarColors) -> Self {
        self.colors = colors;
        self
    }
}