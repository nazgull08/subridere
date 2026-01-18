// fighting/weapon.rs
//
// Хелперы для работы с оружием в боевой системе

use bevy::prelude::*;

use crate::inventory::Equipment;
use crate::items::{EquipmentSlot, ItemCategory, ItemRegistry};
use crate::player::arm::ArmSide;

use super::components::WeaponKind;

/// Конвертирует ArmSide в EquipmentSlot
pub fn arm_to_slot(side: ArmSide) -> EquipmentSlot {
    match side {
        ArmSide::Right => EquipmentSlot::MainHand,
        ArmSide::Left => EquipmentSlot::OffHand,
    }
}

/// Определяет тип оружия по экипировке
pub fn get_weapon_kind(
    side: ArmSide,
    equipment: &Equipment,
    registry: &ItemRegistry,
) -> WeaponKind {
    let slot = arm_to_slot(side);

    let Some(item_id) = equipment.get(slot) else {
        return WeaponKind::Fists;
    };

    let def = registry.get(item_id);

    match &def.category {
        ItemCategory::Weapon(weapon_data) => {
            // Магическое оружие (посох) — пока Fists
            if weapon_data.mana_cost > 0.0 {
                // TODO: WeaponKind::Staff
                return WeaponKind::Fists;
            }

            // Определяем по id или характеристикам
            // TODO: добавить поле weapon_type в WeaponData
            let item_name = def.id.as_str();

            if item_name.contains("sword") || item_name.contains("blade") {
                WeaponKind::Sword
            } else if item_name.contains("hammer") || item_name.contains("mace") {
                // TODO: WeaponKind::Hammer
                WeaponKind::Fists
            } else if item_name.contains("dagger") || item_name.contains("knife") {
                // TODO: WeaponKind::Dagger
                WeaponKind::Fists
            } else {
                // Неизвестное оружие — используем как кулаки
                WeaponKind::Fists
            }
        }
        // Щит в руке — пока Fists
        // TODO: WeaponKind::Shield
        _ => WeaponKind::Fists,
    }
}

/// Система синхронизации оружия с таймингами
pub fn sync_weapon_timings(
    player_query: Query<&Equipment, Changed<Equipment>>,
    registry: Res<ItemRegistry>,
    mut timings: ResMut<super::components::CurrentAttackTimings>,
) {
    for equipment in &player_query {
        // Правая рука
        let right_kind = get_weapon_kind(ArmSide::Right, equipment, &registry);
        if timings.right_weapon != right_kind {
            timings.set_weapon(ArmSide::Right, right_kind);
            info!("⚔️ Right hand weapon: {:?}", right_kind);
        }

        // Левая рука
        let left_kind = get_weapon_kind(ArmSide::Left, equipment, &registry);
        if timings.left_weapon != left_kind {
            timings.set_weapon(ArmSide::Left, left_kind);
            info!("⚔️ Left hand weapon: {:?}", left_kind);
        }
    }
}
