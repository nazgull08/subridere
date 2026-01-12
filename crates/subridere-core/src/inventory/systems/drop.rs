use bevy::prelude::*;

use crate::{
    game_init::assets::GameAssets,
    inventory::{Equipment, Inventory},
    items::{
        definition::ItemDefinition,
        spawn::{WorldItemSpawnConfig, spawn_world_item},
        visual::definition::VisualDefinition,
    },
};

/// Drop an item from inventory slot into the world
pub fn drop_from_inventory(
    commands: &mut Commands,
    inventory: &mut Inventory,
    slot_index: usize,
    drop_position: Vec3,
    drop_velocity: Vec3,
    game_assets: &GameAssets,
    visuals: &Assets<VisualDefinition>,
    item_defs: &Assets<ItemDefinition>,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> bool {
    // Get item from inventory
    let Some(item) = inventory.slots[slot_index].take() else {
        warn!("No item in slot {} to drop", slot_index);
        return false;
    };

    // Use universal spawner
    let spawned = spawn_world_item(
        commands,
        WorldItemSpawnConfig {
            item_id: item.item_id.clone(),
            quantity: item.quantity,
            position: drop_position,
            initial_velocity: Some(drop_velocity),
        },
        game_assets,
        visuals,
        item_defs,
        meshes,
        materials,
    );

    if spawned.is_some() {
        info!(
            "📤 Dropped {} (x{}) from inventory slot {}",
            item.item_id, item.quantity, slot_index
        );
        true
    } else {
        // Failed to spawn, return item to inventory
        inventory.slots[slot_index] = Some(item);
        warn!("⚠️ Failed to drop item, returned to inventory");
        false
    }
}

/// Drop an item from equipment slot into the world
pub fn drop_from_equipment(
    commands: &mut Commands,
    equipment: &mut Equipment,
    slot_type: crate::ui::inventory::systems::EquipmentSlotType,
    drop_position: Vec3,
    drop_velocity: Vec3,
    game_assets: &GameAssets,
    visuals: &Assets<VisualDefinition>,
    item_defs: &Assets<ItemDefinition>,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> bool {
    // Get item from equipment
    let equip_slot = equipment.get_slot_mut(slot_type);
    let Some(item_id) = equip_slot.take() else {
        warn!("⚠️ No item equipped in {:?} to drop", slot_type);
        return false;
    };

    // Use universal spawner
    let spawned = spawn_world_item(
        commands,
        WorldItemSpawnConfig {
            item_id: item_id.clone(),
            quantity: 1, // Equipment is always 1
            position: drop_position,
            initial_velocity: Some(drop_velocity),
        },
        game_assets,
        visuals,
        item_defs,
        meshes,
        materials,
    );

    if spawned.is_some() {
        info!("📤 Dropped {} from equipment {:?}", item_id, slot_type);
        true
    } else {
        // Failed to spawn, return item to equipment
        *equip_slot = Some(item_id);
        warn!("Failed to drop item, returned to equipment");
        false
    }
}
