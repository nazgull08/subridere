use bevy::prelude::*;
use serde::Deserialize;

/// Immutable item template loaded from RON files
///
/// This represents the shared definition that all instances
/// of an item type reference (like "Wooden Staff" blueprint).
#[derive(Asset, TypePath, Deserialize, Clone, Debug)]
pub struct ItemDefinition {
    /// Unique identifier (e.g. "wooden_staff")
    pub id: String,

    /// Human-readable name for UI
    pub name: String,

    /// Reference to visual representation
    /// For now just a string, later can be a proper asset handle
    pub visual_ref: String,

    pub icon: String,

    /// Type-specific properties
    pub properties: ItemProperties,
}

/// What kind of item this is and its specific stats
#[derive(Deserialize, Clone, Debug)]
pub enum ItemProperties {
    /// Weapons that can deal damage
    Weapon(WeaponProperties),

    /// Items that can be consumed (potions, food, etc.)
    Consumable(ConsumableProperties),
}

/// Properties specific to weapons
#[derive(Deserialize, Clone, Debug)]
pub struct WeaponProperties {
    /// Base damage dealt by this weapon
    pub damage: f32,

    /// How fast this weapon attacks (attacks per second)
    pub attack_speed: f32,

    /// Mana cost per attack
    pub mana_cost: f32,
}

/// Properties specific to consumable items
#[derive(Deserialize, Clone, Debug)]
pub struct ConsumableProperties {
    /// Maximum items that can stack in one slot
    pub max_stack: u32,
    // TODO: Add effect system later
}
