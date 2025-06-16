use bevy::prelude::*;

use crate::player::component::{PlayerBody, PlayerVisual};

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

/// Система для управления видимостью тела игрока
pub fn toggle_player_body_visibility(
    mut body_query: Query<&mut Visibility, With<PlayerBody>>,
    player_query: Query<&PlayerVisual, (With<crate::player::component::Player>, Changed<PlayerVisual>)>,
) {
    if let Ok(player_visual) = player_query.single() {
        for mut visibility in &mut body_query {
            *visibility = if player_visual.show_body {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

/// Система для обновления цвета тела игрока
pub fn update_player_body_color(
    mut materials: ResMut<Assets<StandardMaterial>>,
    body_query: Query<&MeshMaterial3d<StandardMaterial>, With<PlayerBody>>,
    player_query: Query<&PlayerVisual, (With<crate::player::component::Player>, Changed<PlayerVisual>)>,
) {
    if let Ok(player_visual) = player_query.single() {
        for material_handle in &body_query {
            if let Some(material) = materials.get_mut(&material_handle.0) {
                material.base_color = player_visual.body_color;
            }
        }
    }
}
