use bevy::prelude::*;

use super::damage::apply_damage;
use super::health::regenerate_health;
use super::mana::regenerate_mana;
use super::recalculate::recalculate_stats;
use super::stamina::regenerate_stamina;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                // Пересчёт статов должен идти первым
                recalculate_stats,
                // Затем регенерация и урон
                regenerate_health,
                regenerate_mana,
                regenerate_stamina,
                apply_damage,
            )
                .chain(),
        );

        info!("✅ Stats plugin initialized");
    }
}

/// Bundle для сущности с полной системой статов (игрок, NPC с RPG статами)
#[derive(Bundle, Default)]
pub struct StatsBundle {
    pub attributes: super::Attributes,
    pub computed: super::ComputedStats,
    pub modifiers: super::StatModifiers,
    pub health: super::Health,
    pub mana: super::Mana,
    pub stamina: super::Stamina,
}

impl StatsBundle {
    /// Создать с кастомными атрибутами
    pub fn with_attributes(mut self, attrs: super::Attributes) -> Self {
        self.attributes = attrs;
        self
    }

    /// Создать с начальными очками для распределения
    pub fn with_unspent_points(mut self, points: u8) -> Self {
        self.attributes.unspent_points = points;
        self
    }
}

/// Bundle для простых врагов (без Attributes, только готовые статы)
#[derive(Bundle)]
pub struct SimpleStatsBundle {
    pub health: super::Health,
    pub computed: super::ComputedStats,
}

impl SimpleStatsBundle {
    pub fn new(max_health: f32, defense: f32) -> Self {
        Self {
            health: super::Health::full(max_health),
            computed: super::ComputedStats {
                physical_defense: defense,
                ..default()
            },
        }
    }
}
