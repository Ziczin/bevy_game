use bevy::prelude::*;

mod depth_ordered_draw;

pub struct BaseSystemsPlugin;

impl Plugin for BaseSystemsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, depth_ordered_draw::depth_ordered_draw);
    }
}