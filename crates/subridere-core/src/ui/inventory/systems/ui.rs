use bevy::prelude::*;

use crate::ui::inventory::{GRID_COLS, GRID_ROWS, SLOT_BORDER_COLOR, SLOT_EMPTY_COLOR, SLOT_GAP, SLOT_SIZE};

/// Marker component for inventory UI root
#[derive(Component)]
pub struct InventoryUI;


#[derive(Component)]
pub struct InventorySlotUI {
    pub slot_index: usize,
}

/// Marker for the icon image inside a slot
#[derive(Component)]
pub struct SlotIcon;

/// Marker for the quantity text inside a slot
#[derive(Component)]
pub struct SlotQuantity;

/// Spawn the inventory UI (called when state enters Open)
pub fn spawn_inventory_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    info!("🎨 Spawning inventory UI");
    
    let font = asset_server.load("fonts/dogica.ttf");
    
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),  // Semi-transparent overlay
        InventoryUI,
        Name::new("Inventory UI Root"),
    ))
    .with_children(|parent| {
        // Main window container
        parent.spawn((
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
            // Header
            window.spawn((
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
            
            // Content area (two columns)
            window.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(550.0),  // 600 - 50 header
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                Name::new("Content"),
            ))
            .with_children(|content| {
                // Left column: Paper doll
                content.spawn((
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Percent(100.0),
                        border: UiRect::right(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.1, 0.2, 0.1)),  // Slight green tint
                    BorderColor(Color::srgb(0.4, 0.4, 0.4)),
                    Name::new("Paper Doll"),
                ))
                .with_children(|paperdoll| {
                    paperdoll.spawn((
                        Text::new("PAPER DOLL\n(placeholder)"),
                        TextFont {
                            font: font.clone(),
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.5, 0.7, 0.5)),
                    ));
                });
                
                // Right column: Inventory + Stats
                content.spawn((
                    Node {
                        width: Val::Px(700.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    Name::new("Right Column"),
                ))
                .with_children(|right_col| {
                    // Inventory grid area
                    right_col.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(400.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.1, 0.1, 0.2)),
                        Name::new("Inventory Grid"),
                    ))
                    .with_children(|grid_area| {
                        spawn_inventory_grid(grid_area, &font);
                    });
                    
                    // Stats area
                    right_col.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(150.0),
                            border: UiRect::top(Val::Px(2.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.1, 0.1)),  // Slight red tint
                        BorderColor(Color::srgb(0.4, 0.4, 0.4)),
                        Name::new("Stats"),
                    ))
                    .with_children(|stats| {
                        stats.spawn((
                            Text::new("STATS\n(placeholder)"),
                            TextFont {
                                font: font.clone(),
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.7, 0.5, 0.5)),
                        ));
                    });
                });
            });
        });
    });
}

/// Despawn the inventory UI (called when state exits Open)
pub fn despawn_inventory_ui(
    mut commands: Commands,
    query: Query<Entity, With<InventoryUI>>,
) {
    for entity in &query {
        info!("🗑️ Despawning inventory UI");
        commands.entity(entity).despawn_recursive();
    }
}

/// Spawn the 5x4 grid of inventory slots
fn spawn_inventory_grid(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    // Container for the grid
    parent.spawn((
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
        // Spawn 20 slots (5x4)
        for i in 0..(GRID_COLS * GRID_ROWS) {
            grid.spawn((
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
                InventorySlotUI { slot_index: i },
                Interaction::default(),
                Name::new(format!("Slot {}", i)),
            ))
            .with_children(|slot| {
                // Icon (hidden by default)
                slot.spawn((
                    ImageNode::default(),
                    Node {
                        width: Val::Px(56.0),  // Slightly smaller than slot
                        height: Val::Px(56.0),
                        ..default()
                    },
                    Visibility::Hidden,  // Start hidden
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
    });
}
