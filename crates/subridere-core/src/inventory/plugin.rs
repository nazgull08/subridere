// inventory/plugin.rs — Inventory plugin

use bevy::prelude::*;

use super::systems::drop::{DropToWorldEvent, handle_drop_to_world};
use super::systems::equipment_stats::sync_equipment_modifiers;
use super::systems::pickup::{
    TargetedItem, detect_pickupable_items, handle_pickup_input, process_pickup_intent,
};
use crate::items::registry::registry_loaded;
use crate::stats::recalculate::recalculate_stats;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<DropToWorldEvent>()
            // Resources
            .init_resource::<TargetedItem>()
            // Systems
            .add_systems(
                Update,
                (
                    detect_pickupable_items,
                    handle_pickup_input,
                    process_pickup_intent,
                    handle_drop_to_world,
                    sync_equipment_modifiers.before(recalculate_stats),
                )
                    .run_if(registry_loaded),
            );

        info!("✅ Inventory plugin initialized");
    }
}
