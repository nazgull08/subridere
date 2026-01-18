// crates/subridere-core/src/player/mod.rs

pub mod arm;
pub mod body;
pub mod component;
pub mod plugin;

pub use arm::{MeleeHitbox, PlayerArmPlugin, spawn_player_arms};
pub use component::{Player, PlayerVisual};
pub use plugin::PlayerPlugin;
