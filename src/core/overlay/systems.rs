use bevy::prelude::*;
use bevy::text::{TextFont, TextColor, TextLayout, Justify};
use bevy::ui::{Node, PositionType, Val};
use bevy::ui::widget::Text;
use crate::components::core::DepthLayer;
use crate::core::config::from_toml;
use super::buffer::DebugOverlay;

from_toml!("config/debug/visual.toml", [
    OVERLAY_FONT_SIZE: f32 = "overlay.font_size",
]);

#[derive(Component)]
pub struct OverlayText;

pub fn spawn_overlay(mut commands: Commands) {
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

pub fn update_overlay(
    overlay: Res<DebugOverlay>,
    active_tags: Res<OverlayActiveTags>,
    mut text_query: Query<&mut Text, With<OverlayText>>,
) {
    let content = overlay.format_filtered(&active_tags.0);
    if let Ok(mut text) = text_query.single_mut() {
        text.0 = content;
    }
}

#[derive(Resource)]
pub struct OverlayActiveTags(pub Vec<String>);
