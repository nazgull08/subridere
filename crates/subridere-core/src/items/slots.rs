// items/slots.rs — Equipment slots

use serde::{Deserialize, Serialize};

/// All possible equipment slots
///
/// separate slots for left/right pauldrons, gloves, boots
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum EquipmentSlot {
    // === Weapons ===
    MainHand,
    OffHand,

    // === Armor ===
    Helmet,
    Cuirass,
    LeftPauldron,
    RightPauldron,
    LeftGauntlet,
    RightGauntlet,
    Greaves,
    LeftBoot,
    RightBoot,

    // === Accessories ===
    Amulet,
    LeftRing,
    RightRing,
    Belt,
}

impl EquipmentSlot {
    /// All slots in display order
    pub fn all() -> &'static [EquipmentSlot] {
        &[
            Self::Helmet,
            Self::Amulet,
            Self::LeftPauldron,
            Self::RightPauldron,
            Self::Cuirass,
            Self::LeftGauntlet,
            Self::RightGauntlet,
            Self::LeftRing,
            Self::RightRing,
            Self::Belt,
            Self::Greaves,
            Self::LeftBoot,
            Self::RightBoot,
            Self::MainHand,
            Self::OffHand,
        ]
    }

    /// Human-readable name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::MainHand => "Main Hand",
            Self::OffHand => "Off Hand",
            Self::Helmet => "Helmet",
            Self::Cuirass => "Cuirass",
            Self::LeftPauldron => "Left Pauldron",
            Self::RightPauldron => "Right Pauldron",
            Self::LeftGauntlet => "Left Gauntlet",
            Self::RightGauntlet => "Right Gauntlet",
            Self::Greaves => "Greaves",
            Self::LeftBoot => "Left Boot",
            Self::RightBoot => "Right Boot",
            Self::Amulet => "Amulet",
            Self::LeftRing => "Left Ring",
            Self::RightRing => "Right Ring",
            Self::Belt => "Belt",
        }
    }

    /// Is this a weapon slot?
    pub fn is_weapon(&self) -> bool {
        matches!(self, Self::MainHand | Self::OffHand)
    }

    /// Is this an armor slot?
    pub fn is_armor(&self) -> bool {
        matches!(
            self,
            Self::Helmet
                | Self::Cuirass
                | Self::LeftPauldron
                | Self::RightPauldron
                | Self::LeftGauntlet
                | Self::RightGauntlet
                | Self::Greaves
                | Self::LeftBoot
                | Self::RightBoot
        )
    }

    /// Is this an accessory slot?
    pub fn is_accessory(&self) -> bool {
        matches!(
            self,
            Self::Amulet | Self::LeftRing | Self::RightRing | Self::Belt
        )
    }
}

/// Weapon-specific slots (for WeaponData in RON)
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum WeaponSlot {
    MainHand,
    OffHand,
    TwoHanded, // Takes both MainHand and OffHand
}

impl From<WeaponSlot> for EquipmentSlot {
    fn from(slot: WeaponSlot) -> Self {
        match slot {
            WeaponSlot::MainHand => EquipmentSlot::MainHand,
            WeaponSlot::OffHand => EquipmentSlot::OffHand,
            WeaponSlot::TwoHanded => EquipmentSlot::MainHand, // Primary slot
        }
    }
}

/// Armor-specific slots (for ArmorData in RON)
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ArmorSlot {
    Helmet,
    Cuirass,
    LeftPauldron,
    RightPauldron,
    LeftGauntlet,
    RightGauntlet,
    Greaves,
    LeftBoot,
    RightBoot,
}

impl From<ArmorSlot> for EquipmentSlot {
    fn from(slot: ArmorSlot) -> Self {
        match slot {
            ArmorSlot::Helmet => EquipmentSlot::Helmet,
            ArmorSlot::Cuirass => EquipmentSlot::Cuirass,
            ArmorSlot::LeftPauldron => EquipmentSlot::LeftPauldron,
            ArmorSlot::RightPauldron => EquipmentSlot::RightPauldron,
            ArmorSlot::LeftGauntlet => EquipmentSlot::LeftGauntlet,
            ArmorSlot::RightGauntlet => EquipmentSlot::RightGauntlet,
            ArmorSlot::Greaves => EquipmentSlot::Greaves,
            ArmorSlot::LeftBoot => EquipmentSlot::LeftBoot,
            ArmorSlot::RightBoot => EquipmentSlot::RightBoot,
        }
    }
}

/// Accessory-specific slots
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum AccessorySlot {
    Amulet,
    LeftRing,
    RightRing,
    Belt,
}

impl From<AccessorySlot> for EquipmentSlot {
    fn from(slot: AccessorySlot) -> Self {
        match slot {
            AccessorySlot::Amulet => EquipmentSlot::Amulet,
            AccessorySlot::LeftRing => EquipmentSlot::LeftRing,
            AccessorySlot::RightRing => EquipmentSlot::RightRing,
            AccessorySlot::Belt => EquipmentSlot::Belt,
        }
    }
}
