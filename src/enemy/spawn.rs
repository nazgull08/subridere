use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::seq::IteratorRandom;

use crate::{
    block_bodies::{enemies::jester::make_jester_body, pose::{BlockPose, PoseToApply}}, enemy::component::*, stats::health::component::Health, unit::component::{Grounded, Unit, Velocity}, utils::block_body::spawn_blocky_body, world::room::types::RoomMap
};

/// Спавним джестера в случайной комнате
pub fn spawn_jester_in_room(
    mut commands: Commands,
    room_map: Res<RoomMap>,
    room_query: Query<&Transform>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Some((_, meta)) = room_map.rooms.iter().choose(&mut rand::thread_rng()) else { return };
    let Some(room_entity) = meta.entity else { return };
    let Ok(room_transform) = room_query.get(room_entity) else { return };

    let pos = room_transform.translation + Vec3::Y;

    let jester_entity = spawn_enemy_base(&mut commands, pos);
    spawn_jester_visuals(&mut commands, &mut meshes, &mut materials, jester_entity);

    info!("Spawned Jester (blocky) at {:?}", pos);
}

/// Базовая логическая сущность врага
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

fn spawn_jester_visuals(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    parent: Entity,
) {
    let red  = materials.add(Color::srgb(0.8, 0.2, 0.2));
    let gray = materials.add(Color::srgb(0.5, 0.5, 0.5));

    // 1. Спавним блочное тело как детей `parent`
    let body = make_jester_body(red, gray);
    body.spawn(commands, meshes, parent);

    // 2. Загружаем нужную позу
    //    (путь относительно корня проекта; RON-файлы лежат в assets/poses/)
    let pose = BlockPose::from_ron_file("assets/poses/idle.ron")
        .expect("failed to load pose");

    // 3. Вешаем PoseToApply на родителя, у которого уже есть Children
    commands.entity(parent).insert(PoseToApply(pose));
}
