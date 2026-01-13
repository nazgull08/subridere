// inventory/systems/drop.rs — Item drop system

use bevy::prelude::*;

use crate::items::{ItemId, ItemRegistry};

use super::super::component::{Equipment, Inventory};
use super::world_item::{Pickupable, WorldItem};

/// Marker component: entity wants to drop an item
#[derive(Component)]
pub struct DropIntent {
    pub source: DropSource,
}

/// Where the dropped item comes from
#[derive(Clone, Debug)]
pub enum DropSource {
    /// From inventory slot
    InventorySlot(usize),
    
    /// From equipment slot  
    EquipmentSlot(crate::items::EquipmentSlot),
}

/// Spawn a world item at position
/// 
/// Creates entity with WorldItem, Pickupable, physics, and visual.
/// Returns the spawned entity.
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

    // Add physics
    entity_commands.insert((
        bevy_rapier3d::prelude::RigidBody::Dynamic,
        bevy_rapier3d::prelude::Collider::cuboid(0.15, 0.15, 0.15),
        bevy_rapier3d::prelude::Restitution::coefficient(0.3),
    ));

    // Add initial velocity if specified
    if let Some(vel) = velocity {
        entity_commands.insert(bevy_rapier3d::prelude::Velocity {
            linvel: vel,
            ..default()
        });
    }

    let entity = entity_commands.id();

    // Spawn visual as child
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
        return; // No visual or Model (future)
    };

    commands.entity(parent).with_children(|builder| {
        for part in parts {
            let mesh = match part.shape {
                VisualShape::Cube => Mesh::from(Cuboid::new(part.size.0, part.size.1, part.size.2)),
                VisualShape::Sphere => Mesh::from(Sphere::new(part.size.0)),
                VisualShape::Cylinder => {
                    Mesh::from(Cylinder::new(part.size.0, part.size.1))
                }
                VisualShape::Capsule => {
                    Mesh::from(Capsule3d::new(part.size.0, part.size.1))
                }
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

/// Drop item from inventory into world
pub fn drop_from_inventory(
    commands: &mut Commands,
    inventory: &mut Inventory,
    slot_index: usize,
    drop_position: Vec3,
    drop_velocity: Vec3,
    registry: &ItemRegistry,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> bool {
    let Some(stack) = inventory.remove_slot(slot_index) else {
        warn!("No item in slot {} to drop", slot_index);
        return false;
    };

    spawn_world_item(
        commands,
        registry,
        stack.id,
        stack.quantity,
        drop_position,
        Some(drop_velocity),
        meshes,
        materials,
    );

    info!("📤 Dropped {} (x{}) from inventory slot {}", stack.id, stack.quantity, slot_index);
    true
}

/// Drop item from equipment into world
pub fn drop_from_equipment(
    commands: &mut Commands,
    equipment: &mut Equipment,
    slot: crate::items::EquipmentSlot,
    drop_position: Vec3,
    drop_velocity: Vec3,
    registry: &ItemRegistry,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> bool {
    let Some(id) = equipment.unequip(slot) else {
        warn!("No item in equipment slot {:?} to drop", slot);
        return false;
    };

    spawn_world_item(
        commands,
        registry,
        id,
        1, // Equipment is always quantity 1
        drop_position,
        Some(drop_velocity),
        meshes,
        materials,
    );

    info!("📤 Dropped {} from equipment {:?}", id, slot);
    true
}
