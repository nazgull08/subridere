use super::ui::InventorySlotUI;
use crate::ui::inventory::{layout::SLOT_BORDER_COLOR, systems::EquipmentSlotUI};
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
    pub slot_index: Option<usize>,
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
        let is_selected = selected_slot.slot_index == Some(slot_ui.slot_index);

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
    mut slot_query: Query<
        // ИСПРАВИТЬ: добавить <>
        (&Interaction, &InventorySlotUI),
        Changed<Interaction>,
    >,
    mut selected_slot: ResMut<SelectedSlot>,
) {
    for (interaction, slot_ui) in &mut slot_query {
        if *interaction == Interaction::Pressed {
            // Toggle selection
            if selected_slot.slot_index == Some(slot_ui.slot_index) {
                // Deselect if clicking same slot
                info!("📦 Deselected slot {}", slot_ui.slot_index);
                selected_slot.slot_index = None;
            } else {
                // Select new slot
                info!("📦 Selected slot {}", slot_ui.slot_index);
                selected_slot.slot_index = Some(slot_ui.slot_index);
            }
        }
    }
}

/// Update visual indication of selected slot
pub fn update_selected_slot_visual(
    mut slot_query: Query<(&InventorySlotUI, &mut BorderColor)>,
    selected_slot: Res<SelectedSlot>,
) {
    // Only run if selection changed
    if !selected_slot.is_changed() {
        return;
    }

    for (slot_ui, mut border_color) in &mut slot_query {
        if selected_slot.slot_index == Some(slot_ui.slot_index) {
            // This slot is selected - golden border
            *border_color = BorderColor(BORDER_SELECTED);
        } else {
            // Not selected - normal border
            *border_color = BorderColor(BORDER_NORMAL);
        }
    }
}

/// Handle hover effect on equipment slots
pub fn handle_equip_slot_hover(
    mut slot_query: Query<(&Interaction, &mut BorderColor, &EquipmentSlotUI), Changed<Interaction>>,
) {
    for (interaction, mut border_color, _slot_ui) in &mut slot_query {
        match *interaction {
            Interaction::Hovered => {
                *border_color = BorderColor(EQUIP_BORDER_HOVERED);
            }
            Interaction::None => {
                *border_color = BorderColor(EQUIP_BORDER_NORMAL);
            }
            Interaction::Pressed => {
                // Handled by click system
            }
        }
    }
}

/// Handle click on equipment slots (for unequipping)
pub fn handle_equip_slot_click(
    slot_query: Query<(&Interaction, &EquipmentSlotUI), Changed<Interaction>>,
) {
    for (interaction, slot_ui) in &slot_query {
        if *interaction == Interaction::Pressed {
            info!("🎯 Clicked equipment slot: {:?}", slot_ui.slot_type);
            // TODO: Unequip logic later
        }
    }
}
