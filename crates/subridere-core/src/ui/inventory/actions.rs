// ui/inventory/actions.rs — UI Actions for inventory interactions

use bevy::prelude::*;
use bevy_ui_actions::prelude::*;

use crate::inventory::component::{Equipment, Inventory};
use crate::items::{EquipmentSlot, ItemId, ItemRegistry, ItemStack};
use crate::player::component::Player;

use super::components::{EquipmentSlotUI, InventorySlotUI};

// ============================================================
// Drop to Inventory Slot
// ============================================================

/// Action: Drop item onto inventory slot (swap/move)
pub struct DropToInventorySlot {
    pub target_index: usize,
}

impl UiAction for DropToInventorySlot {
    fn execute(&self, world: &mut World) {
        // Get source from drag state
        let source = get_drag_source(world);

        let Some(source) = source else {
            return;
        };

        match source {
            DragSource::Inventory(source_index) => {
                if source_index == self.target_index {
                    return; // Same slot, nothing to do
                }
                swap_inventory_slots(world, source_index, self.target_index);
            }
            DragSource::Equipment(source_slot) => {
                unequip_to_slot(world, source_slot, self.target_index);
            }
        }
    }
}

// ============================================================
// Drop to Equipment Slot
// ============================================================

/// Action: Drop item onto equipment slot (equip)
pub struct DropToEquipmentSlot {
    pub target_slot: EquipmentSlot,
}

impl UiAction for DropToEquipmentSlot {
    fn execute(&self, world: &mut World) {
        let source = get_drag_source(world);

        let Some(source) = source else {
            return;
        };

        match source {
            DragSource::Inventory(source_index) => {
                equip_from_inventory(world, source_index, self.target_slot);
            }
            DragSource::Equipment(source_slot) => {
                if source_slot == self.target_slot {
                    return; // Same slot
                }
                // Swap equipment slots? Usually not needed, but could implement
                info!("Equipment to equipment swap not implemented");
            }
        }
    }
}

// ============================================================
// Right-click Actions
// ============================================================

/// Action: Use consumable item
pub struct UseItemAction {
    pub slot_index: usize,
}

impl UiAction for UseItemAction {
    fn execute(&self, world: &mut World) {
        // Get item from inventory
        let item_id = {
            let mut query = world.query_filtered::<&Inventory, With<Player>>();
            let Ok(inventory) = query.get_single(world) else {
                return;
            };

            inventory.get(self.slot_index).map(|stack| stack.id)
        };

        let Some(id) = item_id else {
            return;
        };

        // Check if consumable
        let is_consumable = {
            let registry = world.resource::<ItemRegistry>();
            let def = registry.get(id);
            matches!(def.category, crate::items::ItemCategory::Consumable(_))
        };

        if !is_consumable {
            info!("❌ {} is not consumable", id);
            return;
        }

        // Apply effect and remove from inventory
        apply_consumable_effect(world, id);

        // Remove one from inventory
        let mut query = world.query_filtered::<&mut Inventory, With<Player>>();
        if let Ok(mut inventory) = query.get_single_mut(world) {
            inventory.remove(id, 1);
            info!("✅ Used {}", id);
        }
    }
}

/// Action: Drop item to world
pub struct DropItemAction {
    pub source: DragSource,
}

impl UiAction for DropItemAction {
    fn execute(&self, world: &mut World) {
        // TODO: Implement dropping to world
        // Need player position, spawn_world_item, etc.
        info!("📤 Drop to world not yet implemented for {:?}", self.source);
    }
}

/// Action: Equip item from inventory (right-click quick equip)
pub struct QuickEquipAction {
    pub slot_index: usize,
}

impl UiAction for QuickEquipAction {
    fn execute(&self, world: &mut World) {
        // Get item and find appropriate slot
        let item_data = {
            let mut query = world.query_filtered::<&Inventory, With<Player>>();
            let Ok(inventory) = query.get_single(world) else {
                return;
            };

            inventory.get(self.slot_index).map(|stack| stack.id)
        };

        let Some(id) = item_data else {
            return;
        };

        // Get target slot from item definition
        let target_slot = {
            let registry = world.resource::<ItemRegistry>();
            let def = registry.get(id);
            def.equipment_slot()
        };

        let Some(slot) = target_slot else {
            info!("❌ {} cannot be equipped", id);
            return;
        };

        equip_from_inventory(world, self.slot_index, slot);
    }
}

/// Action: Unequip item (right-click on equipment)
pub struct QuickUnequipAction {
    pub slot: EquipmentSlot,
}

impl UiAction for QuickUnequipAction {
    fn execute(&self, world: &mut World) {
        // Find first empty inventory slot
        let empty_slot = {
            let mut query = world.query_filtered::<&Inventory, With<Player>>();
            let Ok(inventory) = query.get_single(world) else {
                return;
            };

            inventory
                .iter()
                .find(|(_, item)| item.is_none())
                .map(|(i, _)| i)
        };

        let Some(target_index) = empty_slot else {
            info!("❌ Inventory full, cannot unequip");
            return;
        };

        unequip_to_slot(world, self.slot, target_index);
    }
}

