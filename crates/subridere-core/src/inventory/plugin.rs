use super::systems::{TargetedItem, detect_pickupable_items, process_pickup_intent};
use bevy::prelude::*;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TargetedItem>()
            .add_systems(Update, (detect_pickupable_items, process_pickup_intent));

        info!("✅ Inventory plugin initialized");
    }
}
