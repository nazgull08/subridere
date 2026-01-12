// inventory/component.rs
use bevy::prelude::*;

/// Player inventory - storage for items
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
        let slots = vec![None; 20];

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

/// Equipment worn by character
#[derive(Component, Default, Clone, Debug)]
pub struct Equipment {
    // Head
    pub helmet: Option<String>,

    // Shoulders
    pub left_pauldron: Option<String>,
    pub right_pauldron: Option<String>,

    // Body
    pub chest: Option<String>,

    // Hands
    pub left_glove: Option<String>,
    pub right_glove: Option<String>,

    // Legs
    pub greaves: Option<String>,
    pub left_boot: Option<String>,
    pub right_boot: Option<String>,

    // Weapons
    pub main_hand: Option<String>,
    pub off_hand: Option<String>,
}

impl Equipment {
    /// Get mutable reference to equipment slot by type
    pub fn get_slot_mut(
        &mut self,
        slot_type: crate::ui::inventory::systems::EquipmentSlotType,
    ) -> &mut Option<String> {
        use crate::ui::inventory::systems::EquipmentSlotType;

        match slot_type {
            EquipmentSlotType::Helmet => &mut self.helmet,
            EquipmentSlotType::LeftPauldron => &mut self.left_pauldron,
            EquipmentSlotType::RightPauldron => &mut self.right_pauldron,
            EquipmentSlotType::Chest => &mut self.chest,
            EquipmentSlotType::LeftGlove => &mut self.left_glove,
            EquipmentSlotType::RightGlove => &mut self.right_glove,
            EquipmentSlotType::Greaves => &mut self.greaves,
            EquipmentSlotType::LeftBoot => &mut self.left_boot,
            EquipmentSlotType::RightBoot => &mut self.right_boot,
            EquipmentSlotType::MainHand => &mut self.main_hand,
            EquipmentSlotType::OffHand => &mut self.off_hand,
        }
    }

    /// Get immutable reference to equipment slot by type
    pub fn get_slot(
        &self,
        slot_type: crate::ui::inventory::systems::EquipmentSlotType,
    ) -> &Option<String> {
        use crate::ui::inventory::systems::EquipmentSlotType;

        match slot_type {
            EquipmentSlotType::Helmet => &self.helmet,
            EquipmentSlotType::LeftPauldron => &self.left_pauldron,
            EquipmentSlotType::RightPauldron => &self.right_pauldron,
            EquipmentSlotType::Chest => &self.chest,
            EquipmentSlotType::LeftGlove => &self.left_glove,
            EquipmentSlotType::RightGlove => &self.right_glove,
            EquipmentSlotType::Greaves => &self.greaves,
            EquipmentSlotType::LeftBoot => &self.left_boot,
            EquipmentSlotType::RightBoot => &self.right_boot,
            EquipmentSlotType::MainHand => &self.main_hand,
            EquipmentSlotType::OffHand => &self.off_hand,
        }
    }
}
