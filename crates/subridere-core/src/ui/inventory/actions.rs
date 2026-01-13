// ui/inventory/actions.rs — UI Actions for inventory interactions

use bevy::prelude::*;
use bevy_ui_actions::prelude::*;

use crate::inventory::component::{Equipment, Inventory};
use crate::inventory::systems::{DropSource, DropToWorldEvent};
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
        let Some(source) = get_drag_source(world) else {
            return;
        };

        match source {
            DropSource::Inventory(source_index) => {
                if source_index == self.target_index {
                    return;
                }
                swap_inventory_slots(world, source_index, self.target_index);
            }
            DropSource::Equipment(source_slot) => {
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
        let Some(source) = get_drag_source(world) else {
            return;
        };

        match source {
            DropSource::Inventory(source_index) => {
                equip_from_inventory(world, source_index, self.target_slot);
            }
            DropSource::Equipment(source_slot) => {
                if source_slot == self.target_slot {
                    return;
                }
                info!("Equipment to equipment swap not implemented");
            }
        }
    }
}

// ============================================================
// Drop to World (drag cancel)
// ============================================================

/// Action: Drop item to world (when drag cancelled outside UI)
pub struct DropToWorldAction;

impl UiAction for DropToWorldAction {
    fn execute(&self, world: &mut World) {
        info!("🔍 DropToWorldAction::execute() called");
        let Some(source) = get_drag_source(world) else {
            info!("🔍 No drag source found");
            return;
        };

        info!("🔍 Drag source: {:?}", source);
        // Check if slot actually has an item
        let has_item = match source {
            DropSource::Inventory(slot) => {
                let mut query = world.query_filtered::<&Inventory, With<Player>>();
                query
                    .get_single(world)
                    .map(|inv| inv.get(slot).is_some())
                    .unwrap_or(false)
            }
            DropSource::Equipment(slot) => {
                let mut query = world.query_filtered::<&Equipment, With<Player>>();
                query
                    .get_single(world)
                    .map(|eq| eq.get(slot).is_some())
                    .unwrap_or(false)
            }
        };

        if !has_item {
            return;
        }

        // Send event for deferred processing
        world.send_event(DropToWorldEvent { source });
        info!("📤 Queued drop to world: {:?}", source);
    }
}

// ============================================================
// Right-click Actions
// ============================================================

/// Action: Quick equip from inventory (right-click)
pub struct QuickEquipAction {
    pub slot_index: usize,
}

impl UiAction for QuickEquipAction {
    fn execute(&self, world: &mut World) {
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

        let target_slot = {
            let registry = world.resource::<ItemRegistry>();
            registry.get(id).equipment_slot()
        };

        let Some(slot) = target_slot else {
            info!("❌ {} cannot be equipped", id);
            return;
        };

        equip_from_inventory(world, self.slot_index, slot);
    }
}

/// Action: Quick unequip (right-click on equipment)
pub struct QuickUnequipAction {
    pub slot: EquipmentSlot,
}

impl UiAction for QuickUnequipAction {
    fn execute(&self, world: &mut World) {
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

/// Action: Use consumable item
pub struct UseItemAction {
    pub slot_index: usize,
}

impl UiAction for UseItemAction {
    fn execute(&self, world: &mut World) {
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

        let is_consumable = {
            let registry = world.resource::<ItemRegistry>();
            matches!(
                registry.get(id).category,
                crate::items::ItemCategory::Consumable(_)
            )
        };

        if !is_consumable {
            info!("❌ {} is not consumable", id);
            return;
        }

        apply_consumable_effect(world, id);

        let mut query = world.query_filtered::<&mut Inventory, With<Player>>();
        if let Ok(mut inventory) = query.get_single_mut(world) {
            inventory.remove(id, 1);
            info!("✅ Used {}", id);
        }
    }
}

// ============================================================
// Helpers
// ============================================================

fn get_drag_source(world: &mut World) -> Option<DropSource> {
    let drag_state = world.resource::<DragState>();
    let dragging_entity = drag_state.dragging?;

    if let Some(inv_slot) = world.get::<InventorySlotUI>(dragging_entity) {
        return Some(DropSource::Inventory(inv_slot.index));
    }

    if let Some(equip_slot) = world.get::<EquipmentSlotUI>(dragging_entity) {
        return Some(DropSource::Equipment(equip_slot.slot));
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

    let valid_slot = {
        let registry = world.resource::<ItemRegistry>();
        registry.get(id).equipment_slot() == Some(equip_slot)
    };

    if !valid_slot {
        info!("❌ {} cannot be equipped in {:?}", id, equip_slot);
        return;
    }

    let mut query = world.query_filtered::<(&mut Inventory, &mut Equipment), With<Player>>();
    if let Ok((mut inventory, mut equipment)) = query.get_single_mut(world) {
        if let Some(stack) = inventory.remove_slot(inv_slot) {
            if let Some(old_id) = equipment.unequip(equip_slot) {
                inventory.add_single(old_id);
            }
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

        if let Some(id) = equipment.unequip(equip_slot) {
            // Put directly in target slot
            inventory.set_slot(inv_slot, Some(ItemStack::new(id)));
            info!(
                "✅ Unequipped {} from {:?} to slot {}",
                id, equip_slot, inv_slot
            );
        }
    }
}

fn apply_consumable_effect(world: &mut World, id: ItemId) {
    use crate::items::ItemCategory;
    use crate::items::definition::ConsumableEffect;
    use crate::stats::health::component::Health;
    use crate::stats::mana::component::Mana;
    use crate::stats::stamina::component::Stamina;

    let effect = {
        let registry = world.resource::<ItemRegistry>();
        match &registry.get(id).category {
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
