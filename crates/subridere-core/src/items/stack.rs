// items/stack.rs — Runtime item data

use serde::{Deserialize, Serialize};

use super::ItemId;

/// A stack of items in inventory
/// 
/// Morrowind-style: no random modifiers.
/// ItemId is enough to get all properties from registry.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemStack {
    /// Item type
    pub id: ItemId,

    /// Quantity (for stackable items)
    pub quantity: u32,
}

impl ItemStack {
    pub fn new(id: ItemId) -> Self {
        Self { id, quantity: 1 }
    }

    pub fn with_quantity(id: ItemId, quantity: u32) -> Self {
        Self { id, quantity }
    }

    /// Try to merge another stack into this one
    /// Returns remaining quantity that didn't fit
    pub fn try_merge(&mut self, other: &ItemStack, max_stack: u32) -> u32 {
        if self.id != other.id {
            return other.quantity; // Different items, nothing merged
        }

        let can_add = max_stack.saturating_sub(self.quantity);
        let will_add = can_add.min(other.quantity);

        self.quantity += will_add;
        other.quantity - will_add
    }

    /// Split off a portion of this stack
    /// Returns None if requested quantity exceeds available
    pub fn split(&mut self, amount: u32) -> Option<ItemStack> {
        if amount > self.quantity {
            return None;
        }

        self.quantity -= amount;
        Some(ItemStack {
            id: self.id,
            quantity: amount,
        })
    }
}
