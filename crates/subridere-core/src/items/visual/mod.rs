// items/visual/mod.rs — Item visual representation

mod shape;
mod spawn;

pub use shape::{VisualPart, VisualShape};
pub use spawn::{spawn_item_visual, spawn_item_visual_with_colliders};

use serde::{Deserialize, Serialize};

/// How an item looks in the world
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum ItemVisual {
    /// No visual (invisible item)
    #[default]
    None,

    /// Built from primitive shapes (for prototyping)
    Primitive { parts: Vec<VisualPart> },

    /// 3D model file (future)
    Model { path: String },
}

impl ItemVisual {
    /// Check if item has any visual
    pub fn is_visible(&self) -> bool {
        !matches!(self, Self::None)
    }
}
