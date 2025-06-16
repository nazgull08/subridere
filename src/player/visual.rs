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

/// Система для обновления цвета тела игрока
pub fn update_player_body_color(
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(&MeshMaterial3d<StandardMaterial>, &PlayerVisual), (With<crate::player::component::Player>, Changed<PlayerVisual>)>,
) {
    for (material_handle, player_visual) in &query {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.base_color = player_visual.body_color;
        }
    }
}

/// Система для обновления размеров тела игрока
pub fn update_player_body_size(
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(&Mesh3d, &PlayerVisual), (With<crate::player::component::Player>, Changed<PlayerVisual>)>,
) {
    for (mesh_handle, player_visual) in &query {
        if let Some(mesh) = meshes.get_mut(&mesh_handle.0) {
            *mesh = Cuboid::new(
                player_visual.body_size.x,
                player_visual.body_size.y,
                player_visual.body_size.z,
            ).into();
        }
    }
}
