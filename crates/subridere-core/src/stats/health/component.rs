use bevy::prelude::*;

/// Компонент здоровья.
/// `max` и `regen` синхронизируются из ComputedStats.
#[derive(Component, Clone, Copy, Debug)]
pub struct Health {
    pub current: f32,
    pub max: f32,
    pub regen: f32,
}

impl Default for Health {
    fn default() -> Self {
        // Значения для базовых атрибутов (fortitude = 3)
        Self {
            current: 65.0,
            max: 65.0,
            regen: 0.3,
        }
    }
}

impl Health {
    pub fn new(max: f32, regen: f32) -> Self {
        Self {
            current: max,
            max,
            regen,
        }
    }

    /// Создать полностью заполненное здоровье с указанным максимумом
    pub fn full(max: f32) -> Self {
        Self {
            current: max,
            max,
            regen: 0.0,
        }
    }

    /// Нанести урон
    pub fn damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }

    /// Восстановить здоровье
    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    /// Жив ли?
    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }

    /// Мёртв ли?
    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }

    /// Процент здоровья (0.0 - 1.0)
    pub fn percent(&self) -> f32 {
        if self.max > 0.0 {
            self.current / self.max
        } else {
            0.0
        }
    }

    /// Полное ли здоровье?
    pub fn is_full(&self) -> bool {
        self.current >= self.max
    }
}
