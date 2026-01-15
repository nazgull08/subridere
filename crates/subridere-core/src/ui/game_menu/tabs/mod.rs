pub mod character;
pub mod inventory;
pub mod journal;
pub mod map;

pub use character::spawn_character_tab;
pub use inventory::spawn_inventory_content;
pub use journal::spawn_journal_tab;
pub use map::spawn_map_tab;
