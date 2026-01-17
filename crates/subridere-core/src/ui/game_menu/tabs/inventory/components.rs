use crate::items::EquipmentSlot;
use bevy::prelude::*;

/// Unified slot identifier
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum SlotId {
    Inventory(usize),
    Equipment(EquipmentSlot),
}

/// Unified slot UI component
#[derive(Component)]
pub struct SlotUI {
    pub id: SlotId,
}

impl SlotUI {
    pub fn inventory(index: usize) -> Self {
        Self {
            id: SlotId::Inventory(index),
        }
    }

    pub fn equipment(slot: EquipmentSlot) -> Self {
        Self {
            id: SlotId::Equipment(slot),
        }
    }
}

/// Currently selected slot
#[derive(Resource, Default)]
pub struct SelectedSlot(pub Option<SlotId>);

impl SelectedSlot {
    pub fn clear(&mut self) {
        self.0 = None;
    }

    pub fn is_selected(&self, id: SlotId) -> bool {
        self.0 == Some(id)
    }
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

/// Description panel root
#[derive(Component)]
pub struct DescriptionPanel;

/// Text inside description panel
#[derive(Component)]
pub struct DescriptionText;
