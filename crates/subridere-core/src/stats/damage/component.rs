use bevy::prelude::*;

/// Компонент урона - добавляется на сущность чтобы нанести урон
#[derive(Component, Debug)]
pub struct Damage {
    pub amount: f32,
    pub damage_type: DamageType,
}

impl Damage {
    pub fn physical(amount: f32) -> Self {
        Self {
            amount,
            damage_type: DamageType::Physical,
        }
    }

    pub fn magical(amount: f32) -> Self {
        Self {
            amount,
            damage_type: DamageType::Magical,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamageType {
    Physical,
    Magical,
}

/// Маркер что сущность уже нанесла урон в этом кадре
#[derive(Component, Debug)]
pub struct HasDealtDamage;
