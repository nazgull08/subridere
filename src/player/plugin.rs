use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::camera::flycam::FlyCamera;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
           .add_systems(Update, kill_plane_system);
    }
}

fn spawn_player(
    mut commands: Commands,
) {
    // Размеры «гуманоидного» коллайдера
    let half_extents = Vec3::new(0.3, 0.9, 0.3);

    let player_id = commands
        .spawn((
            Player,
            // физика
            RigidBody::Dynamic,
            Collider::cuboid(half_extents.x, half_extents.y, half_extents.z),
            // чтобы не заваливался:
            LockedAxes::ROTATION_LOCKED_X
                | LockedAxes::ROTATION_LOCKED_Z
                | LockedAxes::ROTATION_LOCKED_Y,
            Transform::from_xyz(0.0, 1.5, 10.0),
            Name::new("Player"),
        ))
        .id();

    // Камера — дочерний узел, смещённый на уровень «головы»
    commands.entity(player_id).with_children(|parent| {
        parent.spawn((
            Camera3d::default(),
            FlyCamera::default(),
            Transform::from_xyz(0.0, 1.6, 0.0),
            Name::new("PlayerCamera"),
        ));
    });
}


fn kill_plane_system(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Player>>,
) {
    for (entity, transform) in &query {
        
        if transform.translation.y < -50.0 {
            println!("💀 Игрок погиб. {:?} Respawning...", transform); 
            commands.entity(entity).insert(Transform::from_xyz(0.0, 2.0, 10.0));
        }
    }
}
