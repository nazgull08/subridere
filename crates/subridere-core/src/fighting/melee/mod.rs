pub mod damage;
pub mod intent;
pub mod state;

pub use damage::apply_melee_damage;
pub use intent::MeleeAttackIntent;
pub use state::process_combat_state;
