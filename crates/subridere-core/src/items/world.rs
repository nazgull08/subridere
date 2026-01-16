// items/world.rs — World item spawning

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::visual::{ItemVisual, spawn_item_visual_with_colliders};
use super::{ItemId, ItemRegistry};

/// An item entity in the game world
#[derive(Component, Clone, Debug)]
pub struct WorldItem {
    pub id: ItemId,
    pub quantity: u32,
}

impl WorldItem {
    pub fn new(id: ItemId) -> Self {
        Self { id, quantity: 1 }
    }

    pub fn with_quantity(id: ItemId, quantity: u32) -> Self {
        Self { id, quantity }
    }
}

/// Marker: can be picked up
#[derive(Component, Default)]
pub struct Pickupable;

/// Marker: currently targeted by player
#[derive(Component)]
pub struct Targeted;

/// Spawn item in world with physics and colliders
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
        RigidBody::Dynamic,
        Restitution::coefficient(0.3),
        Damping {
            linear_damping: 2.0,
            angular_damping: 1.5,
        },
    ));

    if let Some(vel) = velocity {
        entity_commands.insert(Velocity {
            linvel: vel,
            ..default()
        });
    }

    let entity = entity_commands.id();

    // Spawn visual children with colliders
    if let ItemVisual::Primitive { parts } = &def.visual {
        commands.entity(entity).with_children(|parent| {
            spawn_item_visual_with_colliders(parent, parts, meshes, materials);
        });
    }

    entity
}
