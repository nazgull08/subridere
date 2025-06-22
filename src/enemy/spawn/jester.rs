use std::collections::HashMap;
use bevy::prelude::*;
use rand::seq::IteratorRandom;

use crate::{
    block_bodies::{model::spawn_model_hierarchical, pose::{BlockPose, PoseToApply}, utils::load_model_from_ron},
    world::room::types::RoomMap,
};

use super::base::spawn_enemy_base;
use crate::enemy::kind::EnemyKind;

/// Спавн джестера в случайной комнате
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
