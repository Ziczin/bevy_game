use bevy::prelude::*;
use bevy::text::{TextFont, TextColor, TextLayout, Justify};
use bevy::ui::{Node, PositionType, Val};
use bevy::ui::widget::Text;
use std::collections::HashMap;
use crate::components::core::DepthLayer;
use crate::core::config::from_toml;

from_toml!("config/debug/visual.toml", [
    OVERLAY_FONT_SIZE: f32 = "overlay.font_size",
]);

#[derive(Resource, Default)]
pub struct DebugOverlay {
    entries: HashMap<String, String>,
}

impl DebugOverlay {
    pub fn set(&mut self, key: impl Into<String>, value: impl ToString) {
        self.entries.insert(key.into(), value.to_string());
    }
    pub fn remove(&mut self, key: &str) {
        self.entries.remove(key);
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    fn format(&self) -> String {
        let mut lines: Vec<_> = self.entries
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect();
        lines.sort();
        lines.join("\n")
    }
}

#[derive(Component)]
struct OverlayText;

pub struct DebugOverlayPlugin;

impl Plugin for DebugOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugOverlay>()
            .add_systems(Startup, spawn_overlay)
            .add_systems(Update, update_overlay);
    }
}

fn spawn_overlay(mut commands: Commands) {
    commands.spawn((
        Text::default(),
        TextFont {
            font_size: *OVERLAY_FONT_SIZE,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(Justify::Left),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        OverlayText,
        DepthLayer::Ui(0),
    ));
}

fn update_overlay(
    overlay: Res<DebugOverlay>,
    mut text_query: Query<&mut Text, With<OverlayText>>,
) {
    if let Ok(mut text) = text_query.single_mut() {
        text.0 = overlay.format();
    }
}
