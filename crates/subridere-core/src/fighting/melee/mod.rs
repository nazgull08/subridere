// fighting/melee/mod.rs

pub mod damage;
pub mod debug;
pub mod intent;
pub mod state;

pub use damage::process_melee_collisions;
pub use debug::track_item_physics;
pub use intent::MeleeAttackIntent;
pub use state::{is_in_active_phase, process_combat_state};
