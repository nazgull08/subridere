// ui/inventory/sync.rs — Sync game data to UI visuals

use bevy::prelude::*;

use crate::inventory::component::{Equipment, Inventory};
use crate::items::ItemRegistry;
use crate::player::component::Player;
use crate::stats::health::component::Health;
use crate::stats::mana::component::Mana;
use crate::stats::stamina::component::Stamina;

use super::components::*;
use super::layout::*;

/// Sync inventory data to slot visuals
pub fn sync_inventory_slots(
    inventory_query: Query<&Inventory, With<Player>>,
    registry: Res<ItemRegistry>,
    mut slot_query: Query<(&InventorySlotUI, &Children, &mut BackgroundColor)>,
    mut icon_query: Query<(&mut ImageNode, &mut Visibility), With<SlotIcon>>,
    mut text_query: Query<&mut Text, With<SlotQuantity>>,
) {
    let Ok(inventory) = inventory_query.single() else {
        return;
    };

    for (slot_ui, children, mut bg) in &mut slot_query {
        let maybe_stack = inventory.get(slot_ui.index);

        if let Some(stack) = maybe_stack {
            // Slot has item
            *bg = BackgroundColor(SLOT_FILLED);

            // Update children
            for child in children.iter() {
                // Icon
                if let Ok((mut image, mut vis)) = icon_query.get_mut(child) {
                    if let Some(icon_handle) = registry.icon(stack.id) {
                        image.image = icon_handle.clone();
                        *vis = Visibility::Visible;
                    }
                }

                // Quantity
                if let Ok(mut text) = text_query.get_mut(child) {
                    if stack.quantity > 1 {
                        text.0 = format!("×{}", stack.quantity);
                    } else {
                        text.0.clear();
                    }
                }
            }
        } else {
            // Slot empty
            *bg = BackgroundColor(SLOT_EMPTY);

            for child in children.iter() {
                if let Ok((_, mut vis)) = icon_query.get_mut(child) {
                    *vis = Visibility::Hidden;
                }
                if let Ok(mut text) = text_query.get_mut(child) {
                    text.0.clear();
                }
            }
        }
    }
}

/// Sync equipment data to slot visuals
pub fn sync_equipment_slots(
    equipment_query: Query<&Equipment, With<Player>>,
    registry: Res<ItemRegistry>,
    mut slot_query: Query<(&EquipmentSlotUI, &Children, &mut BackgroundColor)>,
    mut icon_query: Query<(&mut ImageNode, &mut Visibility), With<SlotIcon>>,
    mut label_query: Query<&mut Visibility, (With<SlotLabel>, Without<SlotIcon>)>,
) {
    let Ok(equipment) = equipment_query.single() else {
        return;
    };

    for (slot_ui, children, mut bg) in &mut slot_query {
        let maybe_id = equipment.get(slot_ui.slot);

        if let Some(id) = maybe_id {
            // Slot has item
            *bg = BackgroundColor(EQUIP_FILLED);

            for child in children.iter() {
                // Show icon
                if let Ok((mut image, mut vis)) = icon_query.get_mut(child) {
                    if let Some(icon_handle) = registry.icon(id) {
                        image.image = icon_handle.clone();
                        *vis = Visibility::Visible;
                    }
                }

                // Hide label
                if let Ok(mut vis) = label_query.get_mut(child) {
                    *vis = Visibility::Hidden;
                }
            }
        } else {
            // Slot empty
            *bg = BackgroundColor(EQUIP_EMPTY);

            for child in children.iter() {
                // Hide icon
                if let Ok((_, mut vis)) = icon_query.get_mut(child) {
                    *vis = Visibility::Hidden;
                }

                // Show label
                if let Ok(mut vis) = label_query.get_mut(child) {
                    *vis = Visibility::Visible;
                }
            }
        }
    }
}

/// Sync player stats to UI
pub fn sync_stats_display(
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
        return;
    };

    if let Ok(mut text) = hp_query.single_mut() {
        **text = format!("{:.0} / {:.0}", health.current, health.max);
    }

    if let (Some(m), Ok(mut text)) = (mana, mp_query.single_mut()) {
        **text = format!("{:.0} / {:.0}", m.current, m.max);
    }

    if let (Some(s), Ok(mut text)) = (stamina, sp_query.single_mut()) {
        **text = format!("{:.0} / {:.0}", s.current, s.max);
    }
}
