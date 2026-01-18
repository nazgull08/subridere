// fighting/mod.rs

pub mod components;
pub mod events;
pub mod melee;
pub mod plugin;
pub mod weapon;

pub use components::*;
pub use plugin::CombatPlugin;
pub use weapon::{arm_to_slot, get_weapon_kind};
