use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::seq::IteratorRandom;

use crate::{
    enemy::component::*,
    stats::health::component::Health,
    unit::component::{Grounded, Unit, Velocity},
    utils::block_body::{BlockPart, spawn_blocky_body},
    world::room::types::RoomMap,
};

pub fn spawn_jester_in_room(
    mut commands: Commands,
    room_map: Res<RoomMap>,
    room_query: Query<&Transform>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Some((_, meta)) = room_map.rooms.iter().choose(&mut rand::thread_rng()) {
        if let Some(room_entity) = meta.entity {
            if let Ok(room_transform) = room_query.get(room_entity) {
                let pos = room_transform.translation + Vec3::Y;

                let jester_entity = spawn_enemy_base(&mut commands, pos);

                // Материалы
                let red = materials.add(Color::srgb(0.8, 0.2, 0.2));
                let gray = materials.add(Color::srgb(0.5, 0.5, 0.5));

                // Добавляем "блочное" тело
                let parts = vec![
                    BlockPart::new(
                        "Torso",
                        Vec3::new(0.0, 0.9, 0.0),
                        Vec3::new(0.5, 0.6, 0.3),
                        gray.clone(),
                    ),
                    BlockPart::new(
                        "Head",
                        Vec3::new(0.0, 1.5, 0.0),
                        Vec3::new(0.4, 0.4, 0.4),
                        red.clone(),
                    ),
                    BlockPart::new(
                        "ArmL",
                        Vec3::new(-0.5, 0.9, 0.0),
                        Vec3::new(0.2, 0.5, 0.2),
                        gray.clone(),
                    ),
                    BlockPart::new(
                        "ArmR",
                        Vec3::new(0.5, 0.9, 0.0),
                        Vec3::new(0.2, 0.5, 0.2),
                        gray.clone(),
                    ),
                    BlockPart::new(
                        "LegL",
                        Vec3::new(-0.2, 0.3, 0.0),
                        Vec3::new(0.2, 0.6, 0.2),
                        gray.clone(),
                    ),
                    BlockPart::new(
                        "LegR",
                        Vec3::new(0.2, 0.3, 0.0),
                        Vec3::new(0.2, 0.6, 0.2),
                        gray.clone(),
                    ),
                ];

                spawn_blocky_body(&mut commands, &mut meshes, jester_entity, parts);
                info!("Spawned Jester (blocky) at {:?}", pos);
            }
        }
    }
}

fn spawn_enemy_base(commands: &mut Commands, pos: Vec3) -> Entity {
    commands
        .spawn((
            Enemy,
            EnemyKind::Jester,
            EnemyState::Idle,
            Unit,
            Grounded(true),
            Velocity::default(),
            Health::new(100.0, 0.0),
            MeleeAttack { damage: 20.0 },
            Transform::from_translation(pos),
            GlobalTransform::default(),
            Visibility::Visible,
            Name::new("Jester"),
            Collider::capsule_y(0.9, 0.3),
            KinematicCharacterController {
                offset: CharacterLength::Absolute(0.01),
                ..default()
            },
            KinematicCharacterControllerOutput::default(),
        ))
        .id()
}
