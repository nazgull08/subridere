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
