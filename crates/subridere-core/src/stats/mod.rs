pub mod attributes;
pub mod computed;
pub mod damage;
pub mod formulas;
pub mod health;
pub mod mana;
pub mod modifiers;
pub mod plugin;
pub mod level;
pub mod recalculate;
pub mod stamina;

// Re-exports для удобства
pub use attributes::{AttributeType, Attributes};
pub use computed::ComputedStats;
pub use damage::{Damage, DamageType};
pub use health::Health;
pub use mana::Mana;
pub use modifiers::{ModifierOp, ModifierSource, ModifierTarget, StatModifiers};
pub use stamina::Stamina;
pub use level::{Experience, ExperienceReward, Level};
