use bevy::prelude::*;
use rand::seq::IteratorRandom;
use std::collections::HashMap;

use crate::{
    block_bodies::{
        animation::component::AnimationCycle, model::spawn_model_hierarchical, pose::BlockPose,
        utils::load_model_from_ron,
    },
    world::room::types::RoomMap,
};

use super::base::spawn_enemy_base;
use crate::enemy::component::EnemyKind;

pub fn spawn_jimbo_in_room(
    mut commands: Commands,
    room_map: Res<RoomMap>,
    room_query: Query<&Transform>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Some((_, meta)) = room_map.rooms.iter().choose(&mut rand::thread_rng()) else {
        return;
    };
    let Some(room_entity) = meta.entity else {
        return;
    };
    let Ok(room_transform) = room_query.get(room_entity) else {
        return;
    };

    let pos = room_transform.translation + Vec3::Y * 0.1 + Vec3::X * 3.0;

    let entity = spawn_enemy_base(&mut commands, pos, EnemyKind::Jimbo);
    spawn_jimbo_visuals(&mut commands, &mut meshes, &mut materials, entity);

    info!("Spawned Jimbo at {:?}", pos);
}

pub fn spawn_jimbo_visuals(
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

    let poses = [
        "poses/jimbo/idle/neutral.ron",
        "poses/jimbo/idle/breath_in.ron",
        "poses/jimbo/idle/breath_out.ron",
    ]
    .into_iter()
    .map(|path| BlockPose::from_ron_file(format!("assets/{}", path)).expect("load pose"))
    .collect::<Vec<_>>()
    .into();

    let pose_duration = 1.0;
    commands.entity(parent).insert(AnimationCycle {
        poses,
        current_index: 0,
        pose_duration,
    });
}
