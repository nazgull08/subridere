use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub struct Stamina {
    pub current: f32,
    pub max: f32,
    pub regen: f32,
}

impl Default for Stamina {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
            regen: 0.0,
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

    pub fn damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }

    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }
}
