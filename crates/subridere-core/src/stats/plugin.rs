use bevy::prelude::*;

use crate::app::AppState;

use super::damage::apply_damage;
use super::health::{check_player_death, regenerate_health}; // ← ИЗМЕНИТЬ
use super::level::plugin::LevelPlugin;
use super::mana::regenerate_mana;
use super::recalculate::recalculate_stats;
use super::stamina::regenerate_stamina;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LevelPlugin).add_systems(
            Update,
            (
                recalculate_stats,
                regenerate_health,
                regenerate_mana,
                regenerate_stamina,
                apply_damage,
                check_player_death, // ← ДОБАВИТЬ
            )
                .chain()
                .run_if(in_state(AppState::InGame)), // ← ДОБАВИТЬ
        );

        info!("✅ Stats plugin initialized");
    }
}

/// Bundle для сущности с полной системой статов
#[derive(Bundle, Default)]
pub struct StatsBundle {
    pub level: super::Level,
    pub experience: super::Experience,
    pub attributes: super::Attributes,
    pub computed: super::ComputedStats,
    pub modifiers: super::StatModifiers,
    pub health: super::Health,
    pub mana: super::Mana,
    pub stamina: super::Stamina,
}

impl StatsBundle {
    pub fn with_attributes(mut self, attrs: super::Attributes) -> Self {
        self.attributes = attrs;
        self
    }

    pub fn with_unspent_points(mut self, points: u8) -> Self {
        self.attributes.unspent_points = points;
        self
    }

    pub fn with_level(mut self, level: u32) -> Self {
        self.level = super::Level::new(level);
        self
    }
}

/// Bundle для простых врагов
#[derive(Bundle)]
pub struct SimpleStatsBundle {
    pub health: super::Health,
    pub computed: super::ComputedStats,
    pub experience_reward: super::ExperienceReward,
}

impl SimpleStatsBundle {
    pub fn new(max_health: f32, defense: f32, xp_reward: u32) -> Self {
        Self {
            health: super::Health::full(max_health),
            computed: super::ComputedStats {
                physical_defense: defense,
                ..default()
            },
            experience_reward: super::ExperienceReward::new(xp_reward),
        }
    }
}
