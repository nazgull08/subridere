// inventory/plugin.rs — Inventory plugin

use bevy::prelude::*;

use super::systems::drop::{DropToWorldEvent, handle_drop_to_world};
use super::systems::pickup::{
    TargetedItem, detect_pickupable_items, handle_pickup_input, process_pickup_intent,
};
use crate::items::registry::registry_loaded;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<DropToWorldEvent>()
            // Resources
            .init_resource::<TargetedItem>()
            // Systems - only run after items are loaded
            .add_systems(
                Update,
                (
                    detect_pickupable_items,
                    handle_pickup_input,
                    process_pickup_intent,
                    handle_drop_to_world,
                )
                    .run_if(registry_loaded),
            );

        info!("✅ Inventory plugin initialized");
    }
}
