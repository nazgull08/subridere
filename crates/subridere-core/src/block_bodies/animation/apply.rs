// src/block_bodies/animation/apply.rs

use crate::block_bodies::pose::{BlockPose, PoseToApply};
use bevy::prelude::*;

/// Рекурсивно применяет позу к телу, не затирая позицию
pub fn apply_pose_to_body_recursive(
    entity: Entity,
    pose: &BlockPose,
    transforms: &mut Query<(&Name, &mut Transform)>,
    children_query: &Query<&Children>,
) {
    if let Ok((name, mut transform)) = transforms.get_mut(entity) {
        if let Some(part_pose) = pose.get_part(name.as_str()) {
            // Применяем только поворот, оставляя трансляцию как есть
            transform.rotation = part_pose.rotation;
        }
    }

    // Обрабатываем дочерние сущности
    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            apply_pose_to_body_recursive(child, pose, transforms, children_query);
        }
    }
}

/// Система одноразового применения позы (удаляет компонент `PoseToApply`)
pub fn apply_pose_once_system(
    mut commands: Commands,
    query: Query<(Entity, &PoseToApply)>,
    mut transforms: Query<(&Name, &mut Transform)>,
    children_query: Query<&Children>,
) {
    for (entity, pose) in &query {
        apply_pose_to_body_recursive(entity, &pose.0, &mut transforms, &children_query);
        commands.entity(entity).remove::<PoseToApply>();
    }
}
