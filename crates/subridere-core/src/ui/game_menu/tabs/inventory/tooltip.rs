//! Dynamic tooltip generation for inventory UI.

use bevy::prelude::*;
use bevy_ui_actions::{StatDiff, Tooltip, TooltipContent, TooltipSection, TooltipState};

use crate::inventory::component::{Equipment, Inventory};
use crate::items::{
    AccessoryData, ArmorData, ConsumableData, ConsumableEffect, ItemCategory, ItemDefinition,
    ItemRegistry,
};
use crate::player::component::Player;

use super::components::{SlotId, SlotUI};

// ============================================================
// Main System
// ============================================================

/// Dynamically generate tooltip content when hovering over slots.
pub fn update_hovered_tooltip(
    tooltip_state: Res<TooltipState>,
    player_query: Query<(&Inventory, &Equipment), With<Player>>,
    registry: Res<ItemRegistry>,
    slots: Query<&SlotUI>,
    mut tooltips: Query<&mut Tooltip>,
) {
    let Some(hovered_entity) = tooltip_state.hovered else {
        return;
    };

    let Ok(mut tooltip) = tooltips.get_mut(hovered_entity) else {
        return;
    };

    if !tooltip.content.is_empty() {
        return;
    }

    let Ok((inventory, equipment)) = player_query.single() else {
        return;
    };

    let Ok(slot_ui) = slots.get(hovered_entity) else {
        return;
    };

    tooltip.content = match slot_ui.id {
        SlotId::Inventory(index) => {
            build_inventory_slot_content(index, inventory, equipment, &registry)
        }
        SlotId::Equipment(slot) => build_equipment_slot_content(slot, equipment, &registry),
    };
}

/// Clear tooltip content when hover ends.
pub fn clear_tooltip_on_unhover(
    tooltip_state: Res<TooltipState>,
    mut last_hovered: Local<Option<Entity>>,
    mut tooltips: Query<&mut Tooltip>,
) {
    let current = tooltip_state.hovered;

    if *last_hovered != current {
        if let Some(old_entity) = *last_hovered {
            if let Ok(mut tooltip) = tooltips.get_mut(old_entity) {
                tooltip.content = TooltipContent::Empty;
            }
        }
        *last_hovered = current;
    }
}

// ============================================================
// Content Builders
// ============================================================

fn build_inventory_slot_content(
    index: usize,
    inventory: &Inventory,
    equipment: &Equipment,
    registry: &ItemRegistry,
) -> TooltipContent {
    let Some(stack) = inventory.get(index) else {
        return TooltipContent::Empty;
    };

    let def = registry.get(stack.id);

    let equipped_def = def
        .equipment_slot()
        .and_then(|slot| equipment.get(slot))
        .map(|id| registry.get(id));

    build_item_content(def, equipped_def, Some(stack.quantity))
}

fn build_equipment_slot_content(
    slot: crate::items::EquipmentSlot,
    equipment: &Equipment,
    registry: &ItemRegistry,
) -> TooltipContent {
    match equipment.get(slot) {
        Some(item_id) => {
            let def = registry.get(item_id);
            build_item_content(def, None, None)
        }
        None => TooltipContent::Text(slot.display_name().to_string()),
    }
}

// ============================================================
// Item Tooltip Builder
// ============================================================

fn build_item_content(
    def: &ItemDefinition,
    equipped: Option<&ItemDefinition>,
    quantity: Option<u32>,
) -> TooltipContent {
    let mut sections = Vec::new();

    // === Title ===
    let title = match quantity {
        Some(qty) if qty > 1 => format!("{} (x{})", def.name, qty),
        _ => def.name.clone(),
    };
    sections.push(TooltipSection::Title(title));

    // === Subtitle ===
    sections.push(TooltipSection::Subtitle(build_subtitle(def)));
    sections.push(TooltipSection::Separator);

    // === Stats ===
    match &def.category {
        ItemCategory::Weapon(w) => add_weapon_stats(&mut sections, w, equipped),
        ItemCategory::Armor(a) => add_armor_stats(&mut sections, a, equipped),
        ItemCategory::Accessory(a) => add_accessory_stats(&mut sections, a),
        ItemCategory::Consumable(c) => add_consumable_stats(&mut sections, c),
        ItemCategory::Misc => {}
    }

    // === Footer ===
    sections.push(TooltipSection::Separator);
    sections.push(TooltipSection::KeyValue(
        "Weight".to_string(),
        format!("{:.1}", def.weight),
    ));
    sections.push(TooltipSection::KeyValue(
        "Value".to_string(),
        format!("{}g", def.value),
    ));

    TooltipContent::Sections(sections)
}

