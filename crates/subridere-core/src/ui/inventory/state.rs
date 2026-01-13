// ui/inventory/state.rs — Inventory UI state

use bevy::prelude::*;

/// State for inventory UI visibility
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InventoryState {
    #[default]
    Closed,
    Open,
}

/// Run condition: inventory is closed (for game input)
pub fn inventory_closed(state: Res<State<InventoryState>>) -> bool {
    *state.get() == InventoryState::Closed
}

/// Run condition: inventory is open
pub fn inventory_open(state: Res<State<InventoryState>>) -> bool {
    *state.get() == InventoryState::Open
}
