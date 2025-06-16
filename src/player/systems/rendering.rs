use bevy::prelude::*;

use crate::player::component::{PlayerBody, PlayerVisual};

/// Система для синхронизации позиции тела игрока с основной сущностью
pub fn sync_player_body_transform(
    player_query: Query<&Transform, (With<crate::player::component::Player>, Without<PlayerBody>)>,
    mut body_query: Query<&mut Transform, (With<PlayerBody>, Without<crate::player::component::Player>)>,
) {
    if let Ok(player_transform) = player_query.single() {
        for mut body_transform in &mut body_query {
            body_transform.translation = player_transform.translation;
            body_transform.rotation = player_transform.rotation;
        }
    }
}

/// Система для обновления размеров тела игрока при изменении настроек
pub fn update_player_body_size(
    mut meshes: ResMut<Assets<Mesh>>,
    player_query: Query<&PlayerVisual, (With<crate::player::component::Player>, Changed<PlayerVisual>)>,
    body_query: Query<&Mesh3d, With<PlayerBody>>,
) {
    if let Ok(player_visual) = player_query.single() {
        for mesh_handle in &body_query {
            if let Some(mesh) = meshes.get_mut(&mesh_handle.0) {
                *mesh = Cuboid::new(
                    player_visual.body_size.x,
                    player_visual.body_size.y,
                    player_visual.body_size.z,
                ).into();
            }
        }
    }
}
