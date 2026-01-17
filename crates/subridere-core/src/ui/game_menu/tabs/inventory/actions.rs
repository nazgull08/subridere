use bevy::prelude::*;
use bevy_ui_actions::prelude::*;

use crate::inventory::component::{Equipment, Inventory};
use crate::inventory::systems::{DropSource, DropToWorldEvent};
use crate::items::{ConsumableEffect, EquipmentSlot, ItemCategory, ItemRegistry, ItemStack};
use crate::player::component::Player;
use crate::stats::{Health, Mana, Stamina};

use super::components::{SelectedSlot, SlotId, SlotUI};

// ============================================================
// Drop to Inventory Slot
// ============================================================

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

        // Select the target slot after drop
        set_selection(world, SlotId::Inventory(self.target_index));
    }
}

// ============================================================
// Drop to Equipment Slot
// ============================================================

pub struct DropToEquipmentSlot {
    pub target_slot: EquipmentSlot,
}

impl UiAction for DropToEquipmentSlot {
    fn execute(&self, world: &mut World) {
        let Some(source) = get_drag_source(world) else {
            return;
        };

        let success = match source {
            DropSource::Inventory(source_index) => {
                equip_from_inventory(world, source_index, self.target_slot)
            }
            DropSource::Equipment(source_slot) => {
                if source_slot == self.target_slot {
                    return;
                }
                info!("Equipment to equipment swap not implemented");
                false
            }
        };

        // Select the target slot only on success
        if success {
            set_selection(world, SlotId::Equipment(self.target_slot));
        }
    }
}

// ============================================================
// Drop to World
// ============================================================

pub struct DropToWorldAction;

impl UiAction for DropToWorldAction {
    fn execute(&self, world: &mut World) {
        let Some(source) = get_drag_source(world) else {
            return;
        };

        let has_item = match source {
            DropSource::Inventory(slot) => {
                let mut query = world.query_filtered::<&Inventory, With<Player>>();
                query
                    .single(world)
                    .map(|inv| inv.get(slot).is_some())
                    .unwrap_or(false)
            }
            DropSource::Equipment(slot) => {
                let mut query = world.query_filtered::<&Equipment, With<Player>>();
                query
                    .single(world)
                    .map(|eq| eq.get(slot).is_some())
                    .unwrap_or(false)
            }
        };

        if !has_item {
            return;
        }

        // Clear selection when dropping to world
        clear_selection(world);

        world.send_event(DropToWorldEvent { source });
        info!("📤 Queued drop to world: {:?}", source);
    }
}

// ============================================================
// Use Consumable (Right Click)
// ============================================================

pub struct UseConsumableAction {
    pub slot_index: usize,
}

impl UiAction for UseConsumableAction {
    fn execute(&self, world: &mut World) {
        use_consumable(world, self.slot_index);
    }
}

// ============================================================
// Selection Actions
// ============================================================

pub struct SelectSlotAction {
    pub id: SlotId,
}

impl UiAction for SelectSlotAction {
    fn execute(&self, world: &mut World) {
        let current = world.resource::<SelectedSlot>().0;

        // Click on same slot — deselect
        if current == Some(self.id) {
            clear_selection(world);
            return;
        }

        // Select new slot
        set_selection(world, self.id);
    }
}

pub struct ClearSelectionAction;

impl UiAction for ClearSelectionAction {
    fn execute(&self, world: &mut World) {
        clear_selection(world);
    }
}

// ============================================================
// Selection Helpers
// ============================================================

fn set_selection(world: &mut World, id: SlotId) {
    // Remove old selection
    let current = world.resource::<SelectedSlot>().0;
    if let Some(old_id) = current {
        if let Some(entity) = find_slot_entity(world, old_id) {
            world.entity_mut(entity).remove::<Selected>();
        }
    }

    // Add new selection
    if let Some(entity) = find_slot_entity(world, id) {
        world.entity_mut(entity).insert(Selected);
    }
    world.resource_mut::<SelectedSlot>().0 = Some(id);
}

fn clear_selection(world: &mut World) {
    let current = world.resource::<SelectedSlot>().0;
    if let Some(id) = current {
        if let Some(entity) = find_slot_entity(world, id) {
            world.entity_mut(entity).remove::<Selected>();
        }
    }
    world.resource_mut::<SelectedSlot>().clear();
}

fn find_slot_entity(world: &mut World, id: SlotId) -> Option<Entity> {
    let mut query = world.query::<(Entity, &SlotUI)>();
    query
        .iter(world)
        .find(|(_, slot)| slot.id == id)
        .map(|(e, _)| e)
}

// ============================================================
// Drag Helpers
// ============================================================