fn build_subtitle(def: &ItemDefinition) -> String {
    let category = match &def.category {
        ItemCategory::Weapon(_) => "Weapon",
        ItemCategory::Armor(_) => "Armor",
        ItemCategory::Accessory(_) => "Accessory",
        ItemCategory::Consumable(_) => "Consumable",
        ItemCategory::Misc => "Misc",
    };

    match def.equipment_slot() {
        Some(slot) => format!("{} • {}", category, slot.display_name()),
        None => category.to_string(),
    }
}

// ============================================================
// Stat Sections
// ============================================================

fn add_weapon_stats(
    sections: &mut Vec<TooltipSection>,
    weapon: &crate::items::WeaponData,
    equipped: Option<&ItemDefinition>,
) {
    let eq_weapon = equipped.and_then(|e| match &e.category {
        ItemCategory::Weapon(w) => Some(w),
        _ => None,
    });

    sections.push(TooltipSection::Stat {
        label: "Damage".to_string(),
        value: format!("{:.0}", weapon.damage),
        diff: eq_weapon.map(|eq| calc_diff(weapon.damage, eq.damage)),
    });

    sections.push(TooltipSection::Stat {
        label: "Speed".to_string(),
        value: format!("{:.1}x", weapon.speed),
        diff: eq_weapon.map(|eq| calc_diff(weapon.speed, eq.speed)),
    });

    if weapon.mana_cost > 0.0 {
        sections.push(TooltipSection::Stat {
            label: "Mana Cost".to_string(),
            value: format!("{:.0}", weapon.mana_cost),
            diff: None,
        });
    }
}

fn add_armor_stats(
    sections: &mut Vec<TooltipSection>,
    armor: &ArmorData,
    equipped: Option<&ItemDefinition>,
) {
    let eq_armor = equipped.and_then(|e| match &e.category {
        ItemCategory::Armor(a) => Some(a),
        _ => None,
    });

    sections.push(TooltipSection::Stat {
        label: "Defense".to_string(),
        value: format!("{:.0}", armor.defense),
        diff: eq_armor.map(|eq| calc_diff(armor.defense, eq.defense)),
    });

    if armor.magic_resist > 0.0 {
        sections.push(TooltipSection::Stat {
            label: "Magic Resist".to_string(),
            value: format!("{:.0}", armor.magic_resist),
            diff: eq_armor.map(|eq| calc_diff(armor.magic_resist, eq.magic_resist)),
        });
    }

    for (target, op) in &armor.modifiers {
        sections.push(TooltipSection::Stat {
            label: target.display_name().to_string(),
            value: op.format_value(),
            diff: None,
        });
    }
}

fn add_accessory_stats(sections: &mut Vec<TooltipSection>, accessory: &AccessoryData) {
    for (target, op) in &accessory.modifiers {
        sections.push(TooltipSection::Stat {
            label: target.display_name().to_string(),
            value: op.format_value(),
            diff: None,
        });
    }
}

fn add_consumable_stats(sections: &mut Vec<TooltipSection>, consumable: &ConsumableData) {
    let effect_text = match &consumable.effect {
        ConsumableEffect::Heal(amount) => format!("Restores {:.0} HP", amount),
        ConsumableEffect::RestoreMana(amount) => format!("Restores {:.0} Mana", amount),
        ConsumableEffect::RestoreStamina(amount) => format!("Restores {:.0} Stamina", amount),
    };

    sections.push(TooltipSection::Stat {
        label: "Effect".to_string(),
        value: effect_text,
        diff: None,
    });
}

// ============================================================
// Helpers
// ============================================================

fn calc_diff(new: f32, current: f32) -> StatDiff {
    let delta = new - current;
    if delta > 0.01 {
        StatDiff::Better(delta)
    } else if delta < -0.01 {
        StatDiff::Worse(delta.abs())
    } else {
        StatDiff::Neutral
    }
}
