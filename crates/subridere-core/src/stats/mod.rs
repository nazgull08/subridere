pub mod attributes;
pub mod computed;
pub mod damage;
pub mod formulas;
pub mod health;
pub mod level;
pub mod mana;
pub mod modifiers;
pub mod plugin;
pub mod recalculate;
pub mod stamina;

// Re-exports
pub use attributes::{AttributeType, Attributes};
pub use computed::ComputedStats;
pub use damage::{Damage, DamageType};
pub use health::Health;
pub use level::{Experience, ExperienceGainEvent, ExperienceReward, Level, LevelUpEvent}; // <-- events added
pub use mana::Mana;
pub use modifiers::{ModifierOp, ModifierSource, ModifierTarget, StatModifiers};
pub use plugin::{SimpleStatsBundle, StatsBundle, StatsPlugin}; // <-- bundles added
pub use stamina::Stamina;
