// src/game_init/maze_rooms.rs

use crate::world::builders::room::spawn_simple_room;
use crate::world::generators::maze::generate_maze;
use crate::world::room::types::{DoorFlags, RoomMap, RoomMetadata, WallFlags};
use bevy::prelude::*;
use rand::{Rng, thread_rng};

use super::state::InitStage;

/// Спавним лабиринт из комнат в сетке width×height
pub fn spawn_maze_rooms(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_state: ResMut<NextState<InitStage>>,
    mut room_map: ResMut<RoomMap>,
) {
    let mut rng = thread_rng();
    let width = 5;
    let height = 5;
    let room_size = Vec3::new(12.0, 6.0, 12.0);
    let wall_t = 0.1;

    // 1) Получаем граф лабиринта
    let graph = generate_maze(width, height);

    // 2) Регистрируем все клетки
    for x in 0..width {
        for z in 0..height {
            let pos = IVec3::new(x, 0, z);
            // по умолчанию все стены закрыты
            let has_light = rng.gen_bool(0.3);
            let meta = RoomMetadata {
                wall_flags: WallFlags::default(),
                door_flags: DoorFlags::default(),
                has_light,
                entity: None,
            };
            room_map.rooms.insert(pos, meta);
        }
    }

    // 3) Устанавливаем флаги на основании графа
    for x in 0..width {
        for z in 0..height {
            let cell = IVec2::new(x, z);
            let pos3 = IVec3::new(x, 0, z);
            let meta = room_map.rooms.get_mut(&pos3).unwrap();

            // соединения из графа
            let neighbors = graph.get(&cell).cloned().unwrap_or_default();

            // для каждого направления выставляем wall/door
            // back (z-1)
            if neighbors.contains(&(cell + IVec2::new(0, -1))) {
                meta.door_flags.back = true;
                meta.wall_flags.back = false;
            } else {
                meta.wall_flags.back = true;
            }
            // front (z+1)
            if neighbors.contains(&(cell + IVec2::new(0, 1))) {
                meta.door_flags.front = true;
                meta.wall_flags.front = false;
            } else {
                meta.wall_flags.front = true;
            }
            // left (x-1)
            if neighbors.contains(&(cell + IVec2::new(-1, 0))) {
                meta.door_flags.left = true;
                meta.wall_flags.left = false;
            } else {
                meta.wall_flags.left = true;
            }
            // right (x+1)
            if neighbors.contains(&(cell + IVec2::new(1, 0))) {
                meta.door_flags.right = true;
                meta.wall_flags.right = false;
            } else {
                meta.wall_flags.right = true;
            }
        }
    }

    // 4) Спавним все комнаты
    for (&pos3, meta) in room_map.rooms.iter_mut() {
        let origin = Vec3::new(
            pos3.x as f32 * room_size.x,
            0.0,
            pos3.z as f32 * room_size.z,
        );
        let ent = spawn_simple_room(
            &mut commands,
            &mut meshes,
            &mut materials,
            origin,
            room_size,
            wall_t,
            meta.wall_flags.clone(),
            meta.door_flags.clone(),
        );
        meta.entity = Some(ent);
    }

    next_state.set(InitStage::MazeReady);
}

pub fn spawn_room_lights(
    mut commands: Commands,
    room_map: Res<RoomMap>,
    mut next_state: ResMut<NextState<InitStage>>,
) {
    for (_, room) in room_map.rooms.iter() {
        if !room.has_light || room.entity.is_none() {
            continue;
        }

        let room_entity = room.entity.unwrap();
        let light_pos = Vec3::new(0.0, 5.0, 0.0); // чуть выше пола, по центру

        commands.entity(room_entity).with_children(|child| {
            child.spawn((
                PointLight {
                    color: Color::srgb(0.0, 0.7, 0.0), // тёплый свет
                    intensity: 1_000_000.0,
                    range: 200.0,
                    shadows_enabled: true,
                    ..default()
                },
                Transform::from_translation(light_pos),
                Name::new("RoomLight"),
            ));
        });
        next_state.set(InitStage::LightsReady);
    }
}
