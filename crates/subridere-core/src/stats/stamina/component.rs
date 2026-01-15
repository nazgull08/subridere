use bevy::prelude::*;

/// Компонент стамины.
/// `max` и `regen` синхронизируются из ComputedStats.
#[derive(Component, Clone, Copy, Debug)]
pub struct Stamina {
    pub current: f32,
    pub max: f32,
    pub regen: f32,
}

impl Default for Stamina {
    fn default() -> Self {
        // Значения для базовых атрибутов (resolve = 3)
        Self {
            current: 59.0,
            max: 59.0,
            regen: 6.5,
        }
    }
}

impl Stamina {
    pub fn new(max: f32, regen: f32) -> Self {
        Self {
            current: max,
            max,
            regen,
        }
    }

    /// Создать полностью заполненную стамину
    pub fn full(max: f32) -> Self {
        Self {
            current: max,
            max,
            regen: 0.0,
        }
    }

    /// Потратить стамину (возвращает true если хватило)
    pub fn spend(&mut self, amount: f32) -> bool {
        if self.current >= amount {
            self.current -= amount;
            true
        } else {
            false
        }
    }

    /// Восстановить стамину
    pub fn restore(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    /// Хватает ли стамины?
    pub fn has(&self, amount: f32) -> bool {
        self.current >= amount
    }

    /// Процент стамины (0.0 - 1.0)
    pub fn percent(&self) -> f32 {
        if self.max > 0.0 {
            self.current / self.max
        } else {
            0.0
        }
    }

    /// Полная ли стамина?
    pub fn is_full(&self) -> bool {
        self.current >= self.max
    }

    /// Пустая ли стамина?
    pub fn is_empty(&self) -> bool {
        self.current <= 0.0
    }

    /// Истощена ли стамина? (ниже 10%)
    pub fn is_exhausted(&self) -> bool {
        self.percent() < 0.1
    }
}
