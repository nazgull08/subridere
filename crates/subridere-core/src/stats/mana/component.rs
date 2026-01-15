use bevy::prelude::*;

/// Компонент маны.
/// `max` и `regen` синхронизируются из ComputedStats.
#[derive(Component, Clone, Copy, Debug)]
pub struct Mana {
    pub current: f32,
    pub max: f32,
    pub regen: f32,
}

impl Default for Mana {
    fn default() -> Self {
        // Значения для базовых атрибутов (arcana = 3)
        Self {
            current: 32.0,
            max: 32.0,
            regen: 1.5,
        }
    }
}

impl Mana {
    pub fn new(max: f32, regen: f32) -> Self {
        Self {
            current: max,
            max,
            regen,
        }
    }

    /// Создать полностью заполненную ману
    pub fn full(max: f32) -> Self {
        Self {
            current: max,
            max,
            regen: 0.0,
        }
    }

    /// Потратить ману (возвращает true если хватило)
    pub fn spend(&mut self, amount: f32) -> bool {
        if self.current >= amount {
            self.current -= amount;
            true
        } else {
            false
        }
    }

    /// Восстановить ману
    pub fn restore(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    /// Хватает ли маны?
    pub fn has(&self, amount: f32) -> bool {
        self.current >= amount
    }

    /// Процент маны (0.0 - 1.0)
    pub fn percent(&self) -> f32 {
        if self.max > 0.0 {
            self.current / self.max
        } else {
            0.0
        }
    }

    /// Полная ли мана?
    pub fn is_full(&self) -> bool {
        self.current >= self.max
    }

    /// Пустая ли мана?
    pub fn is_empty(&self) -> bool {
        self.current <= 0.0
    }
}
