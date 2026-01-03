use bevy::prelude::*;
use super::state::*;

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
            // Determine which buttons to show based on what was clicked
            let show_equip = menu_state.inventory_slot.is_some();
            let show_unequip = menu_state.equipment_slot.is_some();

            // Equip button (only for inventory items)
            if show_equip {
                spawn_menu_button(menu, "Equip", EquipButton, &font);
            }

            // Unequip button (only for equipped items)
            if show_unequip {
                spawn_menu_button(menu, "Unequip", EquipButton, &font);
            }

            // Drop button (always available)
            spawn_menu_button(menu, "Drop", DropButton, &font);

            // Cancel button (always available)
            spawn_menu_button(menu, "Cancel", CancelButton, &font);
        });

    info!("📋 Context menu spawned at {:?}", cursor_pos);
}

/// Helper to spawn a menu button
fn spawn_menu_button(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    marker: impl Component,
    font: &Handle<Font>,
) {
    parent
        .spawn((
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
            Interaction::default(),
            marker,
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

/// Handle hover effect on menu buttons
pub fn handle_menu_button_hover(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, Or<(With<EquipButton>, With<DropButton>, With<CancelButton>)>),
    >,
) {
    for (interaction, mut bg_color) in &mut button_query {
        match *interaction {
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgb(0.4, 0.4, 0.4));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::srgb(0.25, 0.25, 0.25));
            }
            Interaction::Pressed => {
                *bg_color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
            }
        }
    }
}

/// Force close menu when inventory is closing
pub fn force_close_menu_on_inventory_exit(
    mut menu_state: ResMut<ContextMenuState>,
) {
    if menu_state.is_open {
        info!("📋 Force closing menu (inventory closing)");
        menu_state.close();
    }
}
