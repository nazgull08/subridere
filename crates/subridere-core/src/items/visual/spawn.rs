// items/visual/spawn.rs — Item visual spawning

use super::shape::{VisualPart, VisualShape};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Spawn visual parts as children (NO colliders — for UI, equipped display)
pub fn spawn_item_visual(
    spawner: &mut ChildSpawnerCommands,
    parts: &[VisualPart],
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    for part in parts {
        let mesh = create_mesh_for_shape(part.shape, part.size_vec3(), meshes);
        let material = materials.add(StandardMaterial {
            base_color: part.bevy_color(),
            ..default()
        });

        spawner.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(part.offset_vec3()),
        ));
    }
}

/// Spawn visual parts WITH colliders (for world items)
pub fn spawn_item_visual_with_colliders(
    spawner: &mut ChildSpawnerCommands,
    parts: &[VisualPart],
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    for part in parts {
        let mesh = create_mesh_for_shape(part.shape, part.size_vec3(), meshes);
        let material = materials.add(StandardMaterial {
            base_color: part.bevy_color(),
            ..default()
        });
        let collider = create_collider_for_shape(part.shape, part.size_vec3());

        spawner.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(part.offset_vec3()),
            collider,
        ));
    }
}

fn create_mesh_for_shape(
    shape: VisualShape,
    size: Vec3,
    meshes: &mut Assets<Mesh>,
) -> Handle<Mesh> {
    match shape {
        VisualShape::Cube => meshes.add(Cuboid::new(size.x, size.y, size.z)),
        VisualShape::Sphere => meshes.add(Sphere::new(size.x)),
        VisualShape::Cylinder => meshes.add(Cylinder::new(size.x, size.y)),
        VisualShape::Capsule => meshes.add(Capsule3d::new(size.x, size.y)),
    }
}

fn create_collider_for_shape(shape: VisualShape, size: Vec3) -> Collider {
    match shape {
        VisualShape::Cube => Collider::cuboid(size.x / 2.0, size.y / 2.0, size.z / 2.0),
        VisualShape::Sphere => Collider::ball(size.x),
        VisualShape::Cylinder => Collider::cylinder(size.y / 2.0, size.x),
        VisualShape::Capsule => Collider::capsule_y(size.y / 2.0, size.x),
    }
}
