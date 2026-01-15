use bevy::prelude::*;

/// Вычисленные статы — результат (Attributes + Modifiers).
/// Эти значения используются всеми системами игры.
#[derive(Component, Clone, Debug)]
pub struct ComputedStats {
    // === Эффективные первичные (после модификаторов) ===
    pub might: f32,
    pub fortitude: f32,
    pub agility: f32,
    pub arcana: f32,
    pub resolve: f32,

    // === Ресурсы ===
    pub max_health: f32,
    pub max_mana: f32,
    pub max_stamina: f32,
    pub health_regen: f32,
    pub mana_regen: f32,
    pub stamina_regen: f32,

    // === Боевые ===
    pub melee_damage: f32,
    pub magic_damage: f32,
    pub attack_speed: f32,
    pub physical_defense: f32,
    pub magic_resist: f32,

    // === Движение ===
    pub move_speed: f32,
    pub dodge_frames: f32,

    // === Утилиты ===
    pub carry_capacity: f32,
    pub knockback_resist: f32,
    pub status_resist: f32,
}

impl Default for ComputedStats {
    fn default() -> Self {
        // Значения по умолчанию для базовых атрибутов = 3
        Self {
            // Первичные
            might: 3.0,
            fortitude: 3.0,
            agility: 3.0,
            arcana: 3.0,
            resolve: 3.0,

            // Ресурсы (при базе 3)
            max_health: 65.0,   // 50 + 3*5
            max_mana: 32.0,     // 20 + 3*4
            max_stamina: 59.0,  // 50 + 3*3
            health_regen: 0.3,  // 3 * 0.1
            mana_regen: 1.5,    // 3 * 0.5
            stamina_regen: 6.5, // 5 + 3*0.5

            // Боевые (при базе 3)
            melee_damage: 6.0,     // 3 * 2
            magic_damage: 6.0,     // 3 * 2
            attack_speed: 1.0,     // базовая
            physical_defense: 1.5, // 3 * 0.5
            magic_resist: 1.5,     // 3 * 0.5

            // Движение
            move_speed: 1.0,
            dodge_frames: 0.2,

            // Утилиты
            carry_capacity: 50.0,
            knockback_resist: 0.0,
            status_resist: 0.0,
        }
    }
}

impl ComputedStats {
    /// Получить эффективный первичный атрибут по типу
    pub fn get_attribute(&self, attr: crate::stats::attributes::AttributeType) -> f32 {
        use crate::stats::attributes::AttributeType;
        match attr {
            AttributeType::Might => self.might,
            AttributeType::Fortitude => self.fortitude,
            AttributeType::Agility => self.agility,
            AttributeType::Arcana => self.arcana,
            AttributeType::Resolve => self.resolve,
        }
    }
}
