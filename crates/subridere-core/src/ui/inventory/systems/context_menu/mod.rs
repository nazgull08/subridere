pub mod actions;
pub mod state;
pub mod ui;

// Re-export everything needed externally
pub use state::{ContextMenu, ContextMenuState};
pub use ui::{
    despawn_context_menu, force_close_menu_on_inventory_exit,
    spawn_context_menu,
};
