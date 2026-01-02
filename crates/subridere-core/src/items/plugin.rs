use crate::items::visual::definition::VisualDefinition;
use crate::items::visual::loader::VisualDefinitionLoader;
use bevy::prelude::*;

use super::definition::ItemDefinition;
use super::loader::ItemDefinitionLoader;
/// Plugin that sets up the items system
///
/// Registers:
/// - ItemDefinition asset type
/// - ItemDefinitionLoader for .item.ron files
/// - VisualDefinition asset type
/// - VisualDefinitionLoader for .visual.ron files
pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        // Register asset loaders
        app.init_asset::<ItemDefinition>()
            .init_asset_loader::<ItemDefinitionLoader>()
            .init_asset::<VisualDefinition>()
            .init_asset_loader::<VisualDefinitionLoader>();

        info!("✅ Items plugin initialized");

        // TODO: Add systems for item spawning, pickup, etc.
    }
}
