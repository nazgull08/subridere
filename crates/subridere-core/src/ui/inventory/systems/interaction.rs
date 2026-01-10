use super::ui::InventorySlotUI;
use crate::ui::inventory::{
    layout::SLOT_BORDER_COLOR,
    systems::{ContextMenuState, EquipmentSlotType, EquipmentSlotUI},
};
use bevy::prelude::*;

/// Colors for slot states
const BORDER_NORMAL: Color = SLOT_BORDER_COLOR;
const BORDER_HOVERED: Color = Color::srgb(0.7, 0.7, 0.7); // Brighter
const BORDER_SELECTED: Color = Color::srgb(1.0, 0.8, 0.3); // Golden

const EQUIP_BORDER_NORMAL: Color = Color::srgb(0.35, 0.35, 0.35);
const EQUIP_BORDER_HOVERED: Color = Color::srgb(0.6, 0.6, 0.6);

/// Resource to track selected inventory slot
#[derive(Resource, Default)]
pub struct SelectedSlot {
    /// Selected inventory slot index
    pub inventory_slot: Option<usize>,

    /// Selected equipment slot type
    pub equipment_slot: Option<EquipmentSlotType>,
}

impl SelectedSlot {
    /// Clear all selections
    pub fn clear(&mut self) {
        self.inventory_slot = None;
        self.equipment_slot = None;
    }

    /// Check if anything is selected
    pub fn has_selection(&self) -> bool {
        self.inventory_slot.is_some() || self.equipment_slot.is_some()
    }

    /// Check if inventory slot is selected
    pub fn is_inventory_selected(&self, slot: usize) -> bool {
        self.inventory_slot == Some(slot)
    }

    /// Check if equipment slot is selected
    pub fn is_equipment_selected(&self, slot: EquipmentSlotType) -> bool {
        self.equipment_slot == Some(slot)
    }
}

/// Handle hover effect on inventory slots
pub fn handle_slot_hover(
    mut slot_query: Query<
        // ИСПРАВИТЬ: добавить <> вместо переноса строки
        (&Interaction, &mut BorderColor, &InventorySlotUI),
        Changed<Interaction>,
    >,
    selected_slot: Res<SelectedSlot>,
) {
    for (interaction, mut border_color, slot_ui) in &mut slot_query {
        // Check if this slot is selected
        let is_selected = selected_slot.is_inventory_selected(slot_ui.slot_index);

        match *interaction {
            Interaction::Hovered => {
                if !is_selected {
                    *border_color = BorderColor(BORDER_HOVERED);
                }
            }
            Interaction::None => {
                if !is_selected {
                    *border_color = BorderColor(BORDER_NORMAL);
                }
            }
            Interaction::Pressed => {
                // Handled by click system
            }
        }
    }
}

/// Handle click on inventory slots
pub fn handle_slot_click(
    mut slot_query: Query<(&Interaction, &InventorySlotUI), Changed<Interaction>>,
    mut selected_slot: ResMut<SelectedSlot>,
    inventory_query: Query<&crate::inventory::Inventory, With<crate::player::component::Player>>,
) {
    // Get player inventory
    let Ok(inventory) = inventory_query.single() else {
        return;
    };

    for (interaction, slot_ui) in &mut slot_query {
        if *interaction == Interaction::Pressed {
            // If clicking the same slot, deselect
            if selected_slot.is_inventory_selected(slot_ui.slot_index) {
                info!("📦 Deselected inventory slot {}", slot_ui.slot_index);
                selected_slot.clear();
            }
            // If NOTHING is selected AND slot has an item, select it
            else if !selected_slot.has_selection() {
                // Check if slot actually has an item
                if inventory
                    .slots
                    .get(slot_ui.slot_index)
                    .and_then(|s| s.as_ref())
                    .is_some()
                {
                    info!("📦 Selected inventory slot {}", slot_ui.slot_index);
                    selected_slot.inventory_slot = Some(slot_ui.slot_index);
                } else {
                    // Slot is empty, don't select
                    info!("❌ Cannot select empty slot {}", slot_ui.slot_index);
                }
            }
            // If something ELSE is selected, do nothing
            // (let process_item_actions handle the move/swap)
        }
    }
}

