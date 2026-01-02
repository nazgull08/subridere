use super::shape::VisualPart;
use bevy::prelude::*;
use serde::Deserialize;

/// Visual definition loaded from RON files
///
/// Describes how an item looks using primitive shapes.
/// These are loaded as Bevy assets from .visual.ron files.
#[derive(Asset, TypePath, Deserialize, Clone, Debug)]
pub struct VisualDefinition {
    /// Unique identifier for this visual
    pub id: String,

    /// List of visual parts that compose this item's appearance
    pub parts: Vec<VisualPart>,
}
