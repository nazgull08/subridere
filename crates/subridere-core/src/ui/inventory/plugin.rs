use super::state::InventoryState;
use super::systems::*;
use bevy::prelude::*;

pub struct UiInventoryPlugin;

impl Plugin for UiInventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register state
            .init_state::<InventoryState>()
            // Register resources
            .init_resource::<SelectedSlot>() // ДОБАВИТЬ
            // Input system (runs every frame)
            .add_systems(Update, toggle_inventory_input)
            // Cursor management (runs when state changes)
            .add_systems(OnEnter(InventoryState::Open), manage_cursor_on_inventory)
            .add_systems(OnExit(InventoryState::Open), manage_cursor_on_inventory)
            // UI spawn/despawn
            .add_systems(OnEnter(InventoryState::Open), spawn_inventory_ui)
            .add_systems(OnExit(InventoryState::Open), despawn_inventory_ui)
            // Sync and interaction (only when inventory is open)
            .add_systems(
                Update,
                (
                    sync_inventory_to_ui,
                    sync_stats_to_ui,
                    sync_equipment_to_ui,
                    handle_slot_hover,
                    handle_slot_click,
                    handle_equip_slot_click,
                    handle_equip_slot_hover,
                    update_selected_slot_visual,
                )
                    .run_if(in_state(InventoryState::Open)),
            );

        info!("✅ Inventory plugin initialized");
    }
}
