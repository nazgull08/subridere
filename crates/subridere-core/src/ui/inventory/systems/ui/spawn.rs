use super::components::*;
use crate::ui::inventory::layout::*;
use bevy::prelude::*;

/// Spawn the inventory UI (called when state enters Open)
pub fn spawn_inventory_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("🎨 Spawning inventory UI");

    let font = asset_server.load("fonts/dogica.ttf");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            InventoryUI,
            Name::new("Inventory UI Root"),
        ))
        .with_children(|parent| {
            spawn_main_window(parent, &font);
        });
}

/// Despawn the inventory UI (called when state exits Open)
pub fn despawn_inventory_ui(mut commands: Commands, query: Query<Entity, With<InventoryUI>>) {
    for entity in &query {
        info!("🗑️ Despawning inventory UI");
        commands.entity(entity).despawn();
    }
}

/// Spawn the main inventory window
fn spawn_main_window(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    parent
        .spawn((
            Node {
                width: Val::Px(1000.0),
                height: Val::Px(600.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            Name::new("Inventory Window"),
        ))
        .with_children(|window| {
            spawn_header(window, font);
            spawn_content(window, font);
        });
}

/// Spawn the header with title
fn spawn_header(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::bottom(Val::Px(2.0)),
                ..default()
            },
            BorderColor(Color::srgb(0.4, 0.4, 0.4)),
            Name::new("Header"),
        ))
        .with_children(|header| {
            header.spawn((
                Text::new("INVENTORY"),
                TextFont {
                    font: font.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}

/// Spawn the main content area (paper doll + inventory + stats)
fn spawn_content(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(550.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            Name::new("Content"),
        ))
        .with_children(|content| {
            spawn_paper_doll(content, font);
            spawn_right_column(content, font);
        });
}

/// Spawn the right column (inventory grid + stats)
fn spawn_right_column(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    parent
        .spawn((
            Node {
                width: Val::Px(700.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            Name::new("Right Column"),
        ))
        .with_children(|right_col| {
            spawn_inventory_grid_area(right_col, font);
            spawn_stats_panel(right_col, font);
        });
}

/// Spawn the inventory grid area
fn spawn_inventory_grid_area(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(400.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.2)),
            Name::new("Inventory Grid Area"),
        ))
        .with_children(|grid_area| {
            spawn_inventory_grid(grid_area, font);
        });
}

/// Spawn the 5x4 grid of inventory slots
fn spawn_inventory_grid(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    parent
        .spawn((
            Node {
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::px(GRID_COLS as u16, SLOT_SIZE),
                grid_template_rows: RepeatedGridTrack::px(GRID_ROWS as u16, SLOT_SIZE),
                column_gap: Val::Px(SLOT_GAP),
                row_gap: Val::Px(SLOT_GAP),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            Name::new("Grid Container"),
        ))
        .with_children(|grid| {
            for i in 0..(GRID_COLS * GRID_ROWS) {
                spawn_inventory_slot(grid, i, font);
            }
        });
}

/// Spawn a single inventory slot
fn spawn_inventory_slot(parent: &mut ChildSpawnerCommands, slot_index: usize, font: &Handle<Font>) {
    parent
        .spawn((
            Node {
                width: Val::Px(SLOT_SIZE),
                height: Val::Px(SLOT_SIZE),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Relative,
                ..default()
            },
            BackgroundColor(SLOT_EMPTY_COLOR),
            BorderColor(SLOT_BORDER_COLOR),
            Interaction::default(),
            InventorySlotUI { slot_index },
            Name::new(format!("Slot {}", slot_index)),
        ))
        .with_children(|slot| {
            // Icon (hidden by default)
            slot.spawn((
                ImageNode::default(),
                Node {
                    width: Val::Px(56.0),
                    height: Val::Px(56.0),
                    ..default()
                },
                Visibility::Hidden,
                SlotIcon,
                Name::new("Icon"),
            ));

            // Quantity text (bottom right corner)
            slot.spawn((
                Text::new(""),
                TextFont {
                    font: font.clone(),
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(2.0),
                    right: Val::Px(2.0),
                    ..default()
                },
                SlotQuantity,
                Name::new("Quantity"),
            ));
        });
}

/// Spawn the stats panel
fn spawn_stats_panel(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(150.0),
                border: UiRect::top(Val::Px(2.0)),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(15.0)),
                row_gap: Val::Px(8.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.12, 0.12)),
            BorderColor(Color::srgb(0.4, 0.4, 0.4)),
            Name::new("Stats Panel"),
        ))
        .with_children(|stats_panel| {
            // Title
            stats_panel.spawn((
                Text::new("STATS"),
                TextFont {
                    font: font.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
            ));

            // Separator
            stats_panel.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(1.0),
                    margin: UiRect::bottom(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.4, 0.4, 0.4)),
            ));

            spawn_stat_line(
                stats_panel,
                "HP: ",
                Color::srgb(1.0, 0.3, 0.3),
                StatsHpText,
                font,
            );
            spawn_stat_line(
                stats_panel,
                "MP: ",
                Color::srgb(0.3, 0.3, 1.0),
                StatsMpText,
                font,
            );
            spawn_stat_line(
                stats_panel,
                "SP: ",
                Color::srgb(0.3, 1.0, 0.3),
                StatsSpText,
                font,
            );
        });
}

