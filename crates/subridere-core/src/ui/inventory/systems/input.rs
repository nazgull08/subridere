use super::super::state::InventoryState;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;

/// Toggle inventory on Tab or I key
pub fn toggle_inventory_input(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<InventoryState>>,
    mut next_state: ResMut<NextState<InventoryState>>,
) {
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

    // ESC also closes inventory
    if keys.just_pressed(KeyCode::Escape) && *state.get() == InventoryState::Open {
        info!("📦 Closing inventory (ESC)");
        next_state.set(InventoryState::Closed);
    }
}

/// Show cursor when inventory is open
pub fn manage_cursor_on_inventory(
    state: Res<State<InventoryState>>,
    mut windows: Query<&mut Window>,
) {
    let Ok(mut window) = windows.single_mut() else {
        return;
    };

    match state.get() {
        InventoryState::Open => {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
        }
        InventoryState::Closed => {
            window.cursor_options.grab_mode = CursorGrabMode::Confined;
            window.cursor_options.visible = false;
        }
    }
}

pub fn inventory_closed(state: Res<State<InventoryState>>) -> bool {
    *state.get() == InventoryState::Closed
}
