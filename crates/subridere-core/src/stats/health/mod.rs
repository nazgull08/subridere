pub mod component;
pub mod system;

pub use component::Health;
pub use system::{check_player_death, regenerate_health}; // ← ДОБАВИТЬ check_player_death
