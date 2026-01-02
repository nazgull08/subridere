use bevy::prelude::Color;

/// Size of a single inventory slot (square)
pub const SLOT_SIZE: f32 = 64.0;

/// Gap between slots
pub const SLOT_GAP: f32 = 4.0;

/// Inventory grid dimensions
pub const GRID_COLS: usize = 5;
pub const GRID_ROWS: usize = 4;

/// Colors
pub const SLOT_EMPTY_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);
pub const SLOT_FILLED_COLOR: Color = Color::srgb(0.3, 0.3, 0.4);
pub const SLOT_BORDER_COLOR: Color = Color::srgb(0.4, 0.4, 0.4);
