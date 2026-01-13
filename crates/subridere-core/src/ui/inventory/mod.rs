// ui/inventory/mod.rs — Inventory UI with bevy_ui_actions

pub mod actions;
pub mod components;
pub mod layout;
pub mod plugin;
pub mod spawn;
pub mod state;
pub mod sync;

// Re-exports
pub use plugin::UiInventoryPlugin;
pub use state::{InventoryState, inventory_closed};
