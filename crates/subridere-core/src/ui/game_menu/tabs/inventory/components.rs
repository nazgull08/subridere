use bevy::prelude::*;

use crate::items::EquipmentSlot;

/// Inventory grid slot
#[derive(Component)]
pub struct InventorySlotUI {
    pub index: usize,
}

/// Equipment slot
#[derive(Component)]
pub struct EquipmentSlotUI {
    pub slot: EquipmentSlot,
}

/// Icon image inside a slot
#[derive(Component)]
pub struct SlotIcon;

/// Quantity text inside a slot
#[derive(Component)]
pub struct SlotQuantity;

/// Label text for equipment slot
#[derive(Component)]
pub struct SlotLabel;

/// Which slot is currently selected for inspection
#[derive(Resource, Default)]
pub struct SelectedSlot {
    pub inventory: Option<usize>,
    pub equipment: Option<EquipmentSlot>,
}

impl SelectedSlot {
    pub fn clear(&mut self) {
        self.inventory = None;
        self.equipment = None;
    }

    pub fn is_empty(&self) -> bool {
        self.inventory.is_none() && self.equipment.is_none()
    }
}
