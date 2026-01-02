use crate::game_init::assets::GameAssets;
use crate::inventory::{Equipment, Inventory};
use crate::player::component::Player;
use crate::stats::health::component::Health;
use crate::stats::mana::component::Mana;
use crate::stats::stamina::component::Stamina;
use crate::ui::inventory::layout::{SLOT_EMPTY_COLOR, SLOT_FILLED_COLOR};
use crate::ui::inventory::systems::ui::EquipSlotIcon;
use crate::ui::inventory::systems::{
    EquipmentSlotType, EquipmentSlotUI, InventorySlotUI, SlotIcon, SlotQuantity, StatsHpText,
    StatsMpText, StatsSpText,
};
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
                    // Match item to icon
                    let icon_handle = match item.item_id.as_str() {
                        "wooden_staff" => Some(&game_assets.wooden_staff_icon),
                        "iron_helmet" => Some(&game_assets.iron_helmet_icon),
                        _ => None,
                    };

                    if let Some(icon) = icon_handle {
                        image.image = icon.clone();
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

/// Update stats display in inventory UI
/// Update stats display in inventory UI
pub fn sync_stats_to_ui(
    player_query: Query<(&Health, Option<&Mana>, Option<&Stamina>), With<Player>>,
    mut hp_query: Query<&mut TextSpan, With<StatsHpText>>,
    mut mp_query: Query<&mut TextSpan, (With<StatsMpText>, Without<StatsHpText>)>,
    mut sp_query: Query<
        &mut TextSpan,
        (
            With<StatsSpText>,
            Without<StatsHpText>,
            Without<StatsMpText>,
        ),
    >,
) {
    let Ok((health, mana, stamina)) = player_query.single() else {
        warn!("⚠️ Player not found in sync_stats_to_ui!");
        return;
    };

    if let Ok(mut text) = hp_query.single_mut() {
        let formatted = format!("{:.0} / {:.0}", health.current, health.max);
        **text = formatted;
    } else {
        warn!("⚠️ HP text component not found!");
    }

    if let (Some(m), Ok(mut text)) = (mana, mp_query.single_mut()) {
        let formatted = format!("{:.0} / {:.0}", m.current, m.max);
        **text = formatted;
    } else {
        warn!("⚠️ MP text component or mana not found!");
    }

    if let (Some(s), Ok(mut text)) = (stamina, sp_query.single_mut()) {
        let formatted = format!("{:.0} / {:.0}", s.current, s.max);
        **text = formatted;
    } else {
        warn!("⚠️ SP text component or stamina not found!");
    }
}

/// Update equipment slot UI based on player's equipment
pub fn sync_equipment_to_ui(
    equipment_query: Query<&Equipment, With<Player>>,
    game_assets: Res<GameAssets>,
    mut slot_ui_query: Query<(&EquipmentSlotUI, &mut BackgroundColor, &Children)>,
    mut icon_query: Query<(&mut ImageNode, &mut Visibility), With<EquipSlotIcon>>,
    mut text_query: Query<(&mut Text, &mut Visibility), Without<EquipSlotIcon>>,
) {
    let Ok(equipment) = equipment_query.single() else {
        return;
    };

    for (slot_ui, mut bg_color, children) in &mut slot_ui_query {
        // Get item ID for this slot
        let item_id = match slot_ui.slot_type {
            EquipmentSlotType::Helmet => &equipment.helmet,
            EquipmentSlotType::LeftPauldron => &equipment.left_pauldron,
            EquipmentSlotType::RightPauldron => &equipment.right_pauldron,
            EquipmentSlotType::Chest => &equipment.chest,
            EquipmentSlotType::LeftGlove => &equipment.left_glove,
            EquipmentSlotType::RightGlove => &equipment.right_glove,
            EquipmentSlotType::Greaves => &equipment.greaves,
            EquipmentSlotType::LeftBoot => &equipment.left_boot,
            EquipmentSlotType::RightBoot => &equipment.right_boot,
            EquipmentSlotType::MainHand => &equipment.main_hand,
            EquipmentSlotType::OffHand => &equipment.off_hand,
        };

        // Update visual based on whether slot is filled
        if let Some(item) = item_id {
            // Slot is filled
            *bg_color = BackgroundColor(Color::srgb(0.25, 0.2, 0.15));

            for child in children.iter() {
                // Show icon, hide text
                if let Ok((mut image, mut icon_vis)) = icon_query.get_mut(child) {
                    // Match item to icon
                    let icon_handle = match item.as_str() {
                        "wooden_staff" => Some(&game_assets.wooden_staff_icon),
                        "iron_helmet" => Some(&game_assets.iron_helmet_icon),
                        _ => None,
                    };

                    if let Some(icon) = icon_handle {
                        image.image = icon.clone();
                        *icon_vis = Visibility::Visible;
                    }
                }

                if let Ok((_, mut text_vis)) = text_query.get_mut(child) {
                    *text_vis = Visibility::Hidden; // Hide label
                }
            }
        } else {
            // Slot is empty
            *bg_color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));

            for child in children.iter() {
                // Hide icon, show text
                if let Ok((_, mut icon_vis)) = icon_query.get_mut(child) {
                    *icon_vis = Visibility::Hidden;
                }

                if let Ok((mut text, mut text_vis)) = text_query.get_mut(child) {
                    text.0 = slot_ui.slot_type.name().to_string();
                    *text_vis = Visibility::Visible;
                }
            }
        }
    }
}
