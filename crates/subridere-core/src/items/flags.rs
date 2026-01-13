// items/flags.rs — Item flags (immutable properties)

use serde::{Deserialize, Serialize};

/// Flags that define special item behaviors
///
/// These are part of the item definition and cannot be changed at runtime.
/// no random modifiers, each item is unique by definition.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct ItemFlags {
    /// Cannot be dropped or sold (quest items)
    #[serde(default)]
    pub quest_item: bool,

    /// Cannot be unequipped without Remove Curse spell
    #[serde(default)]
    pub cursed: bool,

    /// Only one can exist in the game world
    #[serde(default)]
    pub unique: bool,

    /// Item was stolen (for crime system)
    #[serde(default)]
    pub stolen: bool,
}

impl ItemFlags {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn quest_item(mut self) -> Self {
        self.quest_item = true;
        self
    }

    pub fn cursed(mut self) -> Self {
        self.cursed = true;
        self
    }

    pub fn unique(mut self) -> Self {
        self.unique = true;
        self
    }
}
