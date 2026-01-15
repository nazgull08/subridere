use bevy::prelude::*;
use bevy_ui_actions::{DragPhase, DragState};

use crate::inventory::component::{Equipment, Inventory};
use crate::items::ItemRegistry;
use crate::player::component::Player;

use super::components::*;
use super::layout::*;

/// Sync inventory slots
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
            *bg = BackgroundColor(SLOT_FILLED);

            for child in children.iter() {
                if let Ok((mut image, mut vis)) = icon_query.get_mut(child) {
                    if let Some(icon_handle) = registry.icon(stack.id) {
                        image.image = icon_handle.clone();
                        *vis = Visibility::Visible;
                    }
                }

                if let Ok(mut text) = text_query.get_mut(child) {
                    if stack.quantity > 1 {
                        text.0 = format!("{}", stack.quantity);
                    } else {
                        text.0.clear();
                    }
                }
            }
        } else {
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

/// Sync equipment slots
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
        let item_id = equipment.get(slot_ui.slot);

        if let Some(id) = item_id {
            *bg = BackgroundColor(EQUIP_FILLED);

            for child in children.iter() {
                if let Ok((mut image, mut vis)) = icon_query.get_mut(child) {
                    if let Some(icon_handle) = registry.icon(id) {
                        image.image = icon_handle.clone();
                        *vis = Visibility::Visible;
                    }
                }

                if let Ok(mut vis) = label_query.get_mut(child) {
                    *vis = Visibility::Hidden;
                }
            }
        } else {
            *bg = BackgroundColor(EQUIP_EMPTY);

            for child in children.iter() {
                if let Ok((_, mut vis)) = icon_query.get_mut(child) {
                    *vis = Visibility::Hidden;
                }

                if let Ok(mut vis) = label_query.get_mut(child) {
                    *vis = Visibility::Visible;
                }
            }
        }
    }
}

/// Dim source slot while dragging
pub fn sync_drag_visual(
    drag_state: Res<DragState>,
    mut inv_slots: Query<(Entity, &mut BackgroundColor), With<InventorySlotUI>>,
    mut equip_slots: Query<
        (Entity, &mut BackgroundColor),
        (With<EquipmentSlotUI>, Without<InventorySlotUI>),
    >,
) {
    if drag_state.phase != DragPhase::Active {
        return;
    }

    let Some(dragging_entity) = drag_state.dragging else {
        return;
    };

    const DRAGGING_DIM: Color = Color::srgba(0.1, 0.1, 0.1, 0.7);

    for (entity, mut bg) in &mut inv_slots {
        if entity == dragging_entity {
            *bg = BackgroundColor(DRAGGING_DIM);
        }
    }

    for (entity, mut bg) in &mut equip_slots {
        if entity == dragging_entity {
            *bg = BackgroundColor(DRAGGING_DIM);
        }
    }
}
