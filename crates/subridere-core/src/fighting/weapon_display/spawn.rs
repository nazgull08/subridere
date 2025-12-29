use bevy::prelude::*;
use crate::fighting::projectile::weapons::WeaponType;
use super::component::WeaponDisplay;

/// Spawns weapon display mesh based on weapon type
pub fn spawn_weapon_display_mesh(
    weapon_type: WeaponType,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
    match weapon_type {
        WeaponType::MagicBolt => {
            let mesh = meshes.add(Sphere::new(0.3));
            let material = materials.add(StandardMaterial {
                base_color: Color::srgba(0.3, 0.6, 1.0, 1.0),
                emissive: LinearRgba::rgb(1.0, 2.0, 4.0),
                unlit: false,
                ..default()
            });
            (Mesh3d(mesh), MeshMaterial3d(material))
        }
        WeaponType::PhysicalCube => {
            let size = 0.3;
            let mesh = meshes.add(Cuboid::new(size, size, size));
            let material = materials.add(StandardMaterial {
                base_color: Color::srgba(0.2, 0.2, 1.0, 1.0),
                emissive: LinearRgba::rgb(0.2, 0.2, 1.0),
                unlit: false,
                ..default()
            });
            (Mesh3d(mesh), MeshMaterial3d(material))
        }
    }
}

/// Creates a complete weapon display entity as a child of camera
pub fn create_weapon_display<'w>(
    parent: &mut ChildSpawnerCommands<'w>,  // ✅ Правильный тип из Bevy 0.16
    weapon_type: WeaponType,
    meshes: &mut Assets<Mesh>, 
    materials: &mut Assets<StandardMaterial>,
) {
    let (mesh, material) = spawn_weapon_display_mesh(weapon_type, meshes, materials);
    let display = WeaponDisplay::default();
    
    parent.spawn((
        mesh,
        material,
        Transform::from_translation(display.base_position),
        GlobalTransform::default(),
        Visibility::Visible,
        display,
        Name::new(format!("WeaponDisplay_{:?}", weapon_type)),
    ));
}
