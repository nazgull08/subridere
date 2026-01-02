pub mod component;
pub mod plugin;
pub mod systems;

// Re-exports
pub use component::{Equipment, Inventory, InventorySlot};
pub use plugin::InventoryPlugin;
pub use systems::{TargetedItem, detect_pickupable_items, handle_pickup_input};
