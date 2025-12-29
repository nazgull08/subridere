use bevy::prelude::*;
use bevy_rapier3d::prelude::{
    CharacterLength, Collider, KinematicCharacterController, KinematicCharacterControllerOutput,
};
use block_bodies_core::{PartId, serialization::BlockBodyFile};
use std::collections::HashMap;

use crate::enemies::{
    components::Enemy,
    worm::components::{Worm, WormAI, WormHead, WormState},
};
use crate::utils::animated_body::AnimatedBody;

pub fn spawn_worm(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) -> Entity {
    let model_path = "./assets/models/worm.ron";
    info!("loading from: {}", model_path);

    let body_file = BlockBodyFile::load_from_file(model_path).expect("Failed to load worm.ron");
    let body = body_file.to_body().expect("Failed to parse worm body");

    info!("📊 Loaded worm body with {} parts", body.parts.len());

    // Materials
    let red_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.2, 0.2),
        metallic: 0.1,
        perceptual_roughness: 0.8,
        ..default()
    });

    let green_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.7, 0.3),
        metallic: 0.1,
        perceptual_roughness: 0.8,
        ..default()
    });

    // Create root entity (logical container)
    let worm_id = commands
        .spawn((
            Enemy,
            Worm::default(),
            Transform::from_translation(position),
            Visibility::Visible,
            Name::new("Worm"),
        ))
        .id();

    let mut part_entities: HashMap<PartId, Entity> = HashMap::new();
    let mut head_entity = None;

    // Spawn all parts as flat children
    commands.entity(worm_id).with_children(|parent| {
        for (part_id, part) in body.parts.iter() {
            let material = if part.name == "Head" || part.name == "Tail" {
                red_material.clone()
            } else {
                green_material.clone()
            };

            let mesh = meshes.add(Cuboid::new(part.size.x, part.size.y, part.size.z));

            // Initial positions in a line
            let segment_index = match part.name.as_str() {
                "Head" => 0,
                "Seg1" => 1,
                "Seg2" => 2,
                "Seg3" => 3,
                "Seg4" => 4,
                "Tail" => 5,
                _ => 0,
            };

            let spacing = 0.8;
            let initial_offset = Vec3::new(segment_index as f32 * spacing, 0.0, 0.0);

            let mut entity_commands = parent.spawn((
                Mesh3d(mesh),
                MeshMaterial3d(material),
                Transform::from_translation(initial_offset),
                Name::new(part.name.clone()),
            ));

            // Mark head with special component
            if part.name == "Head" {
                entity_commands.insert((
                    WormHead { worm_root: worm_id },
                    WormAI::default(),
                    WormState::default(),
                    Collider::capsule_y(0.3, 0.4),
                    KinematicCharacterController {
                        offset: CharacterLength::Absolute(0.01),
                        ..default()
                    },
                    KinematicCharacterControllerOutput::default(),
                ));
                head_entity = Some(entity_commands.id());
            }

            part_entities.insert(part_id, entity_commands.id());

            info!("  - Spawned {} at offset {:?}", part.name, initial_offset);
        }
    });

    // Add AnimatedBody to root
    commands.entity(worm_id).insert(AnimatedBody {
        body,
        part_entities,
    });

    info!(
        "🐛 Spawned worm at {:?}, head entity: {:?}",
        position, head_entity
    );

    worm_id
}

/// Recursively spawns body parts with correct parent-child hierarchy
fn spawn_part_hierarchy(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    body: &block_bodies_core::BlockBody,
    parent_entity: Entity,
    parent_name: Option<&str>,
    red_material: &Handle<StandardMaterial>,
    green_material: &Handle<StandardMaterial>,
    part_entities: &mut HashMap<PartId, Entity>,
) {
    // Find all parts with this parent
    for (part_id, part) in body.parts.iter() {
        let is_child_of_parent = match (&part.parent, parent_name) {
            (None, None) => true,           // root part
            (Some(p), Some(pn)) => p == pn, // child of specific parent
            _ => false,
        };

        if !is_child_of_parent {
            continue;
        }

        // Choose material
        let material = if part.name == "Head" || part.name == "Tail" {
            red_material.clone()
        } else {
            green_material.clone()
        };

        // Create mesh
        let mesh = meshes.add(Cuboid::new(part.size.x, part.size.y, part.size.z));

        // Spawn this part as child of parent_entity
        let part_entity = commands
            .spawn((
                Mesh3d(mesh),
                MeshMaterial3d(material),
                Transform::from_translation(part.position).with_rotation(part.rotation),
                Name::new(part.name.clone()),
            ))
            .id();

        commands.entity(parent_entity).add_child(part_entity);

        // Store PartId -> Entity mapping (KEY CHANGE!)
        part_entities.insert(part_id, part_entity);

        // Recursively spawn children of this part
        spawn_part_hierarchy(
            commands,
            meshes,
            body,
            part_entity, // this part becomes parent for next level
            Some(&part.name),
            red_material,
            green_material,
            part_entities,
        );
    }
}
