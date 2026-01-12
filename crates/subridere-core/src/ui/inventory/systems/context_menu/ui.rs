use super::state::*;
use super::actions::*;
use bevy::prelude::*;
use bevy_ui_actions::prelude::*;

/// Spawn context menu when state changes to open
pub fn spawn_context_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    menu_state: Res<ContextMenuState>,
    existing_menu: Query<Entity, With<ContextMenu>>,
) {
    // Only spawn when state JUST changed to open
    if !menu_state.is_changed() {
        return;
    }

    // If menu should be closed, don't spawn
    if !menu_state.is_open {
        return;
    }

    // If menu already exists, don't spawn another
    if !existing_menu.is_empty() {
        return;
    }

    // Use the saved spawn position (not current cursor!)
    let cursor_pos = menu_state.spawn_position;

    let font = asset_server.load("fonts/dogica.ttf");

    // Spawn menu container
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(cursor_pos.x),
                top: Val::Px(cursor_pos.y),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(8.0)),
                row_gap: Val::Px(4.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            BorderColor(Color::srgb(0.6, 0.6, 0.6)),
            ContextMenu,
            Name::new("ContextMenu"),
        ))
        .with_children(|menu| {
            // Equip button (only for inventory items)
            if let Some(slot_index) = menu_state.inventory_slot {
                spawn_menu_button(
                    menu,
                    "Equip",
                    EquipItemAction { slot_index },
                    &font,
                );
            }

            // Unequip button (only for equipped items) — TODO: UnequipAction
            // if let Some(slot_type) = menu_state.equipment_slot {
            //     spawn_menu_button(menu, "Unequip", UnequipAction { slot_type }, &font);
            // }

            // Drop button (context-aware)
            if let Some(slot_index) = menu_state.inventory_slot {
                spawn_menu_button(
                    menu,
                    "Drop",
                    DropFromInventoryAction { slot_index },
                    &font,
                );
            } else if let Some(slot_type) = menu_state.equipment_slot {
                spawn_menu_button(
                    menu,
                    "Drop",
                    DropFromEquipmentAction { slot_type },
                    &font,
                );
            }

            // Cancel button (always available)
            spawn_menu_button(menu, "Cancel", CloseMenuAction, &font);
        });

    info!("📋 Context menu spawned at {:?}", cursor_pos);
}

/// Helper to spawn a menu button with ActionButton
fn spawn_menu_button(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    action: impl UiAction,
    font: &Handle<Font>,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(120.0),
                height: Val::Px(32.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
            BorderColor(Color::srgb(0.4, 0.4, 0.4)),
            ActionButton::new(action),
            Name::new(format!("{}Button", label)),
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font: font.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

/// Despawn context menu when state changes to closed
pub fn despawn_context_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<ContextMenu>>,
    menu_state: Res<ContextMenuState>,
) {
    // Only despawn when state JUST changed to closed
    if !menu_state.is_changed() {
        return;
    }

    // If menu is still open, don't despawn
    if menu_state.is_open {
        return;
    }

    // Despawn all menu entities (should only be one)
    for entity in &menu_query {
        commands.entity(entity).despawn();
        info!("📋 Context menu despawned");
    }
}

/// Force close menu when inventory is closing
pub fn force_close_menu_on_inventory_exit(mut menu_state: ResMut<ContextMenuState>) {
    if menu_state.is_open {
        info!("📋 Force closing menu (inventory closing)");
        menu_state.close();
    }
}
