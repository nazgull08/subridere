pub mod input;
pub mod interaction; // ДОБАВИТЬ
pub mod sync;
pub mod ui;

// Re-exports
pub use input::{inventory_closed, manage_cursor_on_inventory, toggle_inventory_input};
pub use interaction::{
    SelectedSlot, handle_slot_click, handle_slot_hover, update_selected_slot_visual,
}; // ДОБАВИТЬ
pub use sync::sync_inventory_to_ui;
pub use ui::{InventoryUI, SlotIcon, SlotQuantity, despawn_inventory_ui, spawn_inventory_ui};
