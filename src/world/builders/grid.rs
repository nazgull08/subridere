// src/world/builders/grid.rs

use crate::world::builders::room::spawn_simple_room;
use crate::world::room::types::{DoorFlags, RoomMap, RoomMetadata, WallFlags};
use bevy::prelude::*;
use std::collections::HashMap;

pub fn register_room(map: &mut RoomMap, pos: IVec3) {
    let mut wall = WallFlags::default();
    let mut door = DoorFlags::default();

    let dirs = [
        (IVec3::new(0, 0, -1), &mut wall.back, &mut door.back),
        (IVec3::new(0, 0, 1), &mut wall.front, &mut door.front),
        (IVec3::new(-1, 0, 0), &mut wall.left, &mut door.left),
        (IVec3::new(1, 0, 0), &mut wall.right, &mut door.right),
    ];

    for (offset, wall_flag, door_flag) in dirs {
        let neighbor_pos = pos + offset;
        if map.rooms.contains_key(&neighbor_pos) {
            // У нас есть сосед — ставим стену с дверью с нашей стороны
            *wall_flag = true;
            *door_flag = true;

            // И у соседа убираем его стену и добавляем дверь
            if let Some(neigh) = map.rooms.get_mut(&neighbor_pos) {
                if offset.z == -1 {
                    neigh.wall_flags.front = false;
                    neigh.door_flags.front = true;
                }
                if offset.z == 1 {
                    neigh.wall_flags.back = false;
                    neigh.door_flags.back = true;
                }
                if offset.x == -1 {
                    neigh.wall_flags.right = false;
                    neigh.door_flags.right = true;
                }
                if offset.x == 1 {
                    neigh.wall_flags.left = false;
                    neigh.door_flags.left = true;
                }
            }
        } else {
            // Нет соседа — просто сплошная стена
            *wall_flag = true;
        }
    }
    map.rooms.insert(
        pos,
        RoomMetadata {
            wall_flags: wall,
            door_flags: door,
            has_light: true,
            entity: None,
        },
    );
}

pub fn spawn_registered_rooms(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    room_size: Vec3,
    wall_t: f32,
    map: &mut RoomMap,
) {
    for (pos, metadata) in map.rooms.iter_mut() {
        let origin = Vec3::new(pos.x as f32 * room_size.x, 0.0, pos.z as f32 * room_size.z);
        let ent = spawn_simple_room(
            commands,
            meshes,
            materials,
            origin,
            room_size,
            wall_t,
            metadata.wall_flags.clone(),
            metadata.door_flags.clone(),
        );
        metadata.entity = Some(ent);
    }
}
