pub mod input;
pub mod interaction;  // ДОБАВИТЬ
pub mod sync;
pub mod ui;

// Re-exports
pub use input::{toggle_inventory_input, manage_cursor_on_inventory, inventory_closed};
pub use interaction::{SelectedSlot, handle_slot_hover, handle_slot_click, update_selected_slot_visual};  // ДОБАВИТЬ
pub use sync::sync_inventory_to_ui;
pub use ui::{spawn_inventory_ui, despawn_inventory_ui, InventoryUI, SlotIcon, SlotQuantity};
