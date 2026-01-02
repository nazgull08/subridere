use super::ui::{InventorySlotUI, SlotIcon, SlotQuantity};
use crate::game_init::assets::GameAssets;
use crate::player::component::Player;
use crate::ui::inventory::data::Inventory;
use crate::ui::inventory::layout::{SLOT_EMPTY_COLOR, SLOT_FILLED_COLOR};
use bevy::prelude::*;

/// Update inventory slot UI based on player's inventory data
pub fn sync_inventory_to_ui(
    inventory_query: Query<&Inventory, With<Player>>,
    game_assets: Res<GameAssets>,
    mut slot_ui_query: Query<(&InventorySlotUI, &mut BackgroundColor, &Children)>,
    mut icon_query: Query<(&mut ImageNode, &mut Visibility), With<SlotIcon>>,
    mut text_query: Query<&mut Text, With<SlotQuantity>>,
) {
    // Get player's inventory
    let Ok(inventory) = inventory_query.single() else {
        return;
    };

    // Update each slot UI
    for (slot_ui, mut bg_color, children) in &mut slot_ui_query {
        let slot_index = slot_ui.slot_index;

        // Check if slot has an item
        if let Some(item) = inventory.slots.get(slot_index).and_then(|s| s.as_ref()) {
            // Slot is filled
            *bg_color = BackgroundColor(SLOT_FILLED_COLOR);

            // Show icon and quantity
            for child in children.iter() {
                // Update icon
                if let Ok((mut image, mut visibility)) = icon_query.get_mut(child) {
                    // For now, only wooden_staff has icon
                    if item.item_id == "wooden_staff" {
                        image.image = game_assets.wooden_staff_icon.clone();
                        *visibility = Visibility::Visible;
                    }
                }

                // Update quantity text
                if let Ok(mut text) = text_query.get_mut(child) {
                    if item.quantity > 1 {
                        text.0 = format!("×{}", item.quantity);
                    } else {
                        text.0 = String::new(); // Don't show ×1
                    }
                }
            }
        } else {
            // Slot is empty
            *bg_color = BackgroundColor(SLOT_EMPTY_COLOR);

            // Hide icon and quantity
            for child in children.iter() {
                if let Ok((_, mut visibility)) = icon_query.get_mut(child) {
                    *visibility = Visibility::Hidden;
                }

                if let Ok(mut text) = text_query.get_mut(child) {
                    text.0 = String::new();
                }
            }
        }
    }
}
