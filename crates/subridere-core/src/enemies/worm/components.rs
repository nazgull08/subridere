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
    pub target: Option<Entity>,
    pub move_force: f32,
    pub detection_range: f32,

    // ✅ НОВОЕ - параметры прыжка:
    pub jump_range: f32,         // Радиус для начала атаки (3.0м)
    pub jump_prepare_time: f32,  // Время подготовки (0.8 сек)
    pub jump_recovery_time: f32, // Время восстановления (1.5 сек)
    pub jump_force: f32,         // Сила прыжка (500.0)
    pub jump_height: f32,        // Высота прыжка (200.0)
}

impl Default for WormAI {
    fn default() -> Self {
        Self {
            target: None,
            move_force: 40.0,
            detection_range: 50.0,
            jump_range: 10.5,
            jump_prepare_time: 0.8,
            jump_recovery_time: 1.5,
            jump_force: 100.0,
            jump_height: 10.0,
        }
    }
}

#[derive(Component, Clone)]
pub enum WormState {
    Idle,
    Chase {
        target: Entity,
    },

    // ✅ НОВОЕ:
    PrepareAttack {
        target: Entity,
        prepare_timer: f32,
        target_pos: Vec3, // Запомненная позиция игрока
    },
    Lunging {
        target: Entity,
        target_pos: Vec3,
    },
    Recovering {
        recovery_timer: f32,
    },
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

#[derive(Component)]
pub struct WormSegment {
    /// Reference to the worm root entity
    pub worm_root: Entity,
    /// Segment index (0 = first after head, 1 = second, etc.)
    pub index: usize,
}
