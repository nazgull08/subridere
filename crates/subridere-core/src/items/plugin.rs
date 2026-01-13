// items/plugin.rs — Items system plugin

use bevy::prelude::*;

use super::definition::ItemDefinition;
use super::registry::{ItemRegistry, load_item_registry};

/// Plugin that sets up the items system
pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register asset type
            .init_asset::<ItemDefinition>()
            .init_asset_loader::<ItemDefinitionLoader>()
            // Initialize registry
            .init_resource::<ItemRegistry>()
            // Load items
            .add_systems(Update, load_item_registry);

        info!("✅ Items plugin initialized");
    }
}

/// Asset loader for ItemDefinition
use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, LoadContext};

#[derive(Default)]
pub struct ItemDefinitionLoader;

impl AssetLoader for ItemDefinitionLoader {
    type Asset = ItemDefinition;
    type Settings = ();
    type Error = ron::error::SpannedError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await.unwrap();
        let definition: ItemDefinition = ron::de::from_bytes(&bytes)?;
        Ok(definition)
    }

    fn extensions(&self) -> &[&str] {
        &["item.ron"]
    }
}
