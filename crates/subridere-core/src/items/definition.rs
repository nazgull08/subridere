// items/definition.rs — Item definition (loaded from RON)

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::stats::{ModifierOp, ModifierTarget};

use super::flags::ItemFlags;
use super::slots::{AccessorySlot, ArmorSlot, EquipmentSlot, WeaponSlot};
use super::visual::ItemVisual;

/// Complete item definition — loaded from a single RON file
///
/// One file = complete truth about an item.
/// No data fragmentation, no hardcoded mappings.
#[derive(Asset, TypePath, Debug, Clone, Deserialize)]
pub struct ItemDefinition {
    /// Unique identifier (must match enum variant)
    pub id: String,

    /// Display name
    pub name: String,

    /// Description shown in UI
    #[serde(default)]
    pub description: String,

    /// Item category with type-specific data
    pub category: ItemCategory,

    /// Weight in units
    #[serde(default = "default_weight")]
    pub weight: f32,

    /// Base value in gold
    #[serde(default)]
    pub value: u32,

    /// Maximum stack size (1 = not stackable)
    #[serde(default = "default_stack")]
    pub max_stack: u32,

    /// Special flags
    #[serde(default)]
    pub flags: ItemFlags,

    /// Path to icon texture
    #[serde(default)]
    pub icon: String,

    /// Visual representation in world
    #[serde(default)]
    pub visual: ItemVisual,
}

fn default_weight() -> f32 {
    1.0
}

fn default_stack() -> u32 {
    1
}

impl ItemDefinition {
    /// Can this item be stacked?
    pub fn is_stackable(&self) -> bool {
        self.max_stack > 1
    }

    /// Can this item be equipped?
    pub fn is_equippable(&self) -> bool {
        matches!(
            self.category,
            ItemCategory::Weapon(_) | ItemCategory::Armor(_) | ItemCategory::Accessory(_)
        )
    }

    /// Check if this item can be equipped in the given slot
    pub fn can_equip_in(&self, target_slot: EquipmentSlot) -> bool {
        match &self.category {
            ItemCategory::Weapon(w) => w.slot.can_equip_in(target_slot),
            ItemCategory::Armor(a) => EquipmentSlot::from(a.slot) == target_slot,
            ItemCategory::Accessory(a) => EquipmentSlot::from(a.slot) == target_slot,
            _ => false,
        }
    }

    /// Get primary equipment slot if equippable (for UI display)
    /// For weapons, returns the primary slot (MainHand for OneHanded/TwoHanded)
    pub fn equipment_slot(&self) -> Option<EquipmentSlot> {
        match &self.category {
            ItemCategory::Weapon(w) => Some(w.slot.into()),
            ItemCategory::Armor(a) => Some(a.slot.into()),
            ItemCategory::Accessory(a) => Some(a.slot.into()),
            _ => None,
        }
    }

    /// Get weapon slot type display name (One-Handed, Two-Handed, etc.)
    /// Returns None for non-weapons
    pub fn weapon_slot_display(&self) -> Option<&'static str> {
        match &self.category {
            ItemCategory::Weapon(w) => Some(w.slot.display_name()),
            _ => None,
        }
    }

    /// Get damage if weapon
    pub fn damage(&self) -> Option<f32> {
        match &self.category {
            ItemCategory::Weapon(w) => Some(w.damage),
            _ => None,
        }
    }

    /// Get defense if armor
    pub fn defense(&self) -> Option<f32> {
        match &self.category {
            ItemCategory::Armor(a) => Some(a.defense),
            _ => None,
        }
    }
}

/// Item category — determines behavior and required slot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemCategory {
    Weapon(WeaponData),
    Armor(ArmorData),
    Accessory(AccessoryData),
    Consumable(ConsumableData),
    Misc,
}

/// Weapon-specific properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponData {
    /// Base damage
    pub damage: f32,

    /// Attack speed multiplier (1.0 = normal)
    #[serde(default = "default_speed")]
    pub speed: f32,

    /// Equipment slot type
    pub slot: WeaponSlot,

    /// Mana cost per attack (for magic weapons)
    #[serde(default)]
    pub mana_cost: f32,
}

fn default_speed() -> f32 {
    1.0
}

/// Armor-specific properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmorData {
    pub defense: f32,
    pub slot: ArmorSlot,
    #[serde(default)]
    pub magic_resist: f32,
    #[serde(default)]
    pub modifiers: Vec<(ModifierTarget, ModifierOp)>,
}

/// Accessory-specific properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessoryData {
    pub slot: AccessorySlot,
    #[serde(default)]
    pub modifiers: Vec<(ModifierTarget, ModifierOp)>,
}

/// Consumable-specific properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumableData {
    /// Effect when used
    pub effect: ConsumableEffect,
}

/// Effects for consumable items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsumableEffect {
    /// Restore health
    Heal(f32),

    /// Restore mana
    RestoreMana(f32),

    /// Restore stamina
    RestoreStamina(f32),
    // Buff (future)
    // Buff { stat: String, amount: f32, duration: f32 },
}
