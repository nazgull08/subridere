// src/block_bodies/animation/system.rs

use crate::block_bodies::animation::component::AnimationCycle;
use crate::block_bodies::animation::lerp::PoseLerp;
use crate::block_bodies::pose::{BlockPose, BlockPosePart};
use bevy::prelude::*;

fn extract_current_pose(
    entity: Entity,
    transforms: &Query<(&Name, &Transform)>,
    children_query: &Query<&Children>,
) -> BlockPose {
    fn recurse(
        entity: Entity,
        transforms: &Query<(&Name, &Transform)>,
        children_query: &Query<&Children>,
        parts: &mut Vec<BlockPosePart>,
    ) {
        if let Ok((name, transform)) = transforms.get(entity) {
            parts.push(BlockPosePart {
                name: name.as_str().to_owned(),
                rotation: transform.rotation,
            });
        }
        if let Ok(children) = children_query.get(entity) {
            for &child in children {
                recurse(child, transforms, children_query, parts);
            }
        }
    }

    let mut parts = Vec::new();
    recurse(entity, transforms, children_query, &mut parts);
    BlockPose { parts }
}

pub fn animation_cycle_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut AnimationCycle, Option<&PoseLerp>)>,
    transforms: Query<(&Name, &Transform)>,
    children_query: Query<&Children>,
) {
    for (entity, mut cycle, maybe_lerp) in &mut query {
        // Если сейчас идёт анимация — ничего не делаем
        if maybe_lerp.is_some() {
            continue;
        }

        // Текущая поза = то, что было применено
        let current_pose = extract_current_pose(entity, &transforms, &children_query);

        // Цель — следующая в цикле
        cycle.current_index = (cycle.current_index + 1) % cycle.poses.len();
        let next_pose = cycle.poses[cycle.current_index].clone();

        // Запускаем плавный переход
        commands.entity(entity).insert(PoseLerp {
            from: current_pose,
            to: next_pose,
            timer: Timer::from_seconds(cycle.pose_duration, TimerMode::Once),
            looping: false,
        });
    }
}
