use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn spawn_room_colliders(
    mut commands: Commands,
    meshes: &ResMut<Assets<Mesh>>,
    handle: Handle<Mesh>,
) {
    if let Some(mesh) = meshes.get(&handle) {
        if let Some(collider) = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh(TriMeshFlags::ORIENTED)) {
            commands.spawn((
                RigidBody::Fixed,
                collider,
                Transform::IDENTITY,
                Name::new("RoomCollider"),
            ));
        } else {
            print!("❌ Не удалось сгенерировать TriMesh-коллайдер из меша комнаты!");
        }
    }
}
