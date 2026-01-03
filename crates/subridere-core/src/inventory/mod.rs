pub mod component;
pub mod plugin;
pub mod systems;

// Re-exports
pub use component::{Equipment, Inventory, InventorySlot};
pub use plugin::InventoryPlugin;
