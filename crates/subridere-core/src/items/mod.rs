// items/mod.rs — New item system

pub mod definition;
pub mod flags;
pub mod plugin;
pub mod registry;
pub mod slots;
pub mod stack;
pub mod visual;
pub mod world;

mod generated {
    include!(concat!(env!("OUT_DIR"), "/item_ids.rs"));
}

// Re-exports
pub use definition::{ArmorData, ConsumableData, ItemCategory, ItemDefinition, WeaponData};
pub use flags::ItemFlags;
pub use generated::ItemId;
pub use plugin::ItemsPlugin;
pub use registry::{ItemRegistry, registry_loaded};
pub use slots::EquipmentSlot;
pub use stack::ItemStack;
pub use visual::{ItemVisual, VisualPart, VisualShape};
pub use world::{Pickupable, Targeted, WorldItem, spawn_world_item};
