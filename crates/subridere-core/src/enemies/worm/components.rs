use bevy::prelude::*;

/// Worm-specific component
///
/// Stores information about worm's segments and behavior parameters
#[derive(Component)]
pub struct Worm {
    /// Number of body segments (excluding head and tail)
    pub segment_count: usize,
}

impl Default for Worm {
    fn default() -> Self {
        Self {
            segment_count: 4, // matches worm.ron model
        }
    }
}

#[derive(Component)]
pub struct WormAI {
    /// Current target entity (if any)
    pub target: Option<Entity>,

    /// Movement speed in meters per second
    pub move_speed: f32,

    /// Detection range (meters)
    pub detection_range: f32,

    /// Attack range (meters)
    pub attack_range: f32,
}

impl Default for WormAI {
    fn default() -> Self {
        Self {
            target: None,
            move_speed: 2.0,
            detection_range: 50.0,
            attack_range: 1.0,
        }
    }
}
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum WormState {
    /// Standing still, looking around
    Idle,

    /// Moving towards target
    Chase { target: Entity },

    /// Close enough to attack
    Attack { target: Entity },
}

impl Default for WormState {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Component)]
pub struct WormHead {
    pub worm_root: Entity,
}