/// Update visual indication of selected slots (both inventory and equipment)
pub fn update_selected_slot_visual(
    mut inventory_query: Query<(&InventorySlotUI, &mut BorderColor), Without<EquipmentSlotUI>>,
    mut equipment_query: Query<(&EquipmentSlotUI, &mut BorderColor), Without<InventorySlotUI>>,
    selected_slot: Res<SelectedSlot>,
) {
    // Only run if selection changed
    if !selected_slot.is_changed() {
        return;
    }

    // Update inventory slot borders
    for (slot_ui, mut border_color) in &mut inventory_query {
        if selected_slot.is_inventory_selected(slot_ui.slot_index) {
            *border_color = BorderColor(BORDER_SELECTED);
        } else {
            *border_color = BorderColor(BORDER_NORMAL);
        }
    }

    // Update equipment slot borders
    for (slot_ui, mut border_color) in &mut equipment_query {
        if selected_slot.is_equipment_selected(slot_ui.slot_type) {
            *border_color = BorderColor(BORDER_SELECTED);
        } else {
            *border_color = BorderColor(EQUIP_BORDER_NORMAL);
        }
    }
}

/// Handle hover effect on equipment slots
pub fn handle_equip_slot_hover(
    mut slot_query: Query<(&Interaction, &mut BorderColor, &EquipmentSlotUI), Changed<Interaction>>,
    selected_slot: Res<SelectedSlot>, // ← ADD THIS
) {
    for (interaction, mut border_color, slot_ui) in &mut slot_query {
        // Check if this slot is selected
        let is_selected = selected_slot.is_equipment_selected(slot_ui.slot_type);

        match *interaction {
            Interaction::Hovered => {
                // Don't change color if slot is selected
                if !is_selected {
                    *border_color = BorderColor(EQUIP_BORDER_HOVERED);
                }
            }
            Interaction::None => {
                // Don't change color if slot is selected
                if !is_selected {
                    *border_color = BorderColor(EQUIP_BORDER_NORMAL);
                }
            }
            Interaction::Pressed => {
                // Handled by click system
            }
        }
    }
}

/// Handle click on equipment slots
pub fn handle_equip_slot_click(
    slot_query: Query<(&Interaction, &EquipmentSlotUI), Changed<Interaction>>,
    mut selected_slot: ResMut<SelectedSlot>,
    equipment_query: Query<&crate::inventory::Equipment, With<crate::player::component::Player>>,
) {
    // Get player equipment
    let Ok(equipment) = equipment_query.single() else {
        return;
    };

    for (interaction, slot_ui) in &slot_query {
        if *interaction == Interaction::Pressed {
            // If clicking the same slot, deselect
            if selected_slot.is_equipment_selected(slot_ui.slot_type) {
                info!("📦 Deselected equipment slot {:?}", slot_ui.slot_type);
                selected_slot.clear();
            }
            // If NOTHING is selected AND slot has an item, select it
            else if !selected_slot.has_selection() {
                // Check if slot actually has an item equipped
                if equipment.get_slot(slot_ui.slot_type).is_some() {
                    info!("📦 Selected equipment slot {:?}", slot_ui.slot_type);
                    selected_slot.equipment_slot = Some(slot_ui.slot_type);
                } else {
                    // Slot is empty, don't select
                    info!(
                        "❌ Cannot select empty equipment slot {:?}",
                        slot_ui.slot_type
                    );
                }
            }
            // If something ELSE is selected, do nothing
            // (let process_item_actions handle the equip/unequip)
        }
    }
}

