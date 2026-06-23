// FILE: src/modules/health/components.rs
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DamageType {
    Physical = 0,
    Magical = 1,
}

impl DamageType {
    pub const COUNT: usize = 2;

    pub fn index(&self) -> usize {
        *self as usize
    }
}

#[derive(Component, Debug)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn ratio(&self) -> f32 {
        (self.current / self.max).clamp(0.0, 1.0)
    }
}

#[derive(Component, Debug)]
pub struct Mana {
    pub current: f32,
    pub max: f32,
}

impl Mana {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn ratio(&self) -> f32 {
        (self.current / self.max).clamp(0.0, 1.0)
    }

    pub fn set_percent(&mut self, percent: u8) {
        self.current = (self.max * percent as f32 / 100.0).clamp(0.0, self.max);
    }
}

#[derive(Component, Debug)]
pub struct Resistances {
    values: [f32; DamageType::COUNT],
}

impl Resistances {
    pub fn new() -> Self {
        Self {
            values: [0.0; DamageType::COUNT],
        }
    }

    pub fn set(&mut self, damage_type: DamageType, resistance: f32) {
        self.values[damage_type.index()] = resistance.clamp(0.0, 1.0);
    }

    pub fn get(&self, damage_type: DamageType) -> f32 {
        self.values[damage_type.index()]
    }

    pub fn average_resistance(&self, damage_types: &[DamageType]) -> f32 {
        if damage_types.is_empty() {
            return 0.0;
        }

        let total: f32 = damage_types.iter()
            .map(|dt| self.get(*dt))
            .sum();

        total / damage_types.len() as f32
    }
}

impl Default for Resistances {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Message, Debug)]
pub struct DamageEvent {
    pub target: Entity,
    pub amount: f32,
    pub damage_types: Vec<DamageType>,
}