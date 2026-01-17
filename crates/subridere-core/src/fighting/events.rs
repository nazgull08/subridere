use bevy::prelude::*;

/// Событие: melee удар попал по цели
#[derive(Event)]
pub struct MeleeHitEvent {
    pub target: Entity,
    pub damage: f32,
}
