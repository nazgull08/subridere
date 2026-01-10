use crate::{
    input::component::PlayerControlled, inventory::systems::TargetedItem,
    unit::component::PickupIntent,
};
use bevy::prelude::*;

/// Handle E key press to create PickupIntent
///
/// This system only handles input - the actual pickup logic
/// is in the inventory module (process_pickup_intent system).
pub fn handle_pickup_input(
    keys: Res<ButtonInput<KeyCode>>,
    targeted: Res<TargetedItem>,
    mut commands: Commands,
    player_query: Query<Entity, With<PlayerControlled>>,
) {
    // Only check input if E was just pressed
    if !keys.just_pressed(KeyCode::KeyE) {
        return;
    }

    // Check if player is looking at a pickupable item
    if targeted.entity.is_none() {
        info!("❌ No item to pick up");
        return;
    }

    // Get player entity
    let Ok(player_entity) = player_query.single() else {
        warn!("Player not found!");
        return;
    };

    // Add PickupIntent component to player
    commands.entity(player_entity).insert(PickupIntent);
    info!("🎯 PickupIntent created");
}
