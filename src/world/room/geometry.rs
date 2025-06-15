use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::{thread_rng, Rng};

use crate::utils::mesh_merge::merge_cubes;
use super::data::*;

/// Геометрия комнаты + платформы + готовый коллайдер
pub struct RoomGeometry {
    pub mesh: Handle<Mesh>,
    pub collider: Collider,
    pub platform_positions: Vec<Vec3>,
}

pub fn generate_room_geometry(
    meshes: &mut Assets<Mesh>,
) -> RoomGeometry {
    let mut rng = thread_rng();
    let mut cube_positions = vec![];

    // Генерация стен и потолка комнаты
    for x in 0..ROOM_WIDTH {
        for y in 0..ROOM_HEIGHT {
            for z in 0..ROOM_DEPTH {
                let is_surface = x == 0 || x == ROOM_WIDTH - 1
                    || y == 0 || y == ROOM_HEIGHT - 1
                    || z == 0 || z == ROOM_DEPTH - 1;

                if is_surface && rng.r#gen::<f32>() < SURFACE_FILL_RATIO {
                    let pos = Vec3::new(
                        x as f32 * CUBE_SIZE - ROOM_WIDTH as f32 * CUBE_SIZE / 2.0,
                        y as f32 * CUBE_SIZE,
                        z as f32 * CUBE_SIZE - ROOM_DEPTH as f32 * CUBE_SIZE / 2.0 + ROOM_Z_OFFSET,
                    );
                    cube_positions.push(pos);
                }
            }
        }
    }

    // Простейшая генерация платформ
    let mut platforms = vec![];
    let mut pos = PLATFORM_START;

    for _ in 0..PLATFORM_COUNT {
        platforms.push(pos);

        let dx = rng.gen_range(PLATFORM_DX_RANGE.0..PLATFORM_DX_RANGE.1);
        let dy = rng.gen_range(PLATFORM_DY_RANGE.0..PLATFORM_DY_RANGE.1);
        let dz = rng.gen_range(PLATFORM_DZ_RANGE.0..PLATFORM_DZ_RANGE.1);

        pos += Vec3::new(dx, dy, -dz);
    }

    let merged_mesh = merge_cubes(&cube_positions, CUBE_SIZE);
    let collider = Collider::from_bevy_mesh(
        &merged_mesh,
        &ComputedColliderShape::TriMesh(TriMeshFlags::default()),
    ).expect("couldn't create collider from merged mesh");

    let mesh = meshes.add(merged_mesh);

    RoomGeometry {
        mesh,
        collider,
        platform_positions: platforms,
    }
}
