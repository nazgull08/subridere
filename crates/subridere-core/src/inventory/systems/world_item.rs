// inventory/systems/world_item.rs — World item components

use bevy::prelude::*;

use crate::items::ItemId;

/// An item entity that exists in the game world
/// 
/// Can be picked up by player.
#[derive(Component, Clone, Debug)]
pub struct WorldItem {
    /// Item type
    pub id: ItemId,

    /// Quantity (for stackable items)
    pub quantity: u32,
}

impl WorldItem {
    pub fn new(id: ItemId) -> Self {
        Self { id, quantity: 1 }
    }

    pub fn with_quantity(id: ItemId, quantity: u32) -> Self {
        Self { id, quantity }
    }
}

/// Marker: this entity can be picked up
#[derive(Component, Default)]
pub struct Pickupable;

/// Marker: this item is currently highlighted/targeted by player
#[derive(Component)]
pub struct Targeted;