// ============================================================
// Helpers
// ============================================================

#[derive(Clone, Copy, Debug)]
pub enum DragSource {
    Inventory(usize),
    Equipment(EquipmentSlot),
}

fn get_drag_source(world: &mut World) -> Option<DragSource> {
    let drag_state = world.resource::<DragState>();
    let dragging_entity = drag_state.dragging?;

    // Check if it's inventory slot
    if let Some(inv_slot) = world.get::<InventorySlotUI>(dragging_entity) {
        return Some(DragSource::Inventory(inv_slot.index));
    }

    // Check if it's equipment slot
    if let Some(equip_slot) = world.get::<EquipmentSlotUI>(dragging_entity) {
        return Some(DragSource::Equipment(equip_slot.slot));
    }

    None
}

fn swap_inventory_slots(world: &mut World, a: usize, b: usize) {
    let mut query = world.query_filtered::<&mut Inventory, With<Player>>();

    if let Ok(mut inventory) = query.get_single_mut(world) {
        inventory.swap(a, b);
        info!("🔄 Swapped inventory slots {} ↔ {}", a, b);
    }
}

fn equip_from_inventory(world: &mut World, inv_slot: usize, equip_slot: EquipmentSlot) {
    // Validate item can go in this slot
    let item_id = {
        let mut query = world.query_filtered::<&Inventory, With<Player>>();
        let Ok(inventory) = query.get_single(world) else {
            return;
        };

        inventory.get(inv_slot).map(|stack| stack.id)
    };

    let Some(id) = item_id else {
        return;
    };

    // Check if item fits in slot
    let valid_slot = {
        let registry = world.resource::<ItemRegistry>();
        let def = registry.get(id);
        def.equipment_slot() == Some(equip_slot)
    };

    if !valid_slot {
        info!("❌ {} cannot be equipped in {:?}", id, equip_slot);
        return;
    }

    // Do the equip
    let mut query = world.query_filtered::<(&mut Inventory, &mut Equipment), With<Player>>();

    if let Ok((mut inventory, mut equipment)) = query.get_single_mut(world) {
        // Remove from inventory
        let stack = inventory.remove_slot(inv_slot);

        if let Some(stack) = stack {
            // If something already equipped, put it back in inventory
            if let Some(old_id) = equipment.unequip(equip_slot) {
                inventory.add_single(old_id);
            }

            // Equip new item
            equipment.equip(equip_slot, stack.id);
            info!("✅ Equipped {} to {:?}", stack.id, equip_slot);
        }
    }
}

fn unequip_to_slot(world: &mut World, equip_slot: EquipmentSlot, inv_slot: usize) {
    let mut query = world.query_filtered::<(&mut Inventory, &mut Equipment), With<Player>>();

    if let Ok((mut inventory, mut equipment)) = query.get_single_mut(world) {
        // Check target slot is empty
        if inventory.get(inv_slot).is_some() {
            info!("❌ Inventory slot {} not empty", inv_slot);
            return;
        }

        // Unequip
        if let Some(id) = equipment.unequip(equip_slot) {
            // Put directly in target slot
            // We need to access slots directly for this
            // For now, just add to inventory
            inventory.add_single(id);
            info!("✅ Unequipped {} from {:?}", id, equip_slot);
        }
    }
}

fn apply_consumable_effect(world: &mut World, id: ItemId) {
    use crate::items::{ConsumableData, ItemCategory};
    use crate::stats::health::component::Health;
    use crate::stats::mana::component::Mana;
    use crate::stats::stamina::component::Stamina;

    let effect = {
        let registry = world.resource::<ItemRegistry>();
        let def = registry.get(id);

        match &def.category {
            ItemCategory::Consumable(data) => Some(data.effect.clone()),
            _ => None,
        }
    };

    let Some(effect) = effect else {
        return;
    };

    let mut query = world
        .query_filtered::<(&mut Health, Option<&mut Mana>, Option<&mut Stamina>), With<Player>>();

    if let Ok((mut health, mana, stamina)) = query.get_single_mut(world) {
        use crate::items::definition::ConsumableEffect;

        match effect {
            ConsumableEffect::Heal(amount) => {
                health.current = (health.current + amount).min(health.max);
                info!("💚 Healed {} HP", amount);
            }
            ConsumableEffect::RestoreMana(amount) => {
                if let Some(mut m) = mana {
                    m.current = (m.current + amount).min(m.max);
                    info!("💙 Restored {} MP", amount);
                }
            }
            ConsumableEffect::RestoreStamina(amount) => {
                if let Some(mut s) = stamina {
                    s.current = (s.current + amount).min(s.max);
                    info!("💚 Restored {} SP", amount);
                }
            }
        }
    }
}
