use bevy::prelude::*;
use super::shape::{PrimitiveShape, VisualPart};

/// Spawn visual parts as children of a parent entity
///
/// This creates the 3D mesh representation of an item based on its visual parts.
/// All parts are spawned as child entities with transforms relative to the parent.
///
/// # Arguments
/// * `spawner` - ChildSpawnerCommands for the parent entity (Bevy 0.16)
/// * `parts` - List of visual parts to spawn
/// * `meshes` - Bevy mesh assets
/// * `materials` - Bevy material assets
pub fn spawn_item_visual(
    spawner: &mut ChildSpawnerCommands,
    parts: &[VisualPart],
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    for part in parts {
        spawn_visual_part(spawner, part, meshes, materials);
    }
}

/// Spawn a single visual part as a child entity
fn spawn_visual_part(
    spawner: &mut ChildSpawnerCommands,
    part: &VisualPart,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    // Create mesh based on shape type
    let mesh = create_mesh_for_shape(&part.shape, part.size, meshes);
    
    // Create material with specified color
    let material = materials.add(StandardMaterial {
        base_color: part.bevy_color(),
        ..default()
    });
    
    // Spawn entity with mesh and material
    spawner.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(part.offset),
    ));
}

/// Create a mesh handle for a given primitive shape
fn create_mesh_for_shape(
    shape: &PrimitiveShape,
    size: Vec3,
    meshes: &mut Assets<Mesh>,
) -> Handle<Mesh> {
    match shape {
        PrimitiveShape::Cube => {
            meshes.add(Cuboid::new(size.x, size.y, size.z))
        }
        
        PrimitiveShape::Cylinder => {
            // size.x = radius, size.y = height
            meshes.add(Cylinder::new(size.x, size.y))
        }
        
        PrimitiveShape::Sphere => {
            // size.x = radius
            meshes.add(Sphere::new(size.x))
        }
        
        PrimitiveShape::Icosphere => {
            // size.x = radius
            // Icosphere with 5 subdivisions for decent quality
            meshes.add(Sphere::new(size.x).mesh().ico(5).unwrap())
        }
    }
}
