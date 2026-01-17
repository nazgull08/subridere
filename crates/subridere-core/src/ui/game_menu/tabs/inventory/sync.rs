use bevy::prelude::*;
use bevy_ui_actions::{DragPhase, DragState};

use crate::inventory::component::{Equipment, Inventory};
use crate::items::ItemRegistry;
use crate::player::component::Player;

use super::components::*;
use super::layout::*;

/// Sync all slot visuals (icons, quantities, backgrounds)
pub fn sync_slots(
    inventory_query: Query<(&Inventory, &Equipment), With<Player>>,
    registry: Res<ItemRegistry>,
    mut slot_query: Query<(&SlotUI, &Children, &mut BackgroundColor)>,
    mut icon_query: Query<(&mut ImageNode, &mut Visibility), With<SlotIcon>>,
    mut quantity_query: Query<&mut Text, With<SlotQuantity>>,
    mut label_query: Query<&mut Visibility, (With<SlotLabel>, Without<SlotIcon>)>,
) {
    let Ok((inventory, equipment)) = inventory_query.single() else {
        return;
    };

    for (slot_ui, children, mut bg) in &mut slot_query {
        match slot_ui.id {
            SlotId::Inventory(index) => {
                sync_inventory_slot(
                    index,
                    inventory,
                    &registry,
                    &mut bg,
                    children,
                    &mut icon_query,
                    &mut quantity_query,
                );
            }
            SlotId::Equipment(slot) => {
                sync_equipment_slot(
                    slot,
                    equipment,
                    &registry,
                    &mut bg,
                    children,
                    &mut icon_query,
                    &mut label_query,
                );
            }
        }
    }
}

fn sync_inventory_slot(
    index: usize,
    inventory: &Inventory,
    registry: &ItemRegistry,
    bg: &mut BackgroundColor,
    children: &Children,
    icon_query: &mut Query<(&mut ImageNode, &mut Visibility), With<SlotIcon>>,
    quantity_query: &mut Query<&mut Text, With<SlotQuantity>>,
) {
    let maybe_stack = inventory.get(index);

    if let Some(stack) = maybe_stack {
        *bg = BackgroundColor(SLOT_FILLED);

        for child in children.iter() {
            if let Ok((mut image, mut vis)) = icon_query.get_mut(child) {
                if let Some(icon_handle) = registry.icon(stack.id) {
                    image.image = icon_handle.clone();
                    *vis = Visibility::Visible;
                }
            }

            if let Ok(mut text) = quantity_query.get_mut(child) {
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

            if let Ok(mut text) = quantity_query.get_mut(child) {
                text.0.clear();
            }
        }
    }
}

fn sync_equipment_slot(
    slot: crate::items::EquipmentSlot,
    equipment: &Equipment,
    registry: &ItemRegistry,
    bg: &mut BackgroundColor,
    children: &Children,
    icon_query: &mut Query<(&mut ImageNode, &mut Visibility), With<SlotIcon>>,
    label_query: &mut Query<&mut Visibility, (With<SlotLabel>, Without<SlotIcon>)>,
) {
    let item_id = equipment.get(slot);

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

/// Dim source slot while dragging
pub fn sync_drag_visual(
    drag_state: Res<DragState>,
    mut slots: Query<(Entity, &mut BackgroundColor), With<SlotUI>>,
) {
    if drag_state.phase != DragPhase::Active {
        return;
    }

    let Some(dragging_entity) = drag_state.dragging else {
        return;
    };

    const DRAGGING_DIM: Color = Color::srgba(0.1, 0.1, 0.1, 0.7);

    for (entity, mut bg) in &mut slots {
        if entity == dragging_entity {
            *bg = BackgroundColor(DRAGGING_DIM);
        }
    }
}
