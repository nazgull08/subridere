// src/world/builders/room.rs

use crate::world::builders::{
    panel::spawn_panel,
    wall::{spawn_solid_wall, spawn_wall_with_door},
};
use crate::world::room::types::{DoorFlags, WallFlags};
use bevy::prelude::*;

pub fn spawn_simple_room(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    origin: Vec3,
    size: Vec3,
    wall_t: f32,
    walls: WallFlags,
    doors: DoorFlags,
) -> Entity {
    let material = materials.add(Color::srgb(0.75, 0.75, 0.8));
    let room = commands
        .spawn((
            Transform::from_translation(origin),
            InheritedVisibility::VISIBLE,
            Name::new("Room"),
        ))
        .id();

    commands.entity(room).with_children(|child| {
        // Пол
        spawn_panel(
            child,
            meshes,
            material.clone(),
            Vec3::new(size.x, wall_t, size.z),
            Vec3::ZERO,
            "Floor".into(),
        );

        // Стены
        if walls.back {
            let pos = Vec3::new(0.0, size.y / 2.0, -size.z / 2.0);
            if doors.back {
                spawn_wall_with_door(
                    child,
                    meshes,
                    material.clone(),
                    Vec3::new(size.x, size.y, wall_t),
                    3.0,
                    4.0,
                    pos,
                    "BackWall",
                );
            } else {
                spawn_solid_wall(
                    child,
                    meshes,
                    material.clone(),
                    Vec3::new(size.x, size.y, wall_t),
                    pos,
                    "BackWall",
                );
            }
        }
        if walls.front {
            let pos = Vec3::new(0.0, size.y / 2.0, size.z / 2.0);
            if doors.front {
                spawn_wall_with_door(
                    child,
                    meshes,
                    material.clone(),
                    Vec3::new(size.x, size.y, wall_t),
                    3.0,
                    4.0,
                    pos,
                    "FrontWall",
                );
            } else {
                spawn_solid_wall(
                    child,
                    meshes,
                    material.clone(),
                    Vec3::new(size.x, size.y, wall_t),
                    pos,
                    "FrontWall",
                );
            }
        }
        if walls.left {
            let pos = Vec3::new(-size.x / 2.0, size.y / 2.0, 0.0);
            if doors.left {
                spawn_wall_with_door(
                    child,
                    meshes,
                    material.clone(),
                    Vec3::new(wall_t, size.y, size.z),
                    3.0,
                    4.0,
                    pos,
                    "LeftWall",
                );
            } else {
                spawn_solid_wall(
                    child,
                    meshes,
                    material.clone(),
                    Vec3::new(wall_t, size.y, size.z),
                    pos,
                    "LeftWall",
                );
            }
        }
        if walls.right {
            let pos = Vec3::new(size.x / 2.0, size.y / 2.0, 0.0);
            if doors.right {
                spawn_wall_with_door(
                    child,
                    meshes,
                    material.clone(),
                    Vec3::new(wall_t, size.y, size.z),
                    3.0,
                    4.0,
                    pos,
                    "RightWall",
                );
            } else {
                spawn_solid_wall(
                    child,
                    meshes,
                    material.clone(),
                    Vec3::new(wall_t, size.y, size.z),
                    pos,
                    "RightWall",
                );
            }
        }
    });

    room
}
