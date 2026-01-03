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
            .init_resource::<SelectedSlot>()
            .init_resource::<ContextMenuState>()
            // Input system (runs every frame)
            .add_systems(Update, toggle_inventory_input)
            // Cursor management (runs when state changes)
            .add_systems(OnEnter(InventoryState::Open), manage_cursor_on_inventory)
            .add_systems(OnExit(InventoryState::Open), manage_cursor_on_inventory)
            // UI spawn/despawn
            .add_systems(OnEnter(InventoryState::Open), spawn_inventory_ui)
            .add_systems(OnExit(InventoryState::Open), despawn_inventory_ui)
            // Force close menu when inventory exits (safety net)
            .add_systems(
                OnExit(InventoryState::Open),
                (
                    force_close_menu_on_inventory_exit,  // 1. Close menu state
                    despawn_context_menu,                 // 2. Despawn UI
                ).chain()
            )
            // Sync and interaction (only when inventory is open)
            .add_systems(
                Update,
                (
                    // Sync systems can run in any order (parallel)
                    sync_inventory_to_ui,
                    sync_stats_to_ui,
                    sync_equipment_to_ui,
                    handle_slot_hover,
                    handle_equip_slot_hover,
                    // Critical order: clicks → actions → visual
                    (
                        handle_slot_click,
                        handle_equip_slot_click,
                        process_item_actions,
                        update_selected_slot_visual,
                    ).chain(),
                    // Right-click detection (changes menu state)
                    detect_inventory_right_click,
                    detect_equipment_right_click,
                    close_menu_on_outside_click,
                    // Context menu lifecycle (responds to state changes)
                    (
                        despawn_context_menu,      // 1. Despawn if state changed to closed
                        spawn_context_menu,         // 2. Spawn if state changed to open
                        handle_menu_button_hover,   // 3. Handle hover effects
                        handle_menu_button_clicks,  // 4. Handle button clicks
                    ).chain(),  // ← Force exact order for menu lifecycle!
                )
                    .run_if(in_state(InventoryState::Open)),
            );

        info!("✅ Inventory plugin initialized");
    }
}
