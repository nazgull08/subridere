// inventory/systems/equipment_stats.rs

use crate::inventory::component::Equipment;
use crate::items::{ItemCategory, ItemRegistry};
use crate::stats::modifiers::{ModifierOp, ModifierSource, ModifierTarget, StatModifiers};
use bevy::prelude::*;
use std::collections::HashSet;

/// Sync equipment changes to stat modifiers
pub fn sync_equipment_modifiers(
    mut query: Query<(&Equipment, &mut StatModifiers), Changed<Equipment>>,
    registry: Res<ItemRegistry>,
) {
    for (equipment, mut modifiers) in &mut query {
        // 1. Собрать текущие equipped item ids
        let equipped_ids: HashSet<_> = equipment.iter().map(|(_, id)| id).collect();

        // 2. Удалить модификаторы от снятых предметов
        modifiers.retain(|m| match &m.source {
            ModifierSource::Equipment(id) => equipped_ids.contains(id),
            _ => true,
        });

        // 3. Добавить модификаторы от новых предметов
        for (_, id) in equipment.iter() {
            if modifiers.has_source(&ModifierSource::Equipment(id)) {
                continue;
            }

            let def = registry.get(id);
            let item_mods = get_item_modifiers(def);
            modifiers.add_many(ModifierSource::Equipment(id), item_mods);
        }
    }
}

/// Extract modifiers from item definition
fn get_item_modifiers(def: &crate::items::ItemDefinition) -> Vec<(ModifierTarget, ModifierOp)> {
    let mut mods = Vec::new();

    match &def.category {
        ItemCategory::Armor(armor) => {
            // Стандартные поля
            if armor.defense > 0.0 {
                mods.push((
                    ModifierTarget::PhysicalDefense,
                    ModifierOp::Flat(armor.defense),
                ));
            }
            if armor.magic_resist > 0.0 {
                mods.push((
                    ModifierTarget::MagicResist,
                    ModifierOp::Flat(armor.magic_resist),
                ));
            }
            // Дополнительные модификаторы из RON
            mods.extend(armor.modifiers.iter().cloned());
        }
        ItemCategory::Accessory(accessory) => {
            mods.extend(accessory.modifiers.iter().cloned());
        }
        ItemCategory::Weapon(weapon) => {
            if weapon.damage > 0.0 {
                mods.push((ModifierTarget::MeleeDamage, ModifierOp::Flat(weapon.damage)));
            }
            if weapon.speed != 1.0 {
                mods.push((
                    ModifierTarget::AttackSpeed,
                    ModifierOp::Multiply(weapon.speed),
                ));
            }
        }
        _ => {}
    }

    mods
}
