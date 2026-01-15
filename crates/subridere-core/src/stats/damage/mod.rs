pub mod component;
pub mod system;

pub use component::{Damage, DamageType, HasDealtDamage};
pub use system::apply_damage;
