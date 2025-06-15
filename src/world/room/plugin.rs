use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::collision::spawn_room_colliders;
use super::geometry::generate_room_geometry;
use super::layout::spawn_room_layout;

pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_room);
    }
}

fn setup_room(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Генерация геометрии комнаты
    let geometry = generate_room_geometry(&mut meshes);

    // Спавним меш + физические коллайдеры
    spawn_room_colliders(commands.reborrow(), &meshes, geometry.mesh.clone());

    // Спавним всё визуальное и платформы
    spawn_room_layout(commands, materials, meshes, geometry);
}
