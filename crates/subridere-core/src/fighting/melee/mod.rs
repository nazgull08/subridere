// fighting/melee/mod.rs

pub mod damage;
pub mod intent;
pub mod state;

pub use damage::process_melee_collisions;
pub use intent::{AttackInputState, LeftAttackInput, RightAttackInput};
pub use state::{is_any_arm_active, is_arm_in_active_phase, process_combat_state};
