use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Level {
    pub current: u32,
    pub attribute_points: u32,
}

impl Default for Level {
    fn default() -> Self {
        Self {
            current: 1,
            attribute_points: 0,
        }
    }
}

impl Level {
    pub fn new(level: u32) -> Self {
        Self {
            current: level,
            attribute_points: 0,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Experience {
    pub current: u32,
    pub to_next_level: u32,
}

impl Default for Experience {
    fn default() -> Self {
        Self {
            current: 0,
            to_next_level: Self::required_for_level(2),
        }
    }
}

impl Experience {
    /// XP для достижения уровня: 100 * level^1.5
    pub fn required_for_level(level: u32) -> u32 {
        (100.0 * (level as f32).powf(1.5)) as u32
    }

    pub fn add(&mut self, amount: u32) {
        self.current += amount;
    }

    pub fn can_level_up(&self) -> bool {
        self.current >= self.to_next_level
    }

    pub fn level_up(&mut self, new_level: u32) {
        self.current -= self.to_next_level;
        self.to_next_level = Self::required_for_level(new_level + 1);
    }

    pub fn progress(&self) -> f32 {
        self.current as f32 / self.to_next_level as f32
    }
}

/// Компонент для врагов — сколько опыта даёт
#[derive(Component, Debug, Clone)]
pub struct ExperienceReward {
    pub amount: u32,
}

impl ExperienceReward {
    pub fn new(amount: u32) -> Self {
        Self { amount }
    }
}
