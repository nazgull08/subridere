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
#[derive(Debug, Clone, PartialEq)]
pub enum CombatState {
    /// Готов к действию
    Ready,

    /// В процессе атаки
    Attacking {
        phase: AttackPhase,
        phase_timer: f32,
        damage_dealt: bool,
    },

    /// Hitstop — микро-пауза при попадании (souls-like feel)
    Hitstop {
        remaining: f32,
        /// Состояние, в которое вернёмся после hitstop
        return_phase: AttackPhase,
        return_timer: f32,
    },
}

impl Default for CombatState {
    fn default() -> Self {
        Self::Ready
    }
}

/// Компонент боевого состояния игрока
#[derive(Component, Default)]
pub struct PlayerCombatState {
    pub state: CombatState,
}

/// Тайминги атаки (можно будет менять для разного оружия)
#[derive(Debug, Clone, Copy)]
pub struct AttackTimings {
    pub windup: f32,   // Время замаха
    pub active: f32,   // Время активного хитбокса
    pub recovery: f32, // Время отката
}

impl Default for AttackTimings {
    fn default() -> Self {
        Self {
            windup: 0.12,   // Быстрый замах
            active: 0.15,   // Окно для попадания
            recovery: 0.25, // Откат (уязвимость)
        }
    }
}

impl AttackTimings {
    /// Тайминги для кулаков (быстрые)
    pub fn fists() -> Self {
        Self {
            windup: 0.10,
            active: 0.12,
            recovery: 0.20,
        }
    }

    /// Тайминги для меча (средние)
    pub fn sword() -> Self {
        Self {
            windup: 0.15,
            active: 0.18,
            recovery: 0.30,
        }
    }

    /// Тайминги для молота (медленные, но мощные)
    pub fn hammer() -> Self {
        Self {
            windup: 0.25,
            active: 0.20,
            recovery: 0.45,
        }
    }

    /// Общая длительность атаки
    pub fn total(&self) -> f32 {
        self.windup + self.active + self.recovery
    }
}

/// Resource: текущие тайминги атаки (зависят от оружия)
#[derive(Resource, Default)]
pub struct CurrentAttackTimings(pub AttackTimings);
