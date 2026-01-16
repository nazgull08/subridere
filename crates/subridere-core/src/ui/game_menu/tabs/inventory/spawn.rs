use bevy::prelude::*;
use bevy_ui_actions::prelude::*;

use crate::items::EquipmentSlot;

use super::actions::{
    DropToEquipmentSlot, DropToInventorySlot, DropToWorldAction, UseConsumableAction,
};
use super::components::*;
use super::layout::*;

/// Spawn inventory tab content (Equipment + Grid)
pub fn spawn_inventory_content(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(15.0)),
                column_gap: Val::Px(20.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            Name::new("Inventory Content"),
        ))
        .with_children(|content| {
            // Left: Equipment
            spawn_equipment_panel(content, font);

            // Right: Inventory Grid
            spawn_inventory_grid(content, font);
        });
}

// ============================================================
// Equipment Panel
// ============================================================

fn spawn_equipment_panel(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(8.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            Name::new("Equipment Panel"),
        ))
        .with_children(|col| {
            col.spawn((
                Text::new("Equipment"),
                TextFont {
                    font: font.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ));

            // Helmet
            spawn_equip_row(col, font, &[EquipmentSlot::Helmet]);

            // Shoulders + Chest
            spawn_equip_row(
                col,
                font,
                &[
                    EquipmentSlot::LeftPauldron,
                    EquipmentSlot::Cuirass,
                    EquipmentSlot::RightPauldron,
                ],
            );

            // Gauntlets
            spawn_equip_row(
                col,
                font,
                &[EquipmentSlot::LeftGauntlet, EquipmentSlot::RightGauntlet],
            );

            // Greaves
            spawn_equip_row(col, font, &[EquipmentSlot::Greaves]);

            // Boots
            spawn_equip_row(
                col,
                font,
                &[EquipmentSlot::LeftBoot, EquipmentSlot::RightBoot],
            );

            // Separator
            col.spawn(Node {
                height: Val::Px(10.0),
                ..default()
            });

            // Weapons
            spawn_equip_row(
                col,
                font,
                &[EquipmentSlot::MainHand, EquipmentSlot::OffHand],
            );

            // Accessories
            spawn_equip_row(
                col,
                font,
                &[
                    EquipmentSlot::Amulet,
                    EquipmentSlot::LeftRing,
                    EquipmentSlot::RightRing,
                ],
            );
        });
}

fn spawn_equip_row(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    slots: &[EquipmentSlot],
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(EQUIP_GAP),
            ..default()
        })
        .with_children(|row| {
            for &slot in slots {
                spawn_equipment_slot(row, font, slot);
            }
        });
}

fn spawn_equipment_slot(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    slot: EquipmentSlot,
) {
    parent
        .spawn((
            Node {
                width: Val::Px(EQUIP_SLOT_SIZE),
                height: Val::Px(EQUIP_SLOT_SIZE),
                border: UiRect::all(Val::Px(1.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(EQUIP_EMPTY),
            BorderColor(EQUIP_BORDER),
            EquipmentSlotUI { slot },
            Draggable,
            DropTarget,
            OnDrop::new(DropToEquipmentSlot { target_slot: slot }),
            OnDragCancel::new(DropToWorldAction),
            Interaction::None,
            Name::new(format!("Equip: {:?}", slot)),
        ))
        .with_children(|slot_node| {
            // Icon
            slot_node.spawn((
                ImageNode::default(),
                Node {
                    width: Val::Px(EQUIP_SLOT_SIZE - 8.0),
                    height: Val::Px(EQUIP_SLOT_SIZE - 8.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(4.0),
                    top: Val::Px(4.0),
                    ..default()
                },
                Visibility::Hidden,
                SlotIcon,
            ));

            // Label
            slot_node.spawn((
                Text::new(slot_short_name(slot)),
                TextFont {
                    font: font.clone(),
                    font_size: 8.0,
                    ..default()
                },
                TextColor(TEXT_DIM),
                SlotLabel,
            ));
        });
}

fn slot_short_name(slot: EquipmentSlot) -> &'static str {
    match slot {
        EquipmentSlot::Helmet => "Head",
        EquipmentSlot::Cuirass => "Chest",
        EquipmentSlot::LeftPauldron => "L.Sh",
        EquipmentSlot::RightPauldron => "R.Sh",
        EquipmentSlot::LeftGauntlet => "L.Hnd",
        EquipmentSlot::RightGauntlet => "R.Hnd",
        EquipmentSlot::Greaves => "Legs",
        EquipmentSlot::LeftBoot => "L.Ft",
        EquipmentSlot::RightBoot => "R.Ft",
        EquipmentSlot::MainHand => "Main",
        EquipmentSlot::OffHand => "Off",
        EquipmentSlot::Amulet => "Neck",
        EquipmentSlot::LeftRing => "L.Rng",
        EquipmentSlot::RightRing => "R.Rng",
        EquipmentSlot::Belt => "Belt",
    }
}

// ============================================================
// Inventory Grid
// ============================================================

fn spawn_inventory_grid(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(8.0),
                ..default()
            },
            Name::new("Inventory Grid"),
        ))
        .with_children(|col| {
            col.spawn((
                Text::new("Inventory"),
                TextFont {
                    font: font.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ));

            // Grid
            col.spawn((
                Node {
                    display: Display::Grid,
                    grid_template_columns: vec![GridTrack::px(SLOT_SIZE); GRID_COLS],
                    grid_template_rows: vec![GridTrack::px(SLOT_SIZE); GRID_ROWS],
                    row_gap: Val::Px(SLOT_GAP),
                    column_gap: Val::Px(SLOT_GAP),
                    ..default()
                },
                Name::new("Slot Grid"),
            ))
            .with_children(|grid| {
                for i in 0..(GRID_COLS * GRID_ROWS) {
                    spawn_inventory_slot(grid, font, i);
                }
            });
        });
}

fn spawn_inventory_slot(parent: &mut ChildSpawnerCommands, font: &Handle<Font>, index: usize) {
    parent
        .spawn((
            Node {
                width: Val::Px(SLOT_SIZE),
                height: Val::Px(SLOT_SIZE),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(SLOT_EMPTY),
            BorderColor(SLOT_BORDER),
            InventorySlotUI { index },
            Draggable,
            DropTarget,
            OnDrop::new(DropToInventorySlot {
                target_index: index,
            }),
            OnDragCancel::new(DropToWorldAction),
            OnRightClick::new(UseConsumableAction { slot_index: index }),
            Interaction::None,
            Name::new(format!("Slot {}", index)),
        ))
        .with_children(|slot| {
            // Icon
            slot.spawn((
                ImageNode::default(),
                Node {
                    width: Val::Px(SLOT_SIZE - 8.0),
                    height: Val::Px(SLOT_SIZE - 8.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(4.0),
                    top: Val::Px(4.0),
                    ..default()
                },
                Visibility::Hidden,
                SlotIcon,
            ));

            // Quantity
            slot.spawn((
                Text::new(""),
                TextFont {
                    font: font.clone(),
                    font_size: 10.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(2.0),
                    bottom: Val::Px(2.0),
                    ..default()
                },
                SlotQuantity,
            ));
        });
}
