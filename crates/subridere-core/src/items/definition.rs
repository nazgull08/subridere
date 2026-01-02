use bevy::prelude::*;
use serde::Deserialize;

#[derive(Asset, TypePath, Deserialize, Clone, Debug)]
pub struct ItemDefinition {
    pub id: String,
    pub name: String,
    pub visual_ref: String,
    pub icon: String,
    pub properties: ItemProperties,
}

#[derive(Deserialize, Clone, Debug)]
pub enum ItemProperties {
    Weapon(WeaponProperties),
    Armor(ArmorProperties),
    Consumable(ConsumableProperties),
}

#[derive(Deserialize, Clone, Debug)]
pub struct WeaponProperties {
    pub damage: f32,
    pub attack_speed: f32,
    pub mana_cost: f32,
}

/// Properties for armor items
#[derive(Deserialize, Clone, Debug)]
pub struct ArmorProperties {
    /// Which slot this armor goes in
    pub slot: ArmorSlot,

    /// Defense rating
    pub defense: f32,

    /// Weight (optional, for later)
    pub weight: f32,
}

/// Equipment slots for armor (Morrowind style)
#[derive(Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ArmorSlot {
    Helmet,
    LeftPauldron,
    RightPauldron,
    Chest,
    LeftGlove,
    RightGlove,
    Greaves,
    LeftBoot,
    RightBoot,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ConsumableProperties {
    pub max_stack: u32,
}
