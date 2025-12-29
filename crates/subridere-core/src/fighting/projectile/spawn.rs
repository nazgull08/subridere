use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::fighting::projectile::weapons::{DamageProjectile, PhysicalCube};

use super::component::Projectile;

/// Spawns a magic bolt (fast, damages enemies)
pub fn spawn_magic_bolt(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    direction: Vec3,
) {
    let size = 0.3;
    let speed = 30.0;  // ✅ Быстрее чем кубы!
    let lifetime = 3.0;  // ✅ Короткоживущий

    let velocity = direction.normalize() * speed;

    let mesh = meshes.add(Sphere::new(size));  // ✅ Сфера вместо куба
    let material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.3, 0.6, 1.0, 0.9),  // ✅ Синий
        emissive: LinearRgba::rgb(2.0, 4.0, 8.0),  // ✅ Светится!
        ..default()
    });

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(position),
        GlobalTransform::default(),
        Visibility::Visible,
        RigidBody::Dynamic,
        Velocity {
            linvel: velocity,
            angvel: Vec3::ZERO,
        },
        Collider::ball(size),
        Sensor,  // ✅ Не создает физических столкновений
        ActiveEvents::COLLISION_EVENTS,
        Projectile::new(size, lifetime, velocity),
        DamageProjectile,  // ✅ МАРКЕР - наносит урон!
        PointLight {
            intensity: 50000.0,
            radius: 30.0,
            color: Color::srgb(0.3, 0.6, 1.0),
            shadows_enabled: false,
            ..default()
        },
        Name::new("MagicBolt"),
    ));
}

/// Spawns a physical cube (slow, doesn't damage enemies)
pub fn spawn_physical_cube(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    direction: Vec3,
) {
    let size = 0.9;
    let speed = 10.0;  // Медленнее
    let lifetime = 10.0;  // Долгоживущий

    let velocity = direction.normalize() * speed;

    let mesh = meshes.add(Cuboid::new(size, size, size));
    let material = materials.add(Color::srgba(0.1, 0.1, 1.0, 0.8));

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(position),
        GlobalTransform::default(),
        Visibility::Visible,
        RigidBody::Dynamic,
        Velocity {
            linvel: velocity,
            angvel: Vec3::ZERO,
        },
        Collider::cuboid(size / 2.0, size / 2.0, size / 2.0),
        Projectile::new(size, lifetime, velocity),
        PhysicalCube,  // ✅ МАРКЕР - физический объект!
        PointLight {
            intensity: 30000.0,
            radius: 25.0,
            color: Color::srgb(0.2, 0.2, 1.0),
            shadows_enabled: false,
            ..default()
        },
        Name::new("PhysicalCube"),
    ));
}
