// inventory/mod.rs — Inventory system

pub mod component;
pub mod plugin;
pub mod systems;

pub use component::{Equipment, Inventory};
pub use plugin::InventoryPlugin;
