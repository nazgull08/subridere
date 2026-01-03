use bevy::prelude::*;
use crate::{
    inventory::{Equipment, Inventory},
    items::definition::{ItemDefinition, ItemProperties},
    player::component::Player,
    game_init::assets::GameAssets,
};
use super::{EquipmentSlotType, InventorySlotUI, EquipmentSlotUI, SelectedSlot};

/// Process item movement when clicking with a selection active
///
/// Handles:
/// - Inventory → Equipment (equip)
/// - Equipment → Inventory (unequip)
/// - Inventory → Inventory (swap)
pub fn process_item_actions(
    inventory_click: Query<(&Interaction, &InventorySlotUI), Changed<Interaction>>,
    equipment_click: Query<(&Interaction, &EquipmentSlotUI), Changed<Interaction>>,
    mut player_query: Query<(&mut Inventory, &mut Equipment), With<Player>>,
    mut selected_slot: ResMut<SelectedSlot>,
    game_assets: Res<GameAssets>,
    item_defs: Res<Assets<ItemDefinition>>,
) {
    let Ok((mut inventory, mut equipment)) = player_query.single_mut() else {
        return;
    };

    // Check for inventory slot clicks
    for (interaction, target_slot) in &inventory_click {
        if *interaction != Interaction::Pressed {
            continue;
        }
        
        // If we have an equipment slot selected, try to unequip
        if let Some(equip_slot_type) = selected_slot.equipment_slot {
            handle_unequip(
                &mut inventory,
                &mut equipment,
                equip_slot_type,
                target_slot.slot_index,
                &mut selected_slot,
            );
        }
        // If we have another inventory slot selected, try to swap
        else if let Some(source_slot) = selected_slot.inventory_slot && source_slot != target_slot.slot_index {
            handle_inventory_swap(
                &mut inventory,
                source_slot,
                target_slot.slot_index,
                &mut selected_slot,
            );
        }
    }

    // Check for equipment slot clicks
    for (interaction, target_slot) in &equipment_click {
        if *interaction != Interaction::Pressed {
            continue;
        }
        
        // If we have an inventory slot selected, try to equip
        if let Some(inv_slot) = selected_slot.inventory_slot {
            handle_equip(
                &mut inventory,
                &mut equipment,
                inv_slot,
                target_slot.slot_type,
                &mut selected_slot,
                &game_assets,
                &item_defs,
            );
        }
    }
}

/// Core equip logic (shared between click and menu)
pub fn equip_item_core(
    inventory: &mut crate::inventory::Inventory,
    equipment: &mut crate::inventory::Equipment,
    inv_slot: usize,
    equip_slot_type: EquipmentSlotType,
    game_assets: &crate::game_init::assets::GameAssets,
    item_defs: &Assets<crate::items::definition::ItemDefinition>,
) -> bool {
    // Get item_id from inventory (clone to release borrow)
    let item_id = {
        let Some(item) = inventory.slots.get(inv_slot).and_then(|s| s.as_ref()) else {
            warn!(" No item in selected inventory slot {}", inv_slot);
            return false;
        };
        item.item_id.clone()
    };

    // Check if item can be equipped in this slot
    if !can_equip_in_slot(&item_id, equip_slot_type, game_assets, item_defs) {
        info!("❌ Cannot equip {} in {:?} slot", item_id, equip_slot_type);
        return false;
    }

    // Get the equipment slot
    let equip_slot = equipment.get_slot_mut(equip_slot_type);

    // If slot already has item, swap back to inventory
    if let Some(old_item_id) = equip_slot.take() {
        inventory.slots[inv_slot] = Some(crate::inventory::InventorySlot {
            item_id: old_item_id,
            quantity: 1,
        });
    } else {
        // Remove from inventory
        inventory.slots[inv_slot] = None;
    }

    // Equip the new item
    *equip_slot = Some(item_id.clone());

    info!("✅ Equipped {} to {:?}", item_id, equip_slot_type);
    true
}

fn handle_equip(
    inventory: &mut Inventory,
    equipment: &mut Equipment,
    inv_slot: usize,
    equip_slot_type: EquipmentSlotType,
    selected: &mut SelectedSlot,
    game_assets: &GameAssets,
    item_defs: &Assets<ItemDefinition>,
) {
    equip_item_core(inventory, equipment, inv_slot, equip_slot_type, game_assets, item_defs);
    selected.clear();
}

/// Unequip item from equipment slot to inventory slot
fn handle_unequip(
    inventory: &mut Inventory,
    equipment: &mut Equipment,
    equip_slot_type: EquipmentSlotType,
    target_inv_slot: usize,
    selected: &mut SelectedSlot,
) {
    // Get the equipment slot
    let equip_slot = equipment.get_slot_mut(equip_slot_type);

    // Check if equipment slot has an item
    let Some(item_id) = equip_slot.take() else {
        warn!("⚠️ No item equipped in {:?}", equip_slot_type);
        selected.clear();
        return;
    };

    // Check if target inventory slot is empty
    if inventory.slots[target_inv_slot].is_some() {
        info!("❌ Target inventory slot {} is not empty", target_inv_slot);
        // Put item back
        *equip_slot = Some(item_id);
        selected.clear();
        return;
    }

    // Move to inventory
    inventory.slots[target_inv_slot] = Some(crate::inventory::InventorySlot {
        item_id,
        quantity: 1,
    });

    info!("✅ Unequipped to slot {}", target_inv_slot);
    selected.clear();
}

/// Swap two inventory slots
fn handle_inventory_swap(
    inventory: &mut Inventory,
    slot_a: usize,
    slot_b: usize,
    selected: &mut SelectedSlot,
) {
    // Swap the slots
    inventory.slots.swap(slot_a, slot_b);

    info!("🔄 Swapped inventory slots {} and {}", slot_a, slot_b);
    selected.clear();
}

/// Check if an item can be equipped in a specific slot
fn can_equip_in_slot(
    item_id: &str,
    equip_slot_type: EquipmentSlotType,
    game_assets: &GameAssets,
    item_defs: &Assets<ItemDefinition>,
) -> bool {
    // Get item definition handle
    let def_handle = match item_id {
        "wooden_staff" => &game_assets.wooden_staff_def,
        "iron_helmet" => &game_assets.iron_helmet_def,
        _ => return false,
    };

    // Get item definition
    let Some(item_def) = item_defs.get(def_handle) else {
        return false;
    };

    // Check based on item properties
    match &item_def.properties {
        ItemProperties::Weapon(_) => {
            // Weapons go in MainHand or OffHand
            matches!(equip_slot_type, EquipmentSlotType::MainHand | EquipmentSlotType::OffHand)
        }
        ItemProperties::Armor(armor_props) => {
            // Check if armor slot matches equipment slot
            armor_props.slot.matches_equipment_slot(&equip_slot_type)
        }
        ItemProperties::Consumable(_) => {
            // Consumables can't be equipped
            false
        }
    }
}


