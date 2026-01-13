// ui/inventory/spawn.rs — Spawn inventory UI

use bevy::prelude::*;
use bevy_ui_actions::prelude::*;

use crate::items::EquipmentSlot;

use super::actions::{DropToEquipmentSlot, DropToInventorySlot};
use super::components::*;
use super::layout::*;

/// Spawn the complete inventory UI
pub fn spawn_inventory_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/dogica.ttf");

    commands
        .spawn((
            // Root container — fullscreen overlay
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            GlobalZIndex(100),
            InventoryRoot,
            Name::new("Inventory Root"),
        ))
        .with_children(|root| {
            // Main panel container
            root.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    padding: UiRect::all(Val::Px(20.0)),
                    column_gap: Val::Px(20.0),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(PANEL_BG),
                BorderColor(PANEL_BORDER),
                Name::new("Inventory Panel"),
            ))
            .with_children(|panel| {
                // Left: Equipment panel
                spawn_equipment_panel(panel, &font);

                // Center: Inventory grid
                spawn_inventory_grid(panel, &font);

                // Right: Stats panel
                spawn_stats_panel(panel, &font);
            });
        });

    info!("📦 Inventory UI spawned");
}

/// Despawn inventory UI
pub fn despawn_inventory_ui(mut commands: Commands, query: Query<Entity, With<InventoryRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
    info!("📦 Inventory UI despawned");
}

// ============================================================
// Equipment Panel (Left)
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
            // Title
            col.spawn((
                Text::new("Equipment"),
                TextFont {
                    font: font.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ));

            // Equipment layout
            // Row 1: Helmet
            spawn_equip_row(col, font, &[EquipmentSlot::Helmet]);

            // Row 2: L.Pauldron, Cuirass, R.Pauldron
            spawn_equip_row(
                col,
                font,
                &[
                    EquipmentSlot::LeftPauldron,
                    EquipmentSlot::Cuirass,
                    EquipmentSlot::RightPauldron,
                ],
            );

            // Row 3: L.Gauntlet, R.Gauntlet
            spawn_equip_row(
                col,
                font,
                &[EquipmentSlot::LeftGauntlet, EquipmentSlot::RightGauntlet],
            );

            // Row 4: Greaves
            spawn_equip_row(col, font, &[EquipmentSlot::Greaves]);

            // Row 5: L.Boot, R.Boot
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

            // Weapons row
            spawn_equip_row(
                col,
                font,
                &[EquipmentSlot::MainHand, EquipmentSlot::OffHand],
            );

            // Accessories row
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
            Interaction::None,
            Name::new(format!("Equip: {:?}", slot)),
        ))
        .with_children(|slot_node| {
            // Icon — absolute positioned, centered
            slot_node.spawn((
                ImageNode::default(),
                Node {
                    width: Val::Px(EQUIP_SLOT_SIZE - 8.0),
                    height: Val::Px(EQUIP_SLOT_SIZE - 8.0),
                    position_type: PositionType::Absolute,
                    // Center the absolute element
                    left: Val::Px(4.0),
                    top: Val::Px(4.0),
                    ..default()
                },
                Visibility::Hidden,
                SlotIcon,
            ));

            // Label — stays in flex flow, centered by parent
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
// Inventory Grid (Center)
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
            // Title
            col.spawn((
                Text::new("Inventory"),
                TextFont {
                    font: font.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ));

            // Grid container
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
            // bevy_ui_actions components
            Draggable,
            DropTarget,
            OnDrop::new(DropToInventorySlot {
                target_index: index,
            }),
            Interaction::None,
            Name::new(format!("Slot {}", index)),
        ))
        .with_children(|slot| {
            // Icon (hidden by default)
            slot.spawn((
                ImageNode::default(),
                Node {
                    width: Val::Px(SLOT_SIZE - 12.0),
                    height: Val::Px(SLOT_SIZE - 12.0),
                    ..default()
                },
                Visibility::Hidden,
                SlotIcon,
            ));

            // Quantity text (bottom-right)
            slot.spawn((
                Text::new(""),
                TextFont {
                    font: font.clone(),
                    font_size: 12.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(2.0),
                    bottom: Val::Px(0.0),
                    ..default()
                },
                SlotQuantity,
            ));
        });
}

// ============================================================
// Stats Panel (Right)
// ============================================================

fn spawn_stats_panel(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                padding: UiRect::all(Val::Px(10.0)),
                min_width: Val::Px(120.0),
                ..default()
            },
            StatsPanel,
            Name::new("Stats Panel"),
        ))
        .with_children(|col| {
            // Title
            col.spawn((
                Text::new("Stats"),
                TextFont {
                    font: font.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ));

            // HP
            spawn_stat_row(col, font, "HP:", Color::srgb(0.9, 0.3, 0.3), StatsHpText);

            // MP
            spawn_stat_row(col, font, "MP:", Color::srgb(0.3, 0.3, 0.9), StatsMpText);

            // SP
            spawn_stat_row(col, font, "SP:", Color::srgb(0.3, 0.9, 0.3), StatsSpText);
        });
}

fn spawn_stat_row<M: Component>(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    label: &str,
    color: Color,
    marker: M,
) {
    parent
        .spawn((
            Text::new(label),
            TextFont {
                font: font.clone(),
                font_size: 12.0,
                ..default()
            },
            TextColor(TEXT_DIM),
        ))
        .with_child((
            TextSpan::new("0 / 0"),
            TextFont {
                font: font.clone(),
                font_size: 12.0,
                ..default()
            },
            TextColor(color),
            marker,
        ));
}
