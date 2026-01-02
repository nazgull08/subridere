// ui/inventory/data.rs

use bevy::prelude::*;

/// Player inventory component
#[derive(Component, Clone)]
pub struct Inventory {
    /// Inventory slots (None = empty slot)
    pub slots: Vec<Option<InventorySlot>>,

    /// Maximum number of slots
    pub max_slots: usize,
}

/// Single item stack in inventory
#[derive(Clone, Debug)]
pub struct InventorySlot {
    /// Item identifier (e.g. "wooden_staff")
    pub item_id: String,

    /// Quantity of items in this stack
    pub quantity: u32,
}

impl Default for Inventory {
    fn default() -> Self {
        let mut slots = vec![None; 20];

        // Add test item in slot 0
        slots[0] = Some(InventorySlot {
            item_id: "wooden_staff".to_string(),
            quantity: 1,
        });

        Self {
            slots,
            max_slots: 20,
        }
    }
}

impl Inventory {
    /// Create empty inventory with specified number of slots
    pub fn new(max_slots: usize) -> Self {
        Self {
            slots: vec![None; max_slots],
            max_slots,
        }
    }

    /// Add item to first available slot
    pub fn add_item(&mut self, item_id: String, quantity: u32) -> bool {
        for slot in &mut self.slots {
            if slot.is_none() {
                *slot = Some(InventorySlot { item_id, quantity });
                return true;
            }
        }
        false // Inventory full
    }

    /// Remove item from slot
    pub fn remove_item(&mut self, slot_index: usize) -> Option<InventorySlot> {
        if slot_index < self.slots.len() {
            self.slots[slot_index].take()
        } else {
            None
        }
    }
}
