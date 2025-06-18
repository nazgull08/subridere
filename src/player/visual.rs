use bevy::prelude::*;

use crate::player::component::PlayerVisual;

/// Создает меш и материал для тела игрока
pub fn create_player_body_bundle(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    visual: &PlayerVisual,
) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
    let mesh = meshes.add(Cuboid::new(
        visual.body_size.x,
        visual.body_size.y,
        visual.body_size.z,
    ));

    let material = materials.add(StandardMaterial {
        base_color: visual.body_color,
        metallic: 0.1,
        perceptual_roughness: 0.8,
        ..default()
    });

    (Mesh3d(mesh), MeshMaterial3d(material))
}
