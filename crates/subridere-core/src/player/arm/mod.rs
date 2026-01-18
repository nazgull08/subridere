// crates/subridere-core/src/player/arm/mod.rs

pub mod components;
pub mod debug;
pub mod ik_system;
pub mod plugin;
pub mod spawn;
pub mod weapon_visual;

pub use components::*;
pub use debug::ArmDebugState;
pub use plugin::PlayerArmPlugin;
pub use spawn::spawn_player_arms;
pub use weapon_visual::{
    EquippedWeaponVisual, WeaponDebugState, apply_weapon_debug_transform,
    sync_equipped_weapon_visual, weapon_debug_input,
};
