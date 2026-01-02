use bevy::prelude::*;

/// State for inventory UI visibility
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InventoryState {
    #[default]
    Closed,
    Open,
}
