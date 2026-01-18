// fighting/components.rs

use bevy::prelude::*;

/// Фаза атаки (Souls-like)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AttackPhase {
    #[default]
    Windup,
    Active,
    Recovery,
}

/// Тип атаки
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AttackType {
    #[default]
    Light,
    Heavy,
}

/// Тип оружия — определяет анимации и тайминги
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WeaponKind {
    #[default]
    Fists,
    Sword,
    // TODO: Staff, Hammer, Shield, Dagger...
}

/// Состояние одной руки
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ArmCombatState {
    #[default]
    Ready,

    /// Зарядка атаки (кнопка зажата)
    Charging {
        charge_timer: f32,
        weapon_kind: WeaponKind,
    },

    /// Атака (light или heavy)
    Attacking {
        attack_type: AttackType,
        phase: AttackPhase,
        phase_timer: f32,
        damage_dealt: bool,
        charge_level: f32,
        weapon_kind: WeaponKind,
    },
}

/// Компонент боевого состояния игрока (обе руки независимо)
#[derive(Component, Default)]
pub struct PlayerCombatState {
    pub right: ArmCombatState,
    pub left: ArmCombatState,
}

/// Конфигурация заряженной атаки
#[derive(Resource)]
pub struct ChargeConfig {
    /// Порог для тяжёлой атаки (секунды)
    pub heavy_threshold: f32,
    /// Время до 100% заряда (секунды)
    pub max_charge_time: f32,
    /// Множитель урона при charge_level = 0
    pub min_damage_mult: f32,
    /// Множитель урона при charge_level = 1
    pub max_damage_mult: f32,
    /// Множитель knockback при charge_level = 0
    pub min_knockback_mult: f32,
    /// Множитель knockback при charge_level = 1
    pub max_knockback_mult: f32,
}

impl Default for ChargeConfig {
    fn default() -> Self {
        Self {
            heavy_threshold: 0.3,
            max_charge_time: 1.0,
            min_damage_mult: 1.0,
            max_damage_mult: 1.5,
            min_knockback_mult: 1.0,
            max_knockback_mult: 2.0,
        }
    }
}

impl ChargeConfig {
    /// Вычисляет charge_level (0.0-1.0) из времени зарядки
    pub fn charge_level(&self, charge_timer: f32) -> f32 {
        if charge_timer < self.heavy_threshold {
            0.0
        } else {
            ((charge_timer - self.heavy_threshold) / (self.max_charge_time - self.heavy_threshold))
                .clamp(0.0, 1.0)
        }
    }

    /// Вычисляет множитель урона из charge_level
    pub fn damage_mult(&self, charge_level: f32) -> f32 {
        self.min_damage_mult + charge_level * (self.max_damage_mult - self.min_damage_mult)
    }

    /// Вычисляет множитель knockback из charge_level
    pub fn knockback_mult(&self, charge_level: f32) -> f32 {
        self.min_knockback_mult + charge_level * (self.max_knockback_mult - self.min_knockback_mult)
    }
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
        Self::fists()
    }
}

impl AttackTimings {
    // ─────────────────────────────────────────────────────────────
    // FISTS (кулаки)
    // ─────────────────────────────────────────────────────────────

    pub fn fists() -> Self {
        Self {
            windup: 0.10,
            active: 0.12,
            recovery: 0.20,
        }
    }

    pub fn fists_heavy() -> Self {
        Self {
            windup: 0.28,
            active: 0.22,
            recovery: 0.45,
        }
    }

    // ─────────────────────────────────────────────────────────────
    // SWORD (меч)
    // ─────────────────────────────────────────────────────────────

    pub fn sword() -> Self {
        Self {
            windup: 0.12,
            active: 0.15,
            recovery: 0.25,
        }
    }

    pub fn sword_heavy() -> Self {
        Self {
            windup: 0.25,
            active: 0.20,
            recovery: 0.40,
        }
    }

    // ─────────────────────────────────────────────────────────────
    // TODO: другие типы оружия
    // ─────────────────────────────────────────────────────────────

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

    /// Получить тайминги по типу оружия и типу атаки
    pub fn for_weapon(kind: WeaponKind, attack_type: AttackType) -> Self {
        match (kind, attack_type) {
            (WeaponKind::Fists, AttackType::Light) => Self::fists(),
            (WeaponKind::Fists, AttackType::Heavy) => Self::fists_heavy(),
            (WeaponKind::Sword, AttackType::Light) => Self::sword(),
            (WeaponKind::Sword, AttackType::Heavy) => Self::sword_heavy(),
            // TODO: другие комбинации
        }
    }
}

/// Resource: текущие тайминги атаки для каждой руки
#[derive(Resource)]
pub struct CurrentAttackTimings {
    pub right_light: AttackTimings,
    pub right_heavy: AttackTimings,
    pub left_light: AttackTimings,
    pub left_heavy: AttackTimings,
    pub right_weapon: WeaponKind,
    pub left_weapon: WeaponKind,
}

impl Default for CurrentAttackTimings {
    fn default() -> Self {
        Self {
            right_light: AttackTimings::fists(),
            right_heavy: AttackTimings::fists_heavy(),
            left_light: AttackTimings::fists(),
            left_heavy: AttackTimings::fists_heavy(),
            right_weapon: WeaponKind::Fists,
            left_weapon: WeaponKind::Fists,
        }
    }
}

impl CurrentAttackTimings {
    /// Обновить тайминги для руки
    pub fn set_weapon(&mut self, side: crate::player::arm::ArmSide, kind: WeaponKind) {
        use crate::player::arm::ArmSide;
        match side {
            ArmSide::Right => {
                self.right_weapon = kind;
                self.right_light = AttackTimings::for_weapon(kind, AttackType::Light);
                self.right_heavy = AttackTimings::for_weapon(kind, AttackType::Heavy);
            }
            ArmSide::Left => {
                self.left_weapon = kind;
                self.left_light = AttackTimings::for_weapon(kind, AttackType::Light);
                self.left_heavy = AttackTimings::for_weapon(kind, AttackType::Heavy);
            }
        }
    }

    /// Получить тайминги для руки и типа атаки
    pub fn get(
        &self,
        side: crate::player::arm::ArmSide,
        attack_type: AttackType,
    ) -> &AttackTimings {
        use crate::player::arm::ArmSide;
        match (side, attack_type) {
            (ArmSide::Right, AttackType::Light) => &self.right_light,
            (ArmSide::Right, AttackType::Heavy) => &self.right_heavy,
            (ArmSide::Left, AttackType::Light) => &self.left_light,
            (ArmSide::Left, AttackType::Heavy) => &self.left_heavy,
        }
    }

    /// Получить тип оружия для руки
    pub fn weapon(&self, side: crate::player::arm::ArmSide) -> WeaponKind {
        use crate::player::arm::ArmSide;
        match side {
            ArmSide::Right => self.right_weapon,
            ArmSide::Left => self.left_weapon,
        }
    }
}
