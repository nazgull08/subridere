// items/registry.rs — Central item storage

use bevy::prelude::*;
use std::collections::HashMap;

use super::ItemId;
use super::definition::ItemDefinition;

/// Central registry for all item definitions
///
/// Loaded once at startup. Provides O(1) access to item data.
#[derive(Resource, Default)]
pub struct ItemRegistry {
    /// Loaded definitions
    definitions: HashMap<ItemId, ItemDefinition>,

    /// Icon handles
    icons: HashMap<ItemId, Handle<Image>>,

    /// Is loading complete?
    loaded: bool,
}

impl ItemRegistry {
    /// Check if registry is fully loaded
    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    /// Mark registry as loaded
    pub fn mark_loaded(&mut self) {
        self.loaded = true;
        info!("📦 ItemRegistry loaded: {} items", self.definitions.len());
    }

    /// Register an item definition
    pub fn register(&mut self, id: ItemId, definition: ItemDefinition) {
        self.definitions.insert(id, definition);
    }

    /// Register an icon
    pub fn register_icon(&mut self, id: ItemId, handle: Handle<Image>) {
        self.icons.insert(id, handle);
    }

    /// Get item definition
    ///
    /// # Panics
    /// Panics if item not found (should never happen after loading)
    pub fn get(&self, id: ItemId) -> &ItemDefinition {
        self.definitions
            .get(&id)
            .unwrap_or_else(|| panic!("Item {:?} not found in registry", id))
    }

    /// Try to get item definition
    pub fn try_get(&self, id: ItemId) -> Option<&ItemDefinition> {
        self.definitions.get(&id)
    }

    /// Get icon handle
    pub fn icon(&self, id: ItemId) -> Option<&Handle<Image>> {
        self.icons.get(&id)
    }

    /// Iterate over all items
    pub fn iter(&self) -> impl Iterator<Item = (ItemId, &ItemDefinition)> {
        self.definitions.iter().map(|(&id, def)| (id, def))
    }

    /// Number of registered items
    pub fn len(&self) -> usize {
        self.definitions.len()
    }

    /// Is registry empty?
    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }
}

/// System to load all items into registry
pub fn load_item_registry(
    mut registry: ResMut<ItemRegistry>,
    asset_server: Res<AssetServer>,
    mut item_assets: ResMut<Assets<ItemDefinition>>,
    mut pending_handles: Local<Vec<(ItemId, Handle<ItemDefinition>)>>,
    mut icons_loading: Local<Vec<(ItemId, Handle<Image>)>>,
    mut started: Local<bool>,
) {
    // Start loading on first run
    if !*started {
        *started = true;
        info!("📦 Starting item registry load...");

        for id in ItemId::all() {
            // Load definition
            let path = format!("items/{}.item.ron", id.as_str());
            let handle: Handle<ItemDefinition> = asset_server.load(&path);
            pending_handles.push((id, handle));

            // We'll load icons after definitions are ready
        }
    }

    // Check if definitions are loaded
    if registry.is_loaded() {
        return;
    }

    let mut all_loaded = true;

    for (id, handle) in pending_handles.iter() {
        if let Some(definition) = item_assets.remove(handle) {
            // Register definition
            registry.register(*id, definition.clone());

            // Start loading icon if specified
            if !definition.icon.is_empty() {
                let icon_handle: Handle<Image> = asset_server.load(&definition.icon);
                icons_loading.push((*id, icon_handle));
            }
        } else {
            all_loaded = false;
        }
    }

    // Register icons that are loaded
    icons_loading.retain(|(id, handle)| {
        // Just register the handle, don't wait for actual load
        registry.register_icon(*id, handle.clone());
        false // Remove from pending
    });

    if all_loaded && !pending_handles.is_empty() {
        pending_handles.clear();
        registry.mark_loaded();
    }
}

/// Run condition: registry is loaded
pub fn registry_loaded(registry: Res<ItemRegistry>) -> bool {
    registry.is_loaded()
}
