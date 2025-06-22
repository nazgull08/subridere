use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::component::Projectile;

pub fn spawn_projectile(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    direction: Vec3,
) {
    let size = 0.3;
    let speed = 10.0;
    let lifetime = 10.0;

    let velocity = direction.normalize() * speed;

    let mesh = meshes.add(Mesh::from(Cuboid::new(size, size, size)));
    let material = materials.add(Color::srgba(1.0, 0.1, 0.1, 0.5));

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(position),
        RigidBody::Dynamic,
        Velocity {
            linvel: velocity,
            angvel: Vec3::ZERO,
        },
        Collider::cuboid(size / 2.0, size / 2.0, size / 2.0),
        Projectile::new(size, lifetime, velocity),
        PointLight {
            intensity: 30000.0,
            radius: 25.0,
            color: Color::srgb(1.0, 0.2, 0.2),
            shadows_enabled: false,
            ..default()
        },
        Name::new("Projectile"),
    ));
}
