use bevy::prelude::*;

/// Marker for all entities that belong to current game run.
/// These will be despawned when returning to main menu.
#[derive(Component)]
pub struct GameEntity;