/// Spawn a single stat line (label + value)
fn spawn_stat_line<T: Component>(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    color: Color,
    marker: T,
    font: &Handle<Font>,
) {
    parent
        .spawn((
            Text::new(label),
            TextFont {
                font: font.clone(),
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
        ))
        .with_child((
            TextSpan::from("0 / 0"),
            TextFont {
                font: font.clone(),
                font_size: 14.0,
                ..default()
            },
            TextColor(color),
            marker,
        ));
}

/// Spawn the paper doll area (left column)
fn spawn_paper_doll(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    parent
        .spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Percent(100.0),
                border: UiRect::right(Val::Px(2.0)),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                row_gap: Val::Px(8.0),
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.12, 0.15, 0.12)),
            BorderColor(Color::srgb(0.4, 0.4, 0.4)),
            Name::new("Paper Doll"),
        ))
        .with_children(|paperdoll| {
            // Title
            paperdoll.spawn((
                Text::new("EQUIPMENT"),
                TextFont {
                    font: font.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            // Helmet (top)
            spawn_equipment_slot(paperdoll, EquipmentSlotType::Helmet, font);

            // Shoulders + Chest (row)
            paperdoll
                .spawn((Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    column_gap: Val::Px(5.0),
                    ..default()
                },))
                .with_children(|row| {
                    spawn_equipment_slot(row, EquipmentSlotType::LeftPauldron, font);
                    spawn_equipment_slot(row, EquipmentSlotType::Chest, font);
                    spawn_equipment_slot(row, EquipmentSlotType::RightPauldron, font);
                });

            // Gloves (row)
            paperdoll
                .spawn((Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    column_gap: Val::Px(5.0),
                    ..default()
                },))
                .with_children(|row| {
                    spawn_equipment_slot(row, EquipmentSlotType::LeftGlove, font);
                    // Center spacer
                    row.spawn(Node {
                        width: Val::Px(60.0),
                        ..default()
                    });
                    spawn_equipment_slot(row, EquipmentSlotType::RightGlove, font);
                });

            // Greaves
            spawn_equipment_slot(paperdoll, EquipmentSlotType::Greaves, font);

            // Boots (row)
            paperdoll
                .spawn((Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceEvenly,
                    column_gap: Val::Px(10.0),
                    ..default()
                },))
                .with_children(|row| {
                    spawn_equipment_slot(row, EquipmentSlotType::LeftBoot, font);
                    spawn_equipment_slot(row, EquipmentSlotType::RightBoot, font);
                });

            // Separator
            paperdoll.spawn((
                Node {
                    width: Val::Percent(80.0),
                    height: Val::Px(1.0),
                    margin: UiRect::vertical(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.4, 0.4, 0.4)),
            ));

            // Weapons (row)
            paperdoll
                .spawn((Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceEvenly,
                    column_gap: Val::Px(10.0),
                    ..default()
                },))
                .with_children(|row| {
                    spawn_equipment_slot(row, EquipmentSlotType::MainHand, font);
                    spawn_equipment_slot(row, EquipmentSlotType::OffHand, font);
                });
        });
}

/// Spawn a single equipment slot
fn spawn_equipment_slot(
    parent: &mut ChildSpawnerCommands,
    slot_type: EquipmentSlotType,
    font: &Handle<Font>,
) {
    const EQUIP_SLOT_SIZE: f32 = 60.0;

    parent
        .spawn((
            Node {
                width: Val::Px(EQUIP_SLOT_SIZE),
                height: Val::Px(EQUIP_SLOT_SIZE),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Relative,
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            BorderColor(Color::srgb(0.35, 0.35, 0.35)),
            Interaction::default(), // ДОБАВИТЬ
            EquipmentSlotUI { slot_type },
            Name::new(format!("Equip_{:?}", slot_type)),
        ))
        .with_children(|slot| {
            // Icon (hidden by default, shown when equipped)
            slot.spawn((
                ImageNode::default(),
                Node {
                    width: Val::Px(52.0), // Чуть меньше слота
                    height: Val::Px(52.0),
                    ..default()
                },
                Visibility::Hidden,
                EquipSlotIcon,
                Name::new("EquipIcon"),
            ));

            // Label (shown when empty, hidden when equipped)
            slot.spawn((
                Text::new(slot_type.name()),
                TextFont {
                    font: font.clone(),
                    font_size: 9.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
                Node {
                    position_type: PositionType::Absolute,
                    ..default()
                },
                Name::new("Label"),
            ));
        });
}
