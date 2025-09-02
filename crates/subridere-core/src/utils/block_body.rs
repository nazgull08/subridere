use bevy::prelude::*;

#[derive(Clone)]
pub struct BlockPart {
    pub name: String,
    pub offset: Vec3,
    pub size: Vec3,
    pub material: Handle<StandardMaterial>,
}

impl BlockPart {
    pub fn new(name: &str, offset: Vec3, size: Vec3, material: Handle<StandardMaterial>) -> Self {
        Self {
            name: name.to_string(),
            offset,
            size,
            material,
        }
    }
}

pub fn spawn_blocky_body(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    parent: Entity,
    parts: Vec<BlockPart>,
) {
    commands.entity(parent).with_children(|child| {
        for part in parts {
            let mesh = meshes.add(Mesh::from(Cuboid::new(
                part.size.x,
                part.size.y,
                part.size.z,
            )));
            child.spawn((
                Mesh3d(mesh),
                MeshMaterial3d(part.material.clone()),
                Transform::from_translation(part.offset),
                Name::new(part.name),
            ));
        }
    });
}
