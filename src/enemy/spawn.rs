use std::collections::HashMap;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::seq::IteratorRandom;

use crate::{
    block_bodies::{
        model::spawn_model_hierarchical,
        pose::{BlockPose, PoseToApply},
        utils::load_model_from_ron,
    },
    enemy::{component::*, kind::EnemyKind},
    stats::health::component::Health,
    unit::component::{Grounded, Unit, Velocity},
    world::room::types::RoomMap,
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

    let entity = spawn_enemy_base(&mut commands, pos, EnemyKind::Jester);
    spawn_jester_visuals(&mut commands, &mut meshes, &mut materials, entity);

    info!("Spawned Jester at {:?}", pos);
}

/// Спавним Jimbo в случайной комнате
pub fn spawn_jimbo_in_room(
    mut commands: Commands,
    room_map: Res<RoomMap>,
    room_query: Query<&Transform>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Some((_, meta)) = room_map.rooms.iter().choose(&mut rand::thread_rng()) else { return };
    let Some(room_entity) = meta.entity else { return };
    let Ok(room_transform) = room_query.get(room_entity) else { return };

    let pos = room_transform.translation + Vec3::Y + Vec3::X * 3.0;

    let entity = spawn_enemy_base(&mut commands, pos, EnemyKind::Jimbo);
    spawn_jimbo_visuals(&mut commands, &mut meshes, &mut materials, entity);

    info!("Spawned Jimbo at {:?}", pos);
}

/// Общий спавн логики врага
fn spawn_enemy_base(commands: &mut Commands, pos: Vec3, kind: EnemyKind) -> Entity {
    commands
        .spawn((
            Enemy,
            kind,
            EnemyState::Idle,
            Unit,
            Grounded(true),
            Velocity::default(),
            Health::new(100.0, 0.0),
            MeleeAttack { damage: 15.0 },
            Transform::from_translation(pos),
            GlobalTransform::default(),
            Visibility::Visible,
            Name::new(format!("{kind:?}")),
            Collider::capsule_y(0.9, 0.3),
            KinematicCharacterController {
                offset: CharacterLength::Absolute(0.01),
                ..default()
            },
            KinematicCharacterControllerOutput::default(),
        ))
        .id()
}

/// Визуал джестера
fn spawn_jester_visuals(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    parent: Entity,
) {
    let red  = materials.add(Color::srgb(0.8, 0.2, 0.2));
    let gray = materials.add(Color::srgb(0.5, 0.5, 0.5));

    let mut material_map = HashMap::new();
    material_map.insert("red".into(), red);
    material_map.insert("gray".into(), gray);

    let model = load_model_from_ron("assets/models/humanoid.ron", &material_map)
        .expect("failed to load model");

    spawn_model_hierarchical(&model, commands, meshes, parent);

    let pose = BlockPose::from_ron_file("assets/poses/tpose.ron")
        .expect("failed to load pose");

    commands.entity(parent).insert(PoseToApply(pose));
}

/// Визуал джимбо
fn spawn_jimbo_visuals(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    parent: Entity,
) {
    let red = materials.add(Color::srgb(0.8, 0.2, 0.2));
    let gray = materials.add(Color::srgb(0.5, 0.5, 0.5));
    let green = materials.add(Color::srgb(0.2, 0.8, 0.2));

    let mut material_map = HashMap::new();
    material_map.insert("red".into(), red);
    material_map.insert("gray".into(), gray);
    material_map.insert("green".into(), green);

    let model = load_model_from_ron("assets/models/jimbo.ron", &material_map)
        .expect("failed to load jimbo model");

    spawn_model_hierarchical(&model, commands, meshes, parent);

    let pose = BlockPose::from_ron_file("assets/poses/jimbo_tpose.ron")
        .expect("failed to load jimbo pose");

    commands.entity(parent).insert(PoseToApply(pose));
}

/// Дебаг-спавн Jimbo вручную
pub fn spawn_jimbo_debug(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pos: Vec3,
    model_path: &str,
    pose_path: &str,
) -> Entity {
    let red = materials.add(Color::srgb(0.8, 0.2, 0.2));
    let gray = materials.add(Color::srgb(0.5, 0.5, 0.5));
    let green = materials.add(Color::srgb(0.2, 0.8, 0.2));

    let mut material_map = HashMap::new();
    material_map.insert("red".into(), red);
    material_map.insert("gray".into(), gray);
    material_map.insert("green".into(), green);

    let entity = commands
        .spawn((
            Name::new("JimboDebug"),
            Transform::from_translation(pos),
            GlobalTransform::default(),
            Visibility::Visible,
        ))
        .id();

    let model = load_model_from_ron(model_path, &material_map)
        .expect("failed to load debug jimbo model");

    spawn_model_hierarchical(&model, commands, meshes, entity);

    let pose = BlockPose::from_ron_file(pose_path)
        .expect("failed to load debug pose");

    commands.entity(entity).insert(PoseToApply(pose));

    entity
}
