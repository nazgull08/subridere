use bevy::{prelude::*, render::view::NoFrustumCulling};
use bevy_rapier3d::prelude::{Collider, KinematicCharacterController, KinematicCharacterControllerOutput};
use rand::seq::IteratorRandom;

use crate::{
    enemy::component::*,
    stats::health::component::Health,
    unit::component::{Grounded, Unit, Velocity},
    world::room::types::RoomMap,
};

pub fn spawn_jester_in_room(
    mut commands: Commands,
    room_map: Res<RoomMap>,
    room_query: Query<&Transform>,
    asset_server: Res<AssetServer>,
) {
    // Выбираем случайную комнату
    if let Some((_, meta)) = room_map.rooms.iter().choose(&mut rand::thread_rng()) {
        if let Some(room_entity) = meta.entity {
            if let Ok(room_transform) = room_query.get(room_entity) {
                let mut pos = room_transform.translation;
                pos.y += 1.0;

                let scene = asset_server.load("models/jester.glb#Scene0");

                let jester_entity = commands.spawn((
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
                        offset: bevy_rapier3d::prelude::CharacterLength::Absolute(0.01),
                        ..default()
                    },
                    KinematicCharacterControllerOutput::default(),
                )).id();

                // Добавляем визуал и анимацию как детей
                commands.entity(jester_entity).with_children(|child_commands| {
                    child_commands.spawn((
                        SceneRoot(scene.clone()),
                        Transform::from_xyz(0.0, -1.0, 0.0).with_scale(Vec3::splat(1.4)),
                        GlobalTransform::default(),
                        NoFrustumCulling,
                        Name::new("JesterScene"),
                    ));

                    child_commands.spawn((
                        AnimationPlayer::default(),
                        Name::new("JesterAnimPlayer"),
                    ));
                });

                info!("Spawned Jester in room at {:?}", pos);
            }
        }
    }
}
