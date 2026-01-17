use bevy::prelude::*;
use bevy_ui_actions::prelude::*;

use crate::inventory::component::{Equipment, Inventory};
use crate::inventory::systems::{DropSource, DropToWorldEvent};
use crate::items::{ConsumableEffect, EquipmentSlot, ItemCategory, ItemRegistry, ItemStack};
use crate::player::component::Player;
use crate::stats::{Health, Mana, Stamina};
use crate::ui::game_menu::tabs::inventory::SelectedSlot;

use super::components::{EquipmentSlotUI, InventorySlotUI};

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

fn use_consumable(world: &mut World, slot_index: usize) {
    // 1. Получить item_id из слота
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

    // 2. Проверить, что это consumable, и получить эффект
    let effect = {
        let registry = world.resource::<ItemRegistry>();
        let def = registry.get(id);
        match &def.category {
            ItemCategory::Consumable(data) => Some(data.effect.clone()),
            _ => None,
        }
    };

    let Some(effect) = effect else {
        return; // Не consumable — молча игнорируем
    };

    // 3. Применить эффект
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

    // 4. Уменьшить stack или удалить предмет
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
    if let Ok(mut inventory) = query.single_mut(world) {
        inventory.swap(a, b);
        info!("🔄 Swapped slots {} ↔ {}", a, b);
    }
}

fn unequip_to_slot(world: &mut World, equip_slot: EquipmentSlot, inv_slot: usize) {
    let mut inv_query = world.query_filtered::<&mut Inventory, With<Player>>();
    let mut equip_query = world.query_filtered::<&mut Equipment, With<Player>>();

    let item_id = {
        let Ok(equipment) = equip_query.single(world) else {
            return;
        };
        equipment.get(equip_slot)
    };

    let Some(id) = item_id else {
        return;
    };

    if let Ok(mut equipment) = equip_query.single_mut(world) {
        equipment.unequip(equip_slot);
    }

    if let Ok(mut inventory) = inv_query.single_mut(world) {
        inventory.set_slot(inv_slot, Some(ItemStack::new(id)));
    }

    info!("📤 Unequipped {:?} to slot {}", equip_slot, inv_slot);
}

fn equip_from_inventory(world: &mut World, inv_slot: usize, equip_slot: EquipmentSlot) {
    let item_id = {
        let mut query = world.query_filtered::<&Inventory, With<Player>>();
        let Ok(inventory) = query.single(world) else {
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
        info!("❌ Cannot equip in {:?}", equip_slot);
        return;
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

    // Put previously equipped item back into the inventory slot we just emptied
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
}

pub struct SelectInventorySlotAction {
    pub index: usize,
}

impl UiAction for SelectInventorySlotAction {
    fn execute(&self, world: &mut World) {
        select_inventory_slot(world, self.index);
    }
}

fn select_inventory_slot(world: &mut World, index: usize) {
    // Найти entity слота
    let slot_entity = {
        let mut query = world.query::<(Entity, &InventorySlotUI)>();
        query
            .iter(world)
            .find(|(_, slot)| slot.index == index)
            .map(|(e, _)| e)
    };

    let Some(clicked) = slot_entity else { return };

    // Текущее выделение
    let current = world.resource::<SelectedSlot>().inventory;

    // Клик на тот же слот — снять выделение
    if current == Some(index) {
        world.entity_mut(clicked).remove::<Selected>();
        world.resource_mut::<SelectedSlot>().clear();
        return;
    }

    // Снять выделение с предыдущего inventory слота
    if let Some(prev_index) = current {
        let prev_entity = {
            let mut query = world.query::<(Entity, &InventorySlotUI)>();
            query
                .iter(world)
                .find(|(_, slot)| slot.index == prev_index)
                .map(|(e, _)| e)
        };
        if let Some(prev) = prev_entity {
            world.entity_mut(prev).remove::<Selected>();
        }
    }

    // Снять выделение с equipment слота если был
    clear_equipment_selection(world);

    // Выделить новый
    world.entity_mut(clicked).insert(Selected);
    let mut selected = world.resource_mut::<SelectedSlot>();
    selected.inventory = Some(index);
    selected.equipment = None;
}

fn clear_equipment_selection(world: &mut World) {
    let current_eq = world.resource::<SelectedSlot>().equipment;
    if let Some(eq_slot) = current_eq {
        let entity = {
            let mut query = world.query::<(Entity, &EquipmentSlotUI)>();
            query
                .iter(world)
                .find(|(_, slot)| slot.slot == eq_slot)
                .map(|(e, _)| e)
        };
        if let Some(e) = entity {
            world.entity_mut(e).remove::<Selected>();
        }
    }
}

pub struct ClearSelectionAction;

impl UiAction for ClearSelectionAction {
    fn execute(&self, world: &mut World) {
        clear_all_selection(world);
    }
}

fn clear_all_selection(world: &mut World) {
    // Снять с inventory
    if let Some(inv_index) = world.resource::<SelectedSlot>().inventory {
        let entity = {
            let mut query = world.query::<(Entity, &InventorySlotUI)>();
            query
                .iter(world)
                .find(|(_, slot)| slot.index == inv_index)
                .map(|(e, _)| e)
        };
        if let Some(e) = entity {
            world.entity_mut(e).remove::<Selected>();
        }
    }

    // Снять с equipment
    clear_equipment_selection(world);

    // Очистить ресурс
    world.resource_mut::<SelectedSlot>().clear();
}
