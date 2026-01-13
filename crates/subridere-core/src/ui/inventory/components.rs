// ui/inventory/components.rs — UI marker components

use crate::items::EquipmentSlot;
use bevy::prelude::*;

/// Root container for entire inventory UI
#[derive(Component)]
pub struct InventoryRoot;

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

/// Quantity text inside a slot (for stackable items)
#[derive(Component)]
pub struct SlotQuantity;

/// Label text for equipment slot (shown when empty)
#[derive(Component)]
pub struct SlotLabel;

/// Stats panel markers
#[derive(Component)]
pub struct StatsPanel;

#[derive(Component)]
pub struct StatsHpText;

#[derive(Component)]
pub struct StatsMpText;

#[derive(Component)]
pub struct StatsSpText;

/// Tooltip panel
#[derive(Component)]
pub struct TooltipPanel;

#[derive(Component)]
pub struct TooltipName;

#[derive(Component)]
pub struct TooltipDesc;
