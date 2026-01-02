use bevy::prelude::*;

/// Marker component for inventory UI root
#[derive(Component)]
pub struct InventoryUI;

/// Marker component for individual inventory slot UI
#[derive(Component)]
pub struct InventorySlotUI {
    pub slot_index: usize,
}

/// Marker for the icon image inside a slot
#[derive(Component)]
pub struct SlotIcon;

/// Marker for the quantity text inside a slot
#[derive(Component)]
pub struct SlotQuantity;

/// Marker for HP text in stats panel
#[derive(Component)]
pub struct StatsHpText;

/// Marker for MP text in stats panel
#[derive(Component)]
pub struct StatsMpText;

/// Marker for SP text in stats panel
#[derive(Component)]
pub struct StatsSpText;

/// Marker for equipment slot UI elements
#[derive(Component)]
pub struct EquipmentSlotUI {
    pub slot_type: EquipmentSlotType,
}

/// Marker for the icon image inside an equipment slot
#[derive(Component)]
pub struct EquipSlotIcon;

/// Types of equipment slots
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EquipmentSlotType {
    Helmet,
    LeftPauldron,
    RightPauldron,
    Chest,
    LeftGlove,
    RightGlove,
    Greaves,
    LeftBoot,
    RightBoot,
    MainHand,
    OffHand,
}

impl EquipmentSlotType {
    /// Get display name for the slot
    pub fn name(&self) -> &str {
        match self {
            Self::Helmet => "Helmet",
            Self::LeftPauldron => "L.Pauldron",
            Self::RightPauldron => "R.Pauldron",
            Self::Chest => "Chest",
            Self::LeftGlove => "L.Glove",
            Self::RightGlove => "R.Glove",
            Self::Greaves => "Greaves",
            Self::LeftBoot => "L.Boot",
            Self::RightBoot => "R.Boot",
            Self::MainHand => "Main Hand",
            Self::OffHand => "Off Hand",
        }
    }
}
