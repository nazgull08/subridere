// fighting/components.rs

use bevy::prelude::*;

/// Фаза атаки (Souls-like)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AttackPhase {
    #[default]
    Windup, // Замах — можно отменить (roll, etc)
    Active,   // Hitbox активен — наносит урон
    Recovery, // Откат — уязвим, нельзя действовать
}

/// Состояние боевой системы игрока
#[derive(Debug, Clone, PartialEq, Default)]
pub enum CombatState {
    /// Готов к действию
    #[default]
    Ready,

    /// В процессе атаки
    Attacking {
        phase: AttackPhase,
        phase_timer: f32,
        damage_dealt: bool,
    },
}

/// Компонент боевого состояния игрока
#[derive(Component, Default)]
pub struct PlayerCombatState {
    pub state: CombatState,
}

/// Тайминги атаки (можно менять для разного оружия)
#[derive(Debug, Clone, Copy)]
pub struct AttackTimings {
    pub windup: f32,
    pub active: f32,
    pub recovery: f32,
}

impl Default for AttackTimings {
    fn default() -> Self {
        Self {
            windup: 0.50,
            active: 0.60,
            recovery: 1.00,
        }
    }
}

impl AttackTimings {
    pub fn fists() -> Self {
        Self {
            windup: 0.10,
            active: 0.12,
            recovery: 0.20,
        }
    }

    pub fn sword() -> Self {
        Self {
            windup: 0.15,
            active: 0.18,
            recovery: 0.30,
        }
    }

    pub fn hammer() -> Self {
        Self {
            windup: 0.25,
            active: 0.20,
            recovery: 0.45,
        }
    }

    pub fn total(&self) -> f32 {
        self.windup + self.active + self.recovery
    }
}

/// Resource: текущие тайминги атаки
#[derive(Resource, Default)]
pub struct CurrentAttackTimings(pub AttackTimings);
