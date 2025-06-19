// src/game_init/rooms.rs

use bevy::prelude::*;
use crate::world::builders::grid::{register_room, spawn_registered_rooms};
use crate::world::room::types::RoomMap;

pub fn spawn_initial_rooms(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut room_map: ResMut<RoomMap>,
) {
    let room_size = Vec3::new(12.0, 6.0, 12.0);
    let wall_t = 0.1;

    for i in 0..3 {
        let pos = IVec3::new(0, 0, -i);
        register_room(&mut room_map, pos);
    }

    spawn_registered_rooms(
        &mut commands,
        &mut meshes,
        &mut materials,
        room_size,
        wall_t,
        &mut room_map,
    );
}
