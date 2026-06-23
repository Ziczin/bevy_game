// FILE: src/modules/health/systems.rs
use bevy::prelude::*;
use super::components::*;

pub fn apply_damage_events(
    mut health_query: Query<&mut Health>,
    resistances_query: Query<&Resistances>,
    mut damage_events: MessageReader<DamageEvent>,
) {
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