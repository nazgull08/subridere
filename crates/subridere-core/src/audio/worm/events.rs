use bevy::prelude::*;

/// Event when worm bites player
#[derive(Event)]
pub struct WormBiteEvent;

/// Event when worm takes damage
#[derive(Event)]
pub struct WormHurtEvent;
