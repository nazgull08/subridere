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
    player_query: Query<Entity, With<Player>>, // ДОБАВИТЬ
    rapier_context: ReadRapierContext,
    items_query: Query<(&WorldItem, &Name), With<Pickupable>>,
    parent_query: Query<&ChildOf>,
    debug_names: Query<Option<&Name>>,
) {
    let Ok(camera_transform) = camera_query.single() else {
        targeted.entity = None;
        targeted.item_id = None;
        targeted.name = None;
        return;
    };

    let Ok(player_entity) = player_query.single() else {
        // ДОБАВИТЬ
        return;
    };

    let Ok(rapier_context) = rapier_context.single() else {
        return;
    };

    // Raycast from camera forward
    let ray_origin = camera_transform.translation();
    let ray_dir = camera_transform.forward();
    let max_distance = 3.0;

    // Perform raycast, excluding player
    let hit = rapier_context.cast_ray(
        ray_origin,
        *ray_dir,
        max_distance,
        true,
        QueryFilter::default().exclude_rigid_body(player_entity), // Исключить игрока
    );
    if let Some((hit_entity, toi)) = hit {
        let name = debug_names
            .get(hit_entity)
            .ok()
            .flatten()
            .map(|n| n.as_str())
            .unwrap_or("NO NAME");

        info!(
            "🎯 Hit entity: {:?}, name: '{}', distance: {}",
            hit_entity, name, toi
        );

        // Попробуем найти parent
        if let Ok(parent) = parent_query.get(hit_entity) {
            let parent_name = debug_names
                .get(parent.0)
                .ok()
                .flatten()
                .map(|n| n.as_str())
                .unwrap_or("NO NAME");
            info!("  ↑ Parent: {:?}, name: '{}'", parent.0, parent_name);
        } else {
            info!("  ↑ No parent");
        }
    } else {
        info!("🎯 No hit");
    }

    // No item in range
    targeted.entity = None;
    targeted.item_id = None;
    targeted.name = None;
}

/// Handle E key press to pickup targeted item
pub fn handle_pickup_input(
    keys: Res<ButtonInput<KeyCode>>,
    targeted: Res<TargetedItem>,
    mut inventory_query: Query<&mut Inventory, With<crate::player::component::Player>>,
    mut commands: Commands,
) {
    // Check if E was just pressed
    if !keys.just_pressed(KeyCode::KeyE) {
        return;
    }

    // Check if we have a targeted item
    let Some(target_entity) = targeted.entity else {
        return;
    };

    let Some(item_id) = &targeted.item_id else {
        return;
    };

    // Get player inventory
    let Ok(mut inventory) = inventory_query.single_mut() else {
        warn!("Player inventory not found!");
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
        // TODO: Show UI message in next step
    }
}
