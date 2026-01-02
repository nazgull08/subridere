use bevy::prelude::*;
use super::definition::ItemDefinition;

/// Runtime component for an item instance in the game
///
/// This is attached to entities that represent actual items,
/// whether they're in the world, inventory, or equipped.
#[derive(Component, Debug)]
pub struct Item {
    /// Reference to the immutable definition (shared data)
    pub definition: Handle<ItemDefinition>,

    /// How many of this item (for stackables)
    pub quantity: u32,
}

/// Marker component for items that can be picked up from the world
#[derive(Component, Debug)]
pub struct Pickupable;

/// Component for items currently in an inventory slot
#[derive(Component, Debug)]
pub struct InInventory {
    /// Which slot in the inventory (0-based index)
    pub slot: usize,
}

/// Component for items that are currently equipped
#[derive(Component, Debug)]
pub struct Equipped {
    /// Which equipment slot this item occupies
    pub slot: EquipSlot,
}

/// Equipment slot types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EquipSlot {
    /// Main hand (primary weapon)
    MainHand,

    /// Off hand (shield, secondary weapon, etc.)
    OffHand,

    // Future slots can be added here:
    // Head, Chest, Legs, Feet, Ring1, Ring2, etc.
}

