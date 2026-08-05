use bevy::prelude::*;
use crate::core::profiling::{ProfilingBuffer, profile_scope};
use super::components::*;

pub fn apply_damage_events(
    resistances_query: Query<&Resistances>,
    profiling: Res<ProfilingBuffer>,
    mut health_query: Query<&mut Health>,
    mut damage_events: MessageReader<DamageEvent>,
) {
    profile_scope!(&profiling, "modules::health::systems::apply_damage_events", &["health", "damage", "combat", "event"]);
    for event in damage_events.read() {
        if let Ok(mut health) = health_query.get_mut(event.target) {
            let resistance = resistances_query
                .get(event.target)
                .map(|r| r.average_resistance(&event.damage_types))
                .unwrap_or(0.0);
            let final_damage = event.amount * (1.0 - resistance);
            health.current = (health.current - final_damage).max(0.0);
        }
    }
}