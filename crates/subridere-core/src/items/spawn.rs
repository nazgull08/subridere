use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{
    component::{Pickupable, WorldItem},
    visual::{definition::VisualDefinition, spawn_item_visual_with_colliders},
};
use crate::{game_init::assets::GameAssets, items::definition::ItemDefinition};

/// Configuration for spawning a world item
pub struct WorldItemSpawnConfig {
    /// Item identifier (e.g. "wooden_staff")
    pub item_id: String,

    /// Quantity in this stack
    pub quantity: u32,

    /// World position
    pub position: Vec3,

    /// Initial velocity (optional, for throwing/dropping)
    pub initial_velocity: Option<Vec3>,
}

/// Spawn an item in the world with physics and visuals
///
/// This is the UNIVERSAL item spawner - use it everywhere:
/// - Initial loot placement
/// - Dropping from inventory
/// - Quest rewards
/// - Enemy drops
pub fn spawn_world_item(
    commands: &mut Commands,
    config: WorldItemSpawnConfig,
    game_assets: &GameAssets,
    visuals: &Assets<VisualDefinition>,
    item_defs: &Assets<ItemDefinition>,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Option<Entity> {
    // Get visual definition for this item
    let visual_handle = match config.item_id.as_str() {
        "wooden_staff" => &game_assets.wooden_staff_visual,
        "iron_helmet" => &game_assets.iron_helmet_visual,
        _ => {
            warn!("Unknown item_id: {}", config.item_id);
            return None;
        }
    };

    let Some(visual_def) = visuals.get(visual_handle) else {
        warn!("Visual definition not loaded for: {}", config.item_id);
        return None;
    };
    let def_handle = match config.item_id.as_str() {
        "wooden_staff" => &game_assets.wooden_staff_def,
        "iron_helmet" => &game_assets.iron_helmet_def,
        _ => {
            warn!("⚠️ Unknown item_id for definition: {}", config.item_id);
            return None;
        }
    };
    let name = item_defs
        .get(def_handle)
        .map(|def| Name::new(def.name.clone()))
        .unwrap_or_else(|| Name::new(config.item_id.clone()));

    // Build physics components
    let mut entity_commands = commands.spawn((
        Transform::from_translation(config.position),
        GlobalTransform::default(),
        Visibility::default(),
        // Physics
        RigidBody::Dynamic,
        Damping {
            linear_damping: 2.0,  // Stops sliding quickly
            angular_damping: 1.5, // Stops spinning
        },
        GravityScale(1.0),
        // Item data
        WorldItem::new(config.item_id.clone(), config.quantity),
        Pickupable,
        name,
    ));

    // Add velocity if specified (for throwing/dropping)
    if let Some(velocity) = config.initial_velocity {
        entity_commands.insert(Velocity {
            linvel: velocity,
            angvel: Vec3::ZERO,
        });
    }

    // Spawn visual as children
    let entity_id = entity_commands.id();
    entity_commands.with_children(|parent| {
        spawn_item_visual_with_colliders(parent, &visual_def.parts, meshes, materials);
    });

    Some(entity_id)
}
