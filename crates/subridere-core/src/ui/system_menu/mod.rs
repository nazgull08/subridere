pub mod components;
pub mod plugin;
pub mod spawn;
pub mod state;

pub use plugin::SystemMenuPlugin;
pub use state::{SystemMenuState, system_menu_closed, system_menu_open};