fn get_drag_source(world: &mut World) -> Option<DropSource> {
    let drag_state = world.resource::<DragState>();
    let dragging_entity = drag_state.dragging?;

    if let Some(slot_ui) = world.get::<SlotUI>(dragging_entity) {
        return match slot_ui.id {
            SlotId::Inventory(index) => Some(DropSource::Inventory(index)),
            SlotId::Equipment(slot) => Some(DropSource::Equipment(slot)),
        };
    }

    None
}

fn swap_inventory_slots(world: &mut World, a: usize, b: usize) {
    let mut query = world.query_filtered::<&mut Inventory, With<Player>>();
    if let Ok(mut inventory) = query.single_mut(world) {
        inventory.swap(a, b);
        info!("🔄 Swapped slots {} ↔ {}", a, b);
    }
}

fn unequip_to_slot(world: &mut World, equip_slot: EquipmentSlot, inv_slot: usize) {
    let item_id = {
        let mut query = world.query_filtered::<&Equipment, With<Player>>();
        let Ok(equipment) = query.single(world) else {
            return;
        };
        equipment.get(equip_slot)
    };

    let Some(id) = item_id else {
        return;
    };

    {
        let mut query = world.query_filtered::<&mut Equipment, With<Player>>();
        if let Ok(mut equipment) = query.single_mut(world) {
            equipment.unequip(equip_slot);
        }
    }

    {
        let mut query = world.query_filtered::<&mut Inventory, With<Player>>();
        if let Ok(mut inventory) = query.single_mut(world) {
            inventory.set_slot(inv_slot, Some(ItemStack::new(id)));
        }
    }

    info!("📤 Unequipped {:?} to slot {}", equip_slot, inv_slot);
}

fn equip_from_inventory(world: &mut World, inv_slot: usize, equip_slot: EquipmentSlot) -> bool {
    let item_id = {
        let mut query = world.query_filtered::<&Inventory, With<Player>>();
        let Ok(inventory) = query.single(world) else {
            return false;
        };
        inventory.get(inv_slot).map(|stack| stack.id)
    };

    let Some(id) = item_id else {
        return false;
    };

    let valid_slot = {
        let registry = world.resource::<ItemRegistry>();
        registry.get(id).equipment_slot() == Some(equip_slot)
    };

    if !valid_slot {
        info!("❌ Cannot equip in {:?}", equip_slot);
        return false;
    }

    // Remove from inventory slot
    {
        let mut query = world.query_filtered::<&mut Inventory, With<Player>>();
        if let Ok(mut inventory) = query.single_mut(world) {
            inventory.remove_slot(inv_slot);
        }
    }

    // Equip and get previously equipped item
    let previously_equipped = {
        let mut query = world.query_filtered::<&mut Equipment, With<Player>>();
        if let Ok(mut equipment) = query.single_mut(world) {
            equipment.equip(equip_slot, id)
        } else {
            None
        }
    };

    // Put previously equipped item back into the inventory slot
    if let Some(old_id) = previously_equipped {
        let mut query = world.query_filtered::<&mut Inventory, With<Player>>();
        if let Ok(mut inventory) = query.single_mut(world) {
            inventory.set_slot(inv_slot, Some(ItemStack::new(old_id)));
        }
        info!(
            "🔄 Equipped {} to {:?}, swapped with {}",
            id, equip_slot, old_id
        );
    } else {
        info!("✅ Equipped {} to {:?}", id, equip_slot);
    }

    true
}

fn use_consumable(world: &mut World, slot_index: usize) {
    let item_id = {
        let mut query = world.query_filtered::<&Inventory, With<Player>>();
        let Ok(inventory) = query.single(world) else {
            return;
        };
        inventory.get(slot_index).map(|stack| stack.id)
    };

    let Some(id) = item_id else {
        return;
    };

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

    {
        let mut query =
            world.query_filtered::<(&mut Health, &mut Mana, &mut Stamina), With<Player>>();
        if let Ok((mut health, mut mana, mut stamina)) = query.single_mut(world) {
            match effect {
                ConsumableEffect::Heal(amount) => {
                    health.heal(amount);
                    info!("❤️ Healed for {:.0}", amount);
                }
                ConsumableEffect::RestoreMana(amount) => {
                    mana.restore(amount);
                    info!("💙 Restored {:.0} mana", amount);
                }
                ConsumableEffect::RestoreStamina(amount) => {
                    stamina.restore(amount);
                    info!("💚 Restored {:.0} stamina", amount);
                }
            }
        }
    }

    {
        let mut query = world.query_filtered::<&mut Inventory, With<Player>>();
        if let Ok(mut inventory) = query.single_mut(world) {
            if let Some(stack) = inventory.get_mut(slot_index) {
                if stack.quantity > 1 {
                    stack.quantity -= 1;
                    info!("📦 {} remaining", stack.quantity);
                } else {
                    inventory.remove_slot(slot_index);
                    info!("📦 Item consumed");
                }
            }
        }
    }
}
