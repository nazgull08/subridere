pub mod data;
pub mod layout;
pub mod plugin;
pub mod state;
pub mod systems;

// Re-exports
pub use plugin::InventoryPlugin;
pub use state::InventoryState;
pub use systems::inventory_closed;
pub use layout::*;
