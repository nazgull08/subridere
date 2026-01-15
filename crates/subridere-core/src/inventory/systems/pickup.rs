// inventory/systems/pickup.rs — Item pickup system

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::items::{ItemId, ItemRegistry};
use crate::player::component::Player;

use super::super::component::Inventory;
use super::world_item::{Pickupable, WorldItem};

/// Resource: currently targeted item (raycast result)
#[derive(Resource, Default)]
pub struct TargetedItem {
    pub entity: Option<Entity>,
    pub id: Option<ItemId>,
    pub name: Option<String>,
}

impl TargetedItem {
    pub fn clear(&mut self) {
        self.entity = None;
        self.id = None;
        self.name = None;
    }

    pub fn is_some(&self) -> bool {
        self.entity.is_some()
    }
}

/// Marker component: player wants to pick up targeted item
#[derive(Component)]
pub struct PickupIntent;

/// System: Raycast from camera to detect pickupable items
pub fn detect_pickupable_items(
    mut targeted: ResMut<TargetedItem>,
    camera_query: Query<&GlobalTransform, With<Camera>>,
    player_query: Query<Entity, With<Player>>,
    rapier_context: ReadRapierContext,
    items_query: Query<(&WorldItem, &Name), With<Pickupable>>,
    parent_query: Query<&ChildOf>,
) {
    // Early returns for invalid state
    let Ok(camera_transform) = camera_query.single() else {
        targeted.clear();
        return;
    };

    let Ok(player_entity) = player_query.single() else {
        targeted.clear();
        return;
    };

    let Ok(rapier_context) = rapier_context.single() else {
        targeted.clear();
        return;
    };

    let ray_dir = camera_transform.forward();
    let ray_origin = camera_transform.translation() + *ray_dir * 0.5;
    let max_distance = 3.0;

    // Perform raycast
    let Some((hit_entity, _distance)) = rapier_context.cast_ray(
        ray_origin,
        *ray_dir,
        max_distance,
        true,
        QueryFilter::default(),
    ) else {
        targeted.clear();
        return;
    };

    // Don't target self
    if hit_entity == player_entity {
        targeted.clear();
        return;
    }

    // Find WorldItem entity (might be parent if we hit visual child)
    let item_entity = find_item_entity(hit_entity, &items_query, &parent_query);

    if let Some(entity) = item_entity {
        let (world_item, name) = items_query.get(entity).unwrap();

        targeted.entity = Some(entity);
        targeted.id = Some(world_item.id);
        targeted.name = Some(name.to_string());
    } else {
        targeted.clear();
    }
}

/// Helper: Find WorldItem entity from raycast hit (checking parents)
fn find_item_entity(
    hit_entity: Entity,
    items_query: &Query<(&WorldItem, &Name), With<Pickupable>>,
    parent_query: &Query<&ChildOf>,
) -> Option<Entity> {
    // Check direct hit first
    if items_query.contains(hit_entity) {
        return Some(hit_entity);
    }

    // Check parent (for hitting visual children)
    if let Ok(child_of) = parent_query.get(hit_entity) {
        let parent_entity = child_of.parent();
        if items_query.contains(parent_entity) {
            return Some(parent_entity);
        }
    }

    None
}

/// System: Process PickupIntent — add to inventory, despawn world item
pub fn process_pickup_intent(
    mut commands: Commands,
    targeted: Res<TargetedItem>,
    registry: Res<ItemRegistry>,
    world_items: Query<&WorldItem>,
    mut player_query: Query<(Entity, &mut Inventory), (With<Player>, With<PickupIntent>)>,
) {
    // Process each player that has PickupIntent
    for (player_entity, mut inventory) in &mut player_query {
        // Remove intent immediately (consume it)
        commands.entity(player_entity).remove::<PickupIntent>();

        // Verify we have a valid target
        let Some(target_entity) = targeted.entity else {
            warn!("PickupIntent but no valid target!");
            continue;
        };

        let Some(id) = targeted.id else {
            warn!("Target entity has no item_id!");
            continue;
        };

        // Get quantity from world item
        let quantity = world_items
            .get(target_entity)
            .map(|wi| wi.quantity)
            .unwrap_or(1);

        // Try to add item to inventory
        if inventory.add(id, quantity, &registry) {
            // Success! Remove item from world
            commands.entity(target_entity).despawn();
            info!("✅ Picked up: {} (x{})", id, quantity);
        } else {
            // Inventory full
            info!("❌ Inventory full! Cannot pick up {}", id);
            // TODO: Show UI message
        }
    }
}

/// System: Handle E key press to create PickupIntent
///
/// Note: This should be in input module, but included here for completeness.
/// Move to input/systems/pickup.rs when integrating.
pub fn handle_pickup_input(
    keys: Res<ButtonInput<KeyCode>>,
    targeted: Res<TargetedItem>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    if !keys.just_pressed(KeyCode::KeyE) {
        return;
    }

    if !targeted.is_some() {
        return;
    }

    let Ok(player_entity) = player_query.single() else {
        return;
    };

    commands.entity(player_entity).insert(PickupIntent);
}
