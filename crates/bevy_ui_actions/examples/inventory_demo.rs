//! Комплексный пример — демо инвентаря.
//!
//! Демонстрирует ВСЕ возможности библиотеки вместе:
//! - OnClick для выбора
//! - OnRightClick для контекстного меню  
//! - Drag & Drop между слотами
//! - Tooltip с описанием
//! - InteractiveVisual для feedback
//!
//! Запуск: `cargo run --example inventory_demo -p bevy_ui_actions`

use bevy::prelude::*;
use bevy_ui_actions::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiActionsPlugin)
        .init_resource::<SelectedSlot>()
        .add_systems(Startup, setup)
        .add_systems(Update, update_selection_display)
        .run();
}

#[derive(Resource, Default)]
struct SelectedSlot(Option<usize>);

// ============ Actions ============

struct SelectSlotAction {
    index: usize,
}

impl UiAction for SelectSlotAction {
    fn execute(&self, world: &mut World) {
        world.resource_mut::<SelectedSlot>().0 = Some(self.index);
        info!("Selected slot {}", self.index);
    }
}

struct SlotContextMenuAction {
    index: usize,
}

impl UiAction for SlotContextMenuAction {
    fn execute(&self, _world: &mut World) {
        info!("Context menu for slot {} (Use/Drop/Split)", self.index);
    }
}

struct DropToSlotAction {
    target_index: usize,
}

impl UiAction for DropToSlotAction {
    fn execute(&self, world: &mut World) {
        let drag_state = world.resource::<DragState>();
        if let Some(_dragged) = drag_state.dragging {
            info!("Moved item to slot {}", self.target_index);
        }
    }
}

// ============ UI ============

#[derive(Component)]
struct SelectionText;

#[derive(Component)]
struct SlotMarker(usize);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(20.0),
            ..default()
        })
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Inventory Demo"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
            ));

            // Selection display
            parent.spawn((
                Text::new("Selected: None"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                SelectionText,
            ));

            // Inventory grid (3x3)
            parent
                .spawn(Node {
                    display: Display::Grid,
                    grid_template_columns: vec![GridTrack::px(70.0); 3],
                    grid_template_rows: vec![GridTrack::px(70.0); 3],
                    row_gap: Val::Px(5.0),
                    column_gap: Val::Px(5.0),
                    ..default()
                })
                .with_children(|grid| {
                    let items = [
                        Some(("Sword", Color::srgb(0.7, 0.5, 0.3))),
                        Some(("Shield", Color::srgb(0.4, 0.4, 0.6))),
                        Some(("Potion", Color::srgb(0.3, 0.7, 0.4))),
                        None,
                        Some(("Ring", Color::srgb(0.8, 0.7, 0.2))),
                        None,
                        Some(("Scroll", Color::srgb(0.6, 0.5, 0.7))),
                        None,
                        None,
                    ];

                    for (index, item) in items.iter().enumerate() {
                        spawn_inventory_slot(grid, index, item.clone());
                    }
                });

            // Hints
            parent.spawn((
                Text::new("Left click: Select | Right click: Context menu | Drag: Move items"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
            ));

            parent.spawn((
                Text::new("Hover over items for tooltips"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
            ));
        });
}

fn spawn_inventory_slot(
    parent: &mut ChildSpawnerCommands,
    index: usize,
    item: Option<(&'static str, Color)>,
) {
    let bg_color = item
        .map(|(_, c)| c)
        .unwrap_or(Color::srgb(0.15, 0.15, 0.15));

    let tooltip_text = item
        .map(|(name, _)| format!("{}\nClick to select\nRight-click for options", name))
        .unwrap_or_else(|| "Empty slot".to_string());

    let mut slot = parent.spawn((
        Button,
        Node {
            width: Val::Px(64.0),
            height: Val::Px(64.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(bg_color),
        BorderColor(Color::srgb(0.3, 0.3, 0.3)),
        SlotMarker(index),
        OnClick::new(SelectSlotAction { index }),
        OnRightClick::new(SlotContextMenuAction { index }),
        DropTarget,
        OnDrop::new(DropToSlotAction {
            target_index: index,
        }),
        Tooltip::new(tooltip_text),
        InteractiveVisual,
    ));

    // Если есть предмет — добавляем Draggable и текст
    if let Some((name, _)) = item {
        slot.insert(Draggable);
        slot.with_children(|slot_content| {
            slot_content.spawn((
                Text::new(name.chars().next().unwrap().to_string()), // First letter
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
            ));
        });
    }
}

fn update_selection_display(
    selected: Res<SelectedSlot>,
    mut query: Query<&mut Text, With<SelectionText>>,
) {
    if selected.is_changed() {
        for mut text in &mut query {
            **text = match selected.0 {
                Some(idx) => format!("Selected: Slot {}", idx),
                None => "Selected: None".to_string(),
            };
        }
    }
}
