use bevy::prelude::*;

/// Marker for projectiles that damage enemies
#[derive(Component)]
pub struct DamageProjectile;

/// Marker for physical cube projectiles (don't damage enemies)
#[derive(Component)]
pub struct PhysicalCube;

/// Available weapon types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponType {
    MagicBolt,    // Прямой снаряд, наносит урон
    PhysicalCube, // Физический куб, не наносит урон
}

/// Component tracking player's current weapon
#[derive(Component, Debug)]
pub struct CurrentWeapon {
    pub weapon_type: WeaponType,
}

impl Default for CurrentWeapon {
    fn default() -> Self {
        Self {
            weapon_type: WeaponType::MagicBolt,  // По умолчанию - прямые снаряды
        }
    }
}
