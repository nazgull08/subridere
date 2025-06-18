use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::data::*;
use super::geometry::RoomGeometry;

pub fn spawn_room_layout(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,

    geometry: RoomGeometry,
) {
    // ── Стены и пол ───────────────────────────────
    let cube_material = materials.add(Color::srgb(0.9, 0.85, 0.8));
    commands.spawn((
        (Mesh3d(geometry.mesh.clone()),
         MeshMaterial3d(cube_material),
         Transform::IDENTITY,
        ),
        RigidBody::Fixed,
        geometry.collider.clone(),
        Name::new("Room"),
    ));

    // ── Простейшая мебель ─────────────────────────
    let furniture_mesh = meshes.add(Mesh::from(Cuboid::new(0.4, 0.8, 0.4)));
    let furniture_material = materials.add(Color::srgb(0.4, 0.3, 0.2));
    let furniture_positions = [
        Vec3::new(0.0, 0.4, 0.0),
        Vec3::new(1.0, 0.4, 1.0),
        Vec3::new(-1.0, 0.4, -1.0),
    ];
    
    for pos in furniture_positions {
        commands.spawn((
            Mesh3d(furniture_mesh.clone()),       // ← clone здесь
            MeshMaterial3d(furniture_material.clone()),
            Transform::from_translation(pos),
            RigidBody::Fixed,
            Collider::cuboid(0.2, 0.4, 0.2),      // ← если хочешь коллизии
            Name::new("Furniture"),
        ));
    }

    // ── Прыжковые платформы ───────────────────────
    let platform_mesh = meshes.add(Mesh::from(Cuboid::new(2.0, 0.2, 2.0))); // ← правильно
    let platform_material = materials.add(Color::srgb(0.2, 0.5, 1.0));

    for pos in geometry.platform_positions {
        commands.spawn((
            Mesh3d(platform_mesh.clone()),        // ← исправлено здесь
            MeshMaterial3d(platform_material.clone()),
            Transform::from_translation(pos),
            RigidBody::Fixed,
            Collider::cuboid(1.0, 0.1, 1.0),
            Name::new("Platform"),
        ));
    }


    let ball_mesh = meshes.add(Mesh::from(Sphere {
        radius: 0.5,
        ..default()
    }));
    let ball_material = materials.add(Color::srgb(0.0, 0.1, 0.4));

    let object_radius = 0.5;
    let room_top_y = ROOM_HEIGHT as f32 * CUBE_SIZE;
    let spawn_y = room_top_y + object_radius + 0.05;

    for i in 0..10 {
        let x = (i as f32 - 1.0) * 1.0;
        let z = ROOM_Z_OFFSET + 1.0 + i as f32;

        commands.spawn((
            Mesh3d(ball_mesh.clone()),
            MeshMaterial3d(ball_material.clone()),
            Transform::from_xyz(x, spawn_y, z),
            RigidBody::Dynamic,
            Collider::ball(object_radius),
            Name::new(format!("RoofBall_{i}")),
        ));
    }
}
