use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, LoadContext};
use bevy::prelude::*;
use thiserror::Error;

use super::definition::ItemDefinition;

/// Asset loader for ItemDefinition files (.item.ron)
#[derive(Default)]
pub struct ItemDefinitionLoader;

/// Errors that can occur when loading item definitions
#[derive(Debug, Error)]
pub enum ItemLoaderError {
    /// IO error while reading the file
    #[error("Failed to read item file: {0}")]
    Io(#[from] std::io::Error),

    /// RON deserialization error
    #[error("Failed to parse RON: {0}")]
    RonError(#[from] ron::error::SpannedError),

    /// UTF-8 conversion error
    #[error("Invalid UTF-8 in file: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

impl AssetLoader for ItemDefinitionLoader {
    type Asset = ItemDefinition;
    type Settings = ();
    type Error = ItemLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        // Read file contents
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        // Convert to string
        let content = String::from_utf8(bytes)?;

        // Parse RON
        let definition: ItemDefinition = ron::from_str(&content)?;

        info!("✅ Loaded item: {} ({})", definition.name, definition.id);

        Ok(definition)
    }

    fn extensions(&self) -> &[&str] {
        // Load files with .item.ron extension
        &["item.ron"]
    }
}
