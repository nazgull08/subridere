use crate::world::builders::panel::spawn_panel;
use bevy::prelude::*;

/// Сплошная стена-панель
pub fn spawn_solid_wall(
    spawner: &mut ChildSpawnerCommands,
    meshes: &mut Assets<Mesh>,
    material: Handle<StandardMaterial>,
    size: Vec3,   // (width, height, thickness)
    offset: Vec3, // центр панели
    name: &str,
) {
    spawn_panel(spawner, meshes, material, size, offset, name.into());
}

/// Стена с дверным проёмом, расположенным по центру
pub fn spawn_wall_with_door(
    spawner: &mut ChildSpawnerCommands,
    meshes: &mut Assets<Mesh>,
    material: Handle<StandardMaterial>,
    wall: Vec3, // размеры полной стены (width, height, thickness)
    door_w: f32,
    door_h: f32,
    offset: Vec3, // центр всей стены
    name: &str,
) {
    let t = wall.z;
    let side_w = (wall.x - door_w) * 0.5;
    let side_off = door_w * 0.5 + side_w * 0.5;

    // X-координаты центров боковых половинок
    let left = offset + Vec3::new(-side_off, 0.0, 0.0);
    let right = offset + Vec3::new(side_off, 0.0, 0.0);

    // Центр верхней перемычки
    let top_h = wall.y - door_h;
    let top_center = offset + Vec3::new(0.0, door_h + top_h * 0.5 - wall.y * 0.5, 0.0);

    spawn_panel(
        spawner,
        meshes,
        material.clone(),
        Vec3::new(side_w, wall.y, t),
        left,
        format!("{name}_Left"),
    );
    spawn_panel(
        spawner,
        meshes,
        material.clone(),
        Vec3::new(side_w, wall.y, t),
        right,
        format!("{name}_Right"),
    );

    if top_h > 0.0 {
        spawn_panel(
            spawner,
            meshes,
            material,
            Vec3::new(door_w, top_h, t),
            top_center,
            format!("{name}_Top"),
        );
    }
}
