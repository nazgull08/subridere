use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use crate::fighting::projectile::weapons::{CurrentWeapon, WeaponType};
use crate::input::component::PlayerControlled;

/// Handles weapon switching via mouse wheel
pub fn weapon_switch_system(
    mut scroll_events: EventReader<MouseWheel>,
    mut players: Query<&mut CurrentWeapon, With<PlayerControlled>>,
) {
    for event in scroll_events.read() {
        if event.y.abs() > 0.1 {  // Ignore tiny movements
            for mut weapon in &mut players {
                weapon.weapon_type = match weapon.weapon_type {
                    WeaponType::MagicBolt => {
                        info!("🔄 Switched to Physical Cube");
                        WeaponType::PhysicalCube
                    }
                    WeaponType::PhysicalCube => {
                        info!("🔄 Switched to Magic Bolt");
                        WeaponType::MagicBolt
                    }
                };
            }
        }
    }
}
