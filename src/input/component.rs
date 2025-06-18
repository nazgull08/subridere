use bevy::prelude::*;

/// Raw mouse delta used for camera control.
#[derive(Component, Default)]
pub struct MovementInput {
    pub mouse_delta: Vec2,
}

/// Marker component for player-controlled entities.
#[derive(Component)]
pub struct PlayerControlled;
