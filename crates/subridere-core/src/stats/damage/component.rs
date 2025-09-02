use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Damage {
    pub amount: f32,
    pub damage_type: DamageType,
}

#[derive(Debug, Clone)]
pub enum DamageType {
    Physical,
    Magical,
}

#[derive(Component, Debug)]
pub struct HasDealtDamage;
