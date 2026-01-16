// inventory/systems/drop.rs — Item drop system

use bevy::prelude::*;

use crate::inventory::component::{Equipment, Inventory};
use crate::items::{EquipmentSlot, ItemRegistry, spawn_world_item};
use crate::player::component::Player;

// ============================================================
// Event
// ============================================================

/// Event: drop item from inventory/equipment to world
#[derive(Event)]
pub struct DropToWorldEvent {
    pub source: DropSource,
}

/// Where the dropped item comes from
#[derive(Clone, Copy, Debug)]
pub enum DropSource {
    Inventory(usize),
    Equipment(EquipmentSlot),
}

// ============================================================
// System: Handle drop event
// ============================================================

/// Process DropToWorldEvent — remove from inventory/equipment, spawn in world
pub fn handle_drop_to_world(
    mut commands: Commands,
    mut events: EventReader<DropToWorldEvent>,
    mut player_query: Query<(&Transform, &mut Inventory, &mut Equipment), With<Player>>,
    registry: Res<ItemRegistry>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in events.read() {
        info!("🔍 handle_drop_to_world received event: {:?}", event.source);
        let Ok((transform, mut inventory, mut equipment)) = player_query.single_mut() else {
            continue;
        };

        let drop_position = transform.translation + transform.forward() * 1.5 + Vec3::Y * 0.5;
        let drop_velocity = Vec3::Y * 2.0 + transform.forward() * 1.0;

        let (item_id, quantity) = match event.source {
            DropSource::Inventory(slot) => {
                let Some(stack) = inventory.remove_slot(slot) else {
                    warn!("No item in inventory slot {} to drop", slot);
                    continue;
                };
                (stack.id, stack.quantity)
            }
            DropSource::Equipment(slot) => {
                let Some(id) = equipment.unequip(slot) else {
                    warn!("No item in equipment slot {:?} to drop", slot);
                    continue;
                };
                (id, 1)
            }
        };

        spawn_world_item(
            &mut commands,
            &registry,
            item_id,
            quantity,
            drop_position,
            Some(drop_velocity),
            &mut meshes,
            &mut materials,
        );

        info!("📤 Dropped {} (x{}) to world", item_id, quantity);
    }
}
