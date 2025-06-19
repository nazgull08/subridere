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
    let lifetime = 5.0;

    let velocity = direction.normalize() * speed;

    // ── Меш и материал ──
    let mesh = meshes.add(Mesh::from(Cuboid::new(size, size, size)));
    let material = materials.add(Color::srgba(1.0, 0.1, 0.1, 0.5));

    // ── Спавним снаряд ──
    let projectile_entity = commands.spawn((
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
            intensity: 3000.0,      // ← сила света
            radius: 15.0,           // ← зона действия
            color: Color::srgb(1.0, 0.2, 0.2),
            shadows_enabled: true, // можно выключить для производительности
            ..default()
        },
        Name::new("Projectile"),
    ));
    println!("Projectile {:?} spawned", projectile_entity.id());
}
