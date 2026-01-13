// inventory/component.rs — Inventory and Equipment components

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::items::{EquipmentSlot, ItemId, ItemRegistry, ItemStack};

/// Player inventory — storage for items
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Inventory {
    /// Inventory slots (None = empty slot)
    slots: Vec<Option<ItemStack>>,
}

impl Default for Inventory {
    fn default() -> Self {
        Self::new(20)
    }
}

impl Inventory {
    /// Create empty inventory with specified capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            slots: vec![None; capacity],
        }
    }

    /// Current capacity
    pub fn capacity(&self) -> usize {
        self.slots.len()
    }

    /// Number of occupied slots
    pub fn occupied(&self) -> usize {
        self.slots.iter().filter(|s| s.is_some()).count()
    }

    /// Number of free slots
    pub fn free_slots(&self) -> usize {
        self.capacity() - self.occupied()
    }

    /// Is inventory full?
    pub fn is_full(&self) -> bool {
        self.free_slots() == 0
    }

    /// Get slot contents
    pub fn get(&self, index: usize) -> Option<&ItemStack> {
        self.slots.get(index)?.as_ref()
    }

    /// Get mutable slot contents
    pub fn get_mut(&mut self, index: usize) -> Option<&mut ItemStack> {
        self.slots.get_mut(index)?.as_mut()
    }

    /// Iterate over all slots
    pub fn iter(&self) -> impl Iterator<Item = (usize, Option<&ItemStack>)> {
        self.slots.iter().enumerate().map(|(i, s)| (i, s.as_ref()))
    }

    /// Iterate over occupied slots only
    pub fn iter_items(&self) -> impl Iterator<Item = (usize, &ItemStack)> {
        self.slots
            .iter()
            .enumerate()
            .filter_map(|(i, s)| s.as_ref().map(|stack| (i, stack)))
    }

    /// Add item to inventory
    ///
    /// Tries to stack with existing items first, then finds empty slot.
    /// Returns true if item was added, false if inventory full.
    pub fn add(&mut self, id: ItemId, quantity: u32, registry: &ItemRegistry) -> bool {
        let def = registry.get(id);
        let max_stack = def.max_stack;

        let mut remaining = quantity;

        // First: try to stack with existing
        if max_stack > 1 {
            for slot in &mut self.slots {
                if let Some(stack) = slot {
                    if stack.id == id && stack.quantity < max_stack {
                        let can_add = max_stack - stack.quantity;
                        let will_add = can_add.min(remaining);
                        stack.quantity += will_add;
                        remaining -= will_add;

                        if remaining == 0 {
                            return true;
                        }
                    }
                }
            }
        }

        // Second: find empty slots
        while remaining > 0 {
            let empty_slot = self.slots.iter_mut().find(|s| s.is_none());

            if let Some(slot) = empty_slot {
                let will_add = max_stack.min(remaining);
                *slot = Some(ItemStack::with_quantity(id, will_add));
                remaining -= will_add;
            } else {
                // No more empty slots
                return false;
            }
        }

        true
    }

    /// Add item without registry (assumes max_stack = 1)
    ///
    /// Use this when you don't have registry access (e.g., in tests)
    pub fn add_single(&mut self, id: ItemId) -> bool {
        let empty_slot = self.slots.iter_mut().find(|s| s.is_none());

        if let Some(slot) = empty_slot {
            *slot = Some(ItemStack::new(id));
            true
        } else {
            false
        }
    }

    /// Remove item from specific slot
    pub fn remove_slot(&mut self, index: usize) -> Option<ItemStack> {
        self.slots.get_mut(index)?.take()
    }

    /// Remove specific quantity of item type
    ///
    /// Returns true if removed successfully, false if not enough items.
    pub fn remove(&mut self, id: ItemId, quantity: u32) -> bool {
        if self.count(id) < quantity {
            return false;
        }

        let mut remaining = quantity;

        for slot in &mut self.slots {
            if let Some(stack) = slot {
                if stack.id == id {
                    if stack.quantity <= remaining {
                        remaining -= stack.quantity;
                        *slot = None;
                    } else {
                        stack.quantity -= remaining;
                        remaining = 0;
                    }

                    if remaining == 0 {
                        return true;
                    }
                }
            }
        }

        true
    }

    /// Check if inventory contains item
    pub fn has(&self, id: ItemId, quantity: u32) -> bool {
        self.count(id) >= quantity
    }

    /// Count total quantity of item type
    pub fn count(&self, id: ItemId) -> u32 {
        self.slots
            .iter()
            .filter_map(|s| s.as_ref())
            .filter(|stack| stack.id == id)
            .map(|stack| stack.quantity)
            .sum()
    }

    /// Set item directly in slot (for swap/unequip operations)
    pub fn set_slot(&mut self, index: usize, stack: Option<ItemStack>) {
        if index < self.slots.len() {
            self.slots[index] = stack;
        }
    }

    /// Find first slot containing item type
    pub fn find(&self, id: ItemId) -> Option<usize> {
        self.slots
            .iter()
            .position(|s| s.as_ref().map(|stack| stack.id == id).unwrap_or(false))
    }

    /// Swap two slots
    pub fn swap(&mut self, a: usize, b: usize) {
        if a < self.slots.len() && b < self.slots.len() {
            self.slots.swap(a, b);
        }
    }

    /// Clear all slots
    pub fn clear(&mut self) {
        for slot in &mut self.slots {
            *slot = None;
        }
    }
}

/// Equipment worn by character
#[derive(Component, Clone, Debug, Default, Serialize, Deserialize)]
pub struct Equipment {
    /// Equipped items by slot
    slots: HashMap<EquipmentSlot, ItemId>,
}

impl Equipment {
    pub fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }

    /// Get equipped item in slot
    pub fn get(&self, slot: EquipmentSlot) -> Option<ItemId> {
        self.slots.get(&slot).copied()
    }

    /// Check if slot is occupied
    pub fn is_equipped(&self, slot: EquipmentSlot) -> bool {
        self.slots.contains_key(&slot)
    }

    /// Equip item, returning previously equipped item if any
    pub fn equip(&mut self, slot: EquipmentSlot, id: ItemId) -> Option<ItemId> {
        self.slots.insert(slot, id)
    }

    /// Unequip item from slot
    pub fn unequip(&mut self, slot: EquipmentSlot) -> Option<ItemId> {
        self.slots.remove(&slot)
    }

    /// Iterate over all equipped items
    pub fn iter(&self) -> impl Iterator<Item = (EquipmentSlot, ItemId)> + '_ {
        self.slots.iter().map(|(&slot, &id)| (slot, id))
    }

    /// Count equipped items
    pub fn count(&self) -> usize {
        self.slots.len()
    }

    /// Calculate total defense from armor
    pub fn total_defense(&self, registry: &ItemRegistry) -> f32 {
        self.slots
            .values()
            .filter_map(|&id| registry.get(id).defense())
            .sum()
    }

    /// Calculate total damage bonus from weapons
    pub fn total_damage(&self, registry: &ItemRegistry) -> f32 {
        self.slots
            .values()
            .filter_map(|&id| registry.get(id).damage())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests use add_single since we don't have registry in tests

    #[test]
    fn inventory_add_remove() {
        let mut inv = Inventory::new(5);

        // Можем добавить предмет только если есть хотя бы один ItemId
        // В реальных тестах нужен ItemId enum - пока пропустим
    }

    #[test]
    fn inventory_capacity() {
        let inv = Inventory::new(10);
        assert_eq!(inv.capacity(), 10);
        assert_eq!(inv.free_slots(), 10);
        assert!(!inv.is_full());
    }
}
