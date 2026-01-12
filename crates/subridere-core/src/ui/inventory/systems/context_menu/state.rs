use bevy::prelude::*;

/// Marker component for context menu root
#[derive(Component)]
pub struct ContextMenu;

/// Marker components for menu buttons
#[derive(Component)]
pub struct EquipButton;

#[derive(Component)]
pub struct DropButton;

#[derive(Component)]
pub struct CancelButton;

/// Resource to track context menu state
#[derive(Resource, Default)]
pub struct ContextMenuState {
    /// Is menu currently open
    pub is_open: bool,

    /// Which inventory slot was right-clicked (if any)
    pub inventory_slot: Option<usize>,

    /// Which equipment slot was right-clicked (if any)
    pub equipment_slot: Option<super::super::EquipmentSlotType>,

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
