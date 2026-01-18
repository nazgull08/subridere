// crates/subridere-core/src/player/component.rs

use bevy::prelude::*;

/// Marks the player entity.
#[derive(Component)]
pub struct Player;

/// Visual settings for player's body.
#[derive(Component)]
pub struct PlayerVisual {
    pub body_color: Color,
    pub body_size: Vec3,
}

impl Default for PlayerVisual {
    fn default() -> Self {
        Self {
            body_color: Color::srgb(0.2, 0.6, 0.8),
            body_size: Vec3::new(0.6, 2.3, 0.3),
        }
    }
}

/// Where the player spawns initially and on death.
/// Поднято выше чтобы игрок не проваливался в пол
pub static PLAYER_START_POS: Vec3 = Vec3::new(0.0, 10.0, 0.0);
