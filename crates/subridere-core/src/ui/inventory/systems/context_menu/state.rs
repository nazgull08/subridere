use bevy::prelude::*;

use crate::ui::inventory::systems::EquipmentSlotType;

/// Marker component for context menu root
#[derive(Component)]
pub struct ContextMenu;

/// Resource to track context menu state
#[derive(Resource, Default)]
pub struct ContextMenuState {
    /// Is menu currently open
    pub is_open: bool,

    /// Which inventory slot was right-clicked (if any)
    pub inventory_slot: Option<usize>,

    /// Which equipment slot was right-clicked (if any)
    pub equipment_slot: Option<EquipmentSlotType>,

    /// Mouse position where menu was opened (fixed position)
    pub spawn_position: Vec2,
}

impl ContextMenuState {
    /// Close menu and clear state
    pub fn close(&mut self) {
        self.is_open = false;
        self.inventory_slot = None;
        self.equipment_slot = None;
        self.spawn_position = Vec2::ZERO;
    }
}
