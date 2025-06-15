use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::utils::mesh_merge::merge_cubes;

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
    let cube_size = 0.2;
    let room_width = 40;
    let room_height = 20;
    let room_depth = 40;

    let mut rng = thread_rng();
    let mut cube_positions = vec![];

    for x in 0..room_width {
        for y in 0..room_height {
            for z in 0..room_depth {
                let is_surface = x == 0 || x == room_width - 1
                    || y == 0 || y == room_height - 1
                    || z == 0 || z == room_depth - 1;

                if is_surface && rng.r#gen::<f32>() < 0.85 {
                    cube_positions.push(Vec3::new(
                        x as f32 * cube_size - room_width as f32 * cube_size / 2.0,
                        y as f32 * cube_size,
                        z as f32 * cube_size - room_depth as f32 * cube_size / 2.0 - 20.0,
                    ));
                }
            }
        }
    }

    // Генерация одного меша для стен
    let merged_mesh = meshes.add(merge_cubes(&cube_positions, cube_size));
    let cube_material = materials.add(Color::srgb(0.9, 0.85, 0.8));

    commands.spawn((
        Mesh3d(merged_mesh),
        MeshMaterial3d(cube_material),
        Transform::IDENTITY,
    ));

    // Простейшая мебель
    let furniture_mesh = meshes.add(Mesh::from(Cuboid::new(0.4, 0.8, 0.4)));
    let furniture_material = materials.add(Color::srgb(0.4, 0.3, 0.2));

    let furniture_positions = [
        Vec3::new(0.0, 0.4, 0.0),
        Vec3::new(1.0, 0.4, 1.0),
        Vec3::new(-1.0, 0.4, -1.0),
    ];

    for pos in furniture_positions {
        commands.spawn((
            Mesh3d(furniture_mesh.clone()),
            MeshMaterial3d(furniture_material.clone()),
            Transform::from_translation(pos),
        ));
    }
}
