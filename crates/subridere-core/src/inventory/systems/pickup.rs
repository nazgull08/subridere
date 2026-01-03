use crate::{
    inventory::Inventory,
    items::component::{Pickupable, WorldItem},
    player::component::Player,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Resource to store currently targeted item
#[derive(Resource, Default)]
pub struct TargetedItem {
    pub entity: Option<Entity>,
    pub item_id: Option<String>,
    pub name: Option<String>,
}

/// Raycast from camera to detect pickupable items
pub fn detect_pickupable_items(
    mut targeted: ResMut<TargetedItem>,
    camera_query: Query<&GlobalTransform, With<Camera>>,
    player_query: Query<Entity, With<Player>>,
    rapier_context: ReadRapierContext,
    // ✅ Более специфичный query - только Pickupable items
    items_query: Query<(&WorldItem, &Name), With<Pickupable>>,
    // ✅ Для поиска parent
    parent_query: Query<&ChildOf>,
) {
    // Early returns для invalid state
    let Ok(camera_transform) = camera_query.single() else {
        clear_target(&mut targeted);
        return;
    };

    let Ok(player_entity) = player_query.single() else {
        clear_target(&mut targeted);
        return;
    };

    let Ok(rapier_context) = rapier_context.single() else {
        clear_target(&mut targeted);
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
        // No hit at all
        clear_target(&mut targeted);
        return;
    };

    if hit_entity == player_entity {
        clear_target(&mut targeted);
        return;
    }

    let item_entity = find_item_entity(hit_entity, &items_query, &parent_query);
    
    if let Some(entity) = item_entity {
        let (world_item, name) = items_query.get(entity).unwrap();
        
        targeted.entity = Some(entity);
        targeted.item_id = Some(world_item.item_id.clone());
        targeted.name = Some(name.to_string());
    } else {
        clear_target(&mut targeted);
    }
}

/// Helper: Find WorldItem entity from hit (checking parents)
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
    if let Ok(parent) = parent_query.get(hit_entity) {
        let parent_entity = parent.parent();
        if items_query.contains(parent_entity) {
            return Some(parent_entity);
        }
    }
    
    None
}

/// Helper: Clear targeted item
#[inline]
fn clear_target(targeted: &mut TargetedItem) {
    targeted.entity = None;
    targeted.item_id = None;
    targeted.name = None;
}

/// Handle E key press to pickup targeted item
pub fn handle_pickup_input(
    keys: Res<ButtonInput<KeyCode>>,
    targeted: Res<TargetedItem>,
    mut inventory_query: Query<&mut Inventory, With<Player>>,
    mut commands: Commands,
) {
    // Early return if E not pressed
    if !keys.just_pressed(KeyCode::KeyE) {
        return;
    }

    // Check if we have a targeted item
    let Some(target_entity) = targeted.entity else {
        info!("❌ No item targeted");
        return;
    };

    let Some(item_id) = &targeted.item_id else {
        warn!("⚠️ Targeted entity has no item_id!");
        return;
    };

    // Get player inventory
    let Ok(mut inventory) = inventory_query.single_mut() else {
        warn!("⚠️ Player inventory not found!");
        return;
    };

    // Try to add item to inventory
    if inventory.add_item(item_id.clone(), 1) {
        // Success! Despawn the item from world
        commands.entity(target_entity).despawn();
        info!("✅ Picked up: {}", item_id);
    } else {
        // Inventory full
        info!("❌ Inventory full!");
        // TODO: Show UI message
    }
}
