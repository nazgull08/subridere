// ui/inventory/plugin.rs — Inventory UI plugin

use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_ui_actions::UiActionsPlugin;

use super::spawn::{despawn_inventory_ui, spawn_inventory_ui};
use super::state::{InventoryState, inventory_open};
use super::sync::{
    sync_drag_visual, sync_equipment_slots, sync_inventory_slots, sync_stats_display,
};

pub struct UiInventoryPlugin;

impl Plugin for UiInventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            // Depends on UiActionsPlugin
            .add_plugins(UiActionsPlugin)
            // State
            .init_state::<InventoryState>()
            // Input (always runs)
            .add_systems(Update, toggle_inventory_input)
            // Cursor management
            .add_systems(OnEnter(InventoryState::Open), show_cursor)
            .add_systems(OnExit(InventoryState::Open), hide_cursor)
            // UI lifecycle
            .add_systems(OnEnter(InventoryState::Open), spawn_inventory_ui)
            .add_systems(OnExit(InventoryState::Open), despawn_inventory_ui)
            // Sync systems (only when open)
            .add_systems(
                Update,
                (
                    sync_inventory_slots,
                    sync_equipment_slots,
                    sync_drag_visual,
                    sync_stats_display,
                )
                    .chain()
                    .run_if(inventory_open),
            );

        info!("✅ UI Inventory plugin initialized");
    }
}

/// Toggle inventory on Tab/I, close on Escape
fn toggle_inventory_input(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<InventoryState>>,
    mut next_state: ResMut<NextState<InventoryState>>,
) {
    // Toggle on Tab or I
    if keys.just_pressed(KeyCode::Tab) || keys.just_pressed(KeyCode::KeyI) {
        match state.get() {
            InventoryState::Closed => {
                info!("📦 Opening inventory");
                next_state.set(InventoryState::Open);
            }
            InventoryState::Open => {
                info!("📦 Closing inventory");
                next_state.set(InventoryState::Closed);
            }
        }
    }

    // Escape closes
    if keys.just_pressed(KeyCode::Escape) && *state.get() == InventoryState::Open {
        info!("📦 Closing inventory (ESC)");
        next_state.set(InventoryState::Closed);
    }
}

/// Show cursor when inventory opens
fn show_cursor(mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.single_mut() {
        window.cursor_options.grab_mode = CursorGrabMode::None;
        window.cursor_options.visible = true;
    }
}

/// Hide cursor when inventory closes
fn hide_cursor(mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.single_mut() {
        window.cursor_options.grab_mode = CursorGrabMode::Confined;
        window.cursor_options.visible = false;
    }
}
