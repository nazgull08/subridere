// inventory/systems/drop.rs — Item drop system

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::inventory::component::{Equipment, Inventory};
use crate::items::{EquipmentSlot, ItemId, ItemRegistry};
use crate::player::component::Player;

use super::world_item::{Pickupable, WorldItem};

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

        // Calculate drop position (in front of player)
        let drop_position = transform.translation + transform.forward() * 1.5 + Vec3::Y * 0.5;

        // Small upward velocity for nice arc
        let drop_velocity = Vec3::Y * 2.0 + transform.forward() * 1.0;

        // Get item and remove from source
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

        // Spawn in world
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

// ============================================================
// Spawn functions
// ============================================================

/// Spawn a world item at position
pub fn spawn_world_item(
    commands: &mut Commands,
    registry: &ItemRegistry,
    id: ItemId,
    quantity: u32,
    position: Vec3,
    velocity: Option<Vec3>,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Entity {
    let def = registry.get(id);

    let mut entity_commands = commands.spawn((
        Name::new(def.name.clone()),
        WorldItem::with_quantity(id, quantity),
        Pickupable,
        Transform::from_translation(position),
        Visibility::Visible,
    ));

    // Physics
    entity_commands.insert((
        RigidBody::Dynamic,
        Collider::cuboid(0.15, 0.15, 0.15),
        Restitution::coefficient(0.3),
    ));

    // Initial velocity
    if let Some(vel) = velocity {
        entity_commands.insert(Velocity {
            linvel: vel,
            ..default()
        });
    }

    let entity = entity_commands.id();

    // Visual as child
    spawn_item_visual(commands, entity, def, meshes, materials);

    entity
}

/// Spawn visual meshes for item
fn spawn_item_visual(
    commands: &mut Commands,
    parent: Entity,
    def: &crate::items::ItemDefinition,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    use crate::items::visual::{ItemVisual, VisualShape};

    let ItemVisual::Primitive { parts } = &def.visual else {
        return;
    };

    commands.entity(parent).with_children(|builder| {
        for part in parts {
            let mesh = match part.shape {
                VisualShape::Cube => Mesh::from(Cuboid::new(part.size.0, part.size.1, part.size.2)),
                VisualShape::Sphere => Mesh::from(Sphere::new(part.size.0)),
                VisualShape::Cylinder => Mesh::from(Cylinder::new(part.size.0, part.size.1)),
                VisualShape::Capsule => Mesh::from(Capsule3d::new(part.size.0, part.size.1)),
            };

            let material = StandardMaterial {
                base_color: Color::srgba(part.color.0, part.color.1, part.color.2, part.color.3),
                ..default()
            };

            builder.spawn((
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(materials.add(material)),
                Transform::from_translation(Vec3::new(part.offset.0, part.offset.1, part.offset.2)),
            ));
        }
    });
}