/// Detect right-click on inventory slots to open context menu
pub fn detect_inventory_right_click(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    slot_query: Query<(&Interaction, &InventorySlotUI)>,
    inventory_query: Query<&crate::inventory::Inventory, With<crate::player::component::Player>>,
    mut menu_state: ResMut<super::context_menu::ContextMenuState>,
    windows: Query<&Window>,
) {
    // Only process right-click
    if !mouse_buttons.just_pressed(MouseButton::Right) {
        return;
    }

    // Get player inventory
    let Ok(inventory) = inventory_query.single() else {
        return;
    };

    // Check which slot was right-clicked
    for (interaction, slot_ui) in &slot_query {
        if *interaction == Interaction::Hovered {
            // Check if slot has an item
            if inventory
                .slots
                .get(slot_ui.slot_index)
                .and_then(|s| s.as_ref())
                .is_some()
            {
                // Close existing menu if any
                if menu_state.is_open {
                    menu_state.close();
                    // Despawn will happen next frame
                }

                let cursor_pos = windows
                    .single()
                    .ok()
                    .and_then(|w| w.cursor_position())
                    .unwrap_or(Vec2::new(400.0, 300.0));

                // Open new menu for this slot
                menu_state.is_open = true;
                menu_state.inventory_slot = Some(slot_ui.slot_index);
                menu_state.equipment_slot = None;
                menu_state.spawn_position = cursor_pos;

                info!("📋 Right-clicked inventory slot {}", slot_ui.slot_index);
                return;
            }
        }
    }
}

/// Detect right-click on equipment slots to open context menu
pub fn detect_equipment_right_click(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    slot_query: Query<(&Interaction, &EquipmentSlotUI)>,
    equipment_query: Query<&crate::inventory::Equipment, With<crate::player::component::Player>>,
    mut menu_state: ResMut<super::context_menu::ContextMenuState>,
    windows: Query<&Window>,
) {
    // Only process right-click
    if !mouse_buttons.just_pressed(MouseButton::Right) {
        return;
    }

    // Get player equipment
    let Ok(equipment) = equipment_query.single() else {
        return;
    };

    // Check which slot was right-clicked
    for (interaction, slot_ui) in &slot_query {
        if *interaction == Interaction::Hovered {
            // Check if slot has an item equipped
            if equipment.get_slot(slot_ui.slot_type).is_some() {
                // Close existing menu if any
                if menu_state.is_open {
                    menu_state.close();
                }

                // Get current cursor position
                let cursor_pos = windows
                    .single()
                    .ok()
                    .and_then(|w| w.cursor_position())
                    .unwrap_or(Vec2::new(400.0, 300.0));

                // Open new menu for this slot
                menu_state.is_open = true;
                menu_state.inventory_slot = None;
                menu_state.equipment_slot = Some(slot_ui.slot_type);
                menu_state.spawn_position = cursor_pos;

                info!("📋 Right-clicked equipment slot {:?}", slot_ui.slot_type);
                return;
            }
        }
    }
}

/// Close context menu on clicks outside menu buttons
pub fn close_menu_on_outside_click(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    // Check if clicked on menu buttons
    menu_button_query: Query<
        &Interaction,
        Or<(
            With<super::context_menu::EquipButton>,
            With<super::context_menu::DropButton>,
            With<super::context_menu::CancelButton>,
        )>,
    >,
    // Check if clicked on inventory/equipment slots (valid targets)
    mut menu_state: ResMut<super::context_menu::ContextMenuState>,
) {
    // If menu is not open, nothing to do
    if !menu_state.is_open {
        return;
    }

    // Check for any mouse click (left or right)
    let clicked = mouse_buttons.just_pressed(MouseButton::Left)
        || mouse_buttons.just_pressed(MouseButton::Right);

    if !clicked {
        return;
    }

    // Check if clicked on a menu button
    let clicked_on_menu_button = menu_button_query
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed);

    if clicked_on_menu_button {
        // Let the button handler deal with this
        return;
    }

    // Any other click (on slots, on empty space, etc.) closes menu
    info!("📋 Closing menu (clicked outside)");
    menu_state.close();
}

/// Force close menu when inventory is closing
pub fn force_close_menu_on_inventory_exit(mut menu_state: ResMut<ContextMenuState>) {
    if menu_state.is_open {
        info!("📋 Force closing menu (inventory closing)");
        menu_state.close();
    }
}
