// ui/inventory/layout.rs — Layout constants

use bevy::prelude::Color;

// === Inventory Grid ===
pub const GRID_COLS: usize = 5;
pub const GRID_ROWS: usize = 4;
pub const SLOT_SIZE: f32 = 64.0;
pub const SLOT_GAP: f32 = 4.0;

// === Equipment Panel ===
pub const EQUIP_SLOT_SIZE: f32 = 56.0;
pub const EQUIP_GAP: f32 = 4.0;

// === Colors ===
pub const SLOT_EMPTY: Color = Color::srgb(0.15, 0.15, 0.18);
pub const SLOT_FILLED: Color = Color::srgb(0.25, 0.25, 0.30);
pub const SLOT_BORDER: Color = Color::srgb(0.35, 0.35, 0.40);
pub const SLOT_BORDER_HOVER: Color = Color::srgb(0.6, 0.6, 0.65);
pub const SLOT_BORDER_DRAG: Color = Color::srgb(0.8, 0.7, 0.3);

pub const EQUIP_EMPTY: Color = Color::srgb(0.12, 0.12, 0.15);
pub const EQUIP_FILLED: Color = Color::srgb(0.22, 0.20, 0.18);
pub const EQUIP_BORDER: Color = Color::srgb(0.30, 0.28, 0.25);

pub const PANEL_BG: Color = Color::srgba(0.1, 0.1, 0.12, 0.95);
pub const PANEL_BORDER: Color = Color::srgb(0.3, 0.3, 0.35);

// === Text ===
pub const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
pub const TEXT_DIM: Color = Color::srgb(0.6, 0.6, 0.6);
