use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct ExperienceGainEvent {
    pub entity: Entity,
    pub amount: u32,
}

#[derive(Event, Debug)]
pub struct LevelUpEvent {
    pub entity: Entity,
    pub new_level: u32,
}
