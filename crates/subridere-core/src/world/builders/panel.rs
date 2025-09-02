use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn spawn_panel(
    spawner: &mut ChildSpawnerCommands,
    meshes: &mut Assets<Mesh>,
    material: Handle<StandardMaterial>,
    size: Vec3,
    local_offset: Vec3,
    name: String,
) -> Entity {
    let mesh = meshes.add(Mesh::from(Cuboid::new(size.x, size.y, size.z)));
    spawner
        .spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(local_offset),
            RigidBody::Fixed,
            Collider::cuboid(size.x * 0.5, size.y * 0.5, size.z * 0.5),
            Name::new(name),
        ))
        .id()
}
